import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/grpc.dart' as grpc;
import 'package:socialgym_mobile/providers/consent_provider.dart';
import 'package:socialgym_mobile/services/grpc/grpc_business_profile_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_consent_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_legal_document_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_media_service.dart';
import 'package:socialgym_mobile/services/upload_service.dart';
import 'package:socialgym_mobile/src/generated/grpc/business_profile.pbgrpc.dart' as $bp;
import 'package:socialgym_mobile/src/generated/grpc/consent.pbgrpc.dart' as $consent;
import 'package:socialgym_mobile/src/generated/grpc/legal.pbgrpc.dart' as $legal;
import 'package:socialgym_mobile/src/generated/grpc/media.pbgrpc.dart' as $media;

import 'utils/fake_grpc_server.dart';

class _Legal extends $legal.LegalDocumentServiceBase {
  @override
  Future<$legal.ListLegalDocumentsResponse> listLegalDocuments(grpc.ServiceCall call, $legal.ListLegalDocumentsRequest request) async =>
      $legal.ListLegalDocumentsResponse();

  @override
  Future<$legal.LegalDocument> getLegalDocument(grpc.ServiceCall call, $legal.GetLegalDocumentRequest request) async =>
      $legal.LegalDocument(document: request.document, version: '1.0.0', title: 't', content: 'c');
}

/// Terms and privacy are outstanding until each is accepted.
class _Consent extends $consent.ConsentServiceBase {
  final Set<String> outstanding = {'terms', 'privacy'};
  bool failPending = false;

  @override
  Future<$consent.ListPendingConsentsResponse> listPendingConsents(grpc.ServiceCall call, $consent.ListPendingConsentsRequest request) async {
    if (failPending) throw grpc.GrpcError.unavailable('down');
    return $consent.ListPendingConsentsResponse(pending: outstanding.map((d) => $consent.PendingConsent(document: d, currentVersion: '1.0.0')));
  }

  @override
  Future<$consent.Consent> acceptConsent(grpc.ServiceCall call, $consent.AcceptConsentRequest request) async {
    outstanding.remove(request.document);
    return $consent.Consent(document: request.document, version: request.version);
  }

  @override
  Future<$consent.ListConsentsResponse> listConsents(grpc.ServiceCall call, $consent.ListConsentsRequest request) async => $consent.ListConsentsResponse();

  @override
  Future<$consent.RevokeConsentResponse> revokeConsent(grpc.ServiceCall call, $consent.RevokeConsentRequest request) async => $consent.RevokeConsentResponse();
}

class _Media extends $media.MediaServiceBase {
  @override
  Future<$media.MediaUploadResponse> getPostMediaUploadUrl(grpc.ServiceCall call, $media.MediaUploadRequest request) async =>
      $media.MediaUploadResponse(url: 'https://s3/${request.album}', objectKey: 'k/${request.album}');
}

class _Profiles extends $bp.BusinessProfileServiceBase {
  @override
  Future<$bp.BusinessProfileImageUploadResponse> getBusinessProfileImageUploadUrl(grpc.ServiceCall call, $bp.BusinessProfileImageUploadRequest request) async =>
      $bp.BusinessProfileImageUploadResponse(url: 'https://s3/${request.imageType}', objectKey: 'bp/${request.imageType}');

  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

void main() {
  late FakeGrpcServer server;
  final consent = _Consent();

  setUpAll(() async {
    server = await FakeGrpcServer.start([_Legal(), consent, _Media(), _Profiles()]);
    GrpcLegalDocumentService.useClient($legal.LegalDocumentServiceClient(server.channel));
    GrpcConsentService.useClient($consent.ConsentServiceClient(server.channel));
    GrpcMediaService.useClient($media.MediaServiceClient(server.channel));
    GrpcBusinessProfileService.useClient($bp.BusinessProfileServiceClient(server.channel));
  });
  tearDownAll(() => server.stop());

  group('ConsentProvider', () {
    test('the gate rises with the outstanding documents and drops once every one is accepted', () async {
      final provider = ConsentProvider();
      await provider.trigger();
      expect(provider.blocking, isTrue);
      expect(provider.outstanding.map((p) => p.document), unorderedEquals(['terms', 'privacy']));

      await provider.accept('terms');
      expect(provider.blocking, isTrue, reason: 'privacy is still outstanding');
      await provider.accept('privacy');
      expect(provider.blocking, isFalse);
      expect(provider.error, isNull);
    });

    test('a failed check keeps the gate up and says why', () async {
      final provider = ConsentProvider();
      consent.failPending = true;
      await provider.trigger();
      expect(provider.blocking, isTrue);
      expect(provider.error, isNotNull);
      consent.failPending = false;
    });
  });

  group('UploadService pre-signed URLs', () {
    test('post and chat media come from the MediaService', () async {
      final url = await UploadService.getPostMediaPresignedUrl('image/png', 'chat');
      expect((url.url, url.objectKey), ('https://s3/chat', 'k/chat'));
    });

    test('the business-profile logo and cover come from the BusinessProfileService', () async {
      expect((await UploadService.getBusinessProfileLogoPresignedUrl('image/png')).objectKey, 'bp/logo');
      expect((await UploadService.getBusinessProfileCoverPresignedUrl('image/png')).objectKey, 'bp/cover');
    });
  });
}
