import 'package:flutter_test/flutter_test.dart';
import 'package:grpc/grpc.dart' as grpc;
import 'package:socialgym_mobile/services/base_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_address_search_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_consent_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_legal_document_service.dart';
import 'package:socialgym_mobile/src/generated/grpc/address.pbgrpc.dart' as $address;
import 'package:socialgym_mobile/src/generated/grpc/consent.pbgrpc.dart' as $consent;
import 'package:socialgym_mobile/src/generated/grpc/legal.pbgrpc.dart' as $legal;

import 'utils/fake_grpc_server.dart';

class _Legal extends $legal.LegalDocumentServiceBase {
  @override
  Future<$legal.ListLegalDocumentsResponse> listLegalDocuments(grpc.ServiceCall call, $legal.ListLegalDocumentsRequest request) async =>
      $legal.ListLegalDocumentsResponse(documents: [
        $legal.LegalDocument(document: 'terms', version: '1.0.0', title: 'Terms', content: 'T'),
        $legal.LegalDocument(document: 'privacy', version: '2.0.0', title: 'Privacy', content: 'P'),
      ]);

  @override
  Future<$legal.LegalDocument> getLegalDocument(grpc.ServiceCall call, $legal.GetLegalDocumentRequest request) async {
    if (request.document == 'terms') {
      return $legal.LegalDocument(document: 'terms', version: '1.0.0', title: 'Terms', content: 'T');
    }
    if (request.document == 'health_data') {
      return $legal.LegalDocument(document: 'health_data', version: '3.0.0', title: 'Health', content: 'H');
    }
    throw grpc.GrpcError.notFound('no such document');
  }
}

class _Consent extends $consent.ConsentServiceBase {
  bool alreadyActive = false;
  $consent.AcceptConsentRequest? accepted;

  @override
  Future<$consent.ListConsentsResponse> listConsents(grpc.ServiceCall call, $consent.ListConsentsRequest request) async =>
      $consent.ListConsentsResponse(consents: [
        $consent.Consent(document: 'health_data', version: '3.0.0'),
        $consent.Consent(document: 'terms', version: '0.9.0'),
        $consent.Consent(document: 'privacy', version: '2.0.0', revokedAt: '2026-01-01T00:00:00Z'),
      ]);

  @override
  Future<$consent.ListPendingConsentsResponse> listPendingConsents(grpc.ServiceCall call, $consent.ListPendingConsentsRequest request) async =>
      $consent.ListPendingConsentsResponse(pending: [
        $consent.PendingConsent(document: 'terms', currentVersion: '1.0.0', acceptedVersion: '0.9.0'),
        $consent.PendingConsent(document: 'privacy', currentVersion: '2.0.0'),
      ]);

  @override
  Future<$consent.Consent> acceptConsent(grpc.ServiceCall call, $consent.AcceptConsentRequest request) async {
    accepted = request;
    if (alreadyActive) throw grpc.GrpcError.alreadyExists('already active');
    return $consent.Consent(document: request.document, version: request.version);
  }

  @override
  Future<$consent.RevokeConsentResponse> revokeConsent(grpc.ServiceCall call, $consent.RevokeConsentRequest request) async =>
      $consent.RevokeConsentResponse();
}

class _Address extends $address.AddressSearchServiceBase {
  $address.SearchAddressRequest? last;

  @override
  Future<$address.SearchAddressResponse> searchAddress(grpc.ServiceCall call, $address.SearchAddressRequest request) async {
    last = request;
    if (request.text == 'down') throw grpc.GrpcError.unavailable('places down');
    return $address.SearchAddressResponse(candidates: [
      $address.AddressCandidate(
        placeId: 'p1',
        formattedAddress: 'Rua A 1, Sao Paulo',
        addressLine1: 'Rua A 1',
        locality: 'Sao Paulo',
        administrativeArea: 'Sao Paulo',
        administrativeAreaCode: 'SP',
        postalCode: '01000-000',
        countryCode: 'BR',
        latitude: -23.5,
        longitude: -46.6,
      ),
    ]);
  }
}

void main() {
  late FakeGrpcServer server;
  final consent = _Consent();
  final address = _Address();

  setUpAll(() async {
    server = await FakeGrpcServer.start([_Legal(), consent, address]);
    GrpcLegalDocumentService.useClient($legal.LegalDocumentServiceClient(server.channel));
    GrpcConsentService.useClient($consent.ConsentServiceClient(server.channel));
    GrpcAddressSearchService.useClient($address.AddressSearchServiceClient(server.channel));
  });
  tearDownAll(() => server.stop());

  group('GrpcLegalDocumentService', () {
    test('lists and gets documents', () async {
      final all = await GrpcLegalDocumentService.list();
      expect(all.map((d) => '${d.document}@${d.version}'), ['terms@1.0.0', 'privacy@2.0.0']);
      expect((await GrpcLegalDocumentService.get('terms')).content, 'T');
    });

    test('an unknown document becomes a 404 AppException', () async {
      await expectLater(
        GrpcLegalDocumentService.get('nope'),
        throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 404)),
      );
    });
  });

  group('GrpcConsentService', () {
    test('hasActive needs the current version and no revocation', () async {
      expect(await GrpcConsentService.hasActive('health_data'), isTrue);
      expect(await GrpcConsentService.hasActive('terms'), isFalse, reason: 'accepted at an older version');
    });

    test('pending maps the first-time and the outdated documents', () async {
      final pending = await GrpcConsentService.pending();
      expect(pending.map((p) => (p.document, p.isFirstTime)), [('terms', false), ('privacy', true)]);
      expect(pending.first.acceptedVersion, '0.9.0');
    });

    test('accept sends the current version and treats ALREADY_EXISTS as success', () async {
      consent.alreadyActive = false;
      await GrpcConsentService.accept('health_data');
      expect((consent.accepted!.document, consent.accepted!.version, consent.accepted!.accepted), ('health_data', '3.0.0', true));
      consent.alreadyActive = true;
      await GrpcConsentService.accept('health_data');
    });
  });

  group('GrpcAddressSearchService', () {
    test('maps the candidates and the optional fields', () async {
      final found = await GrpcAddressSearchService.search(text: 'rua a', latitude: -23.5, longitude: -46.6);
      expect(found.single.placeId, 'p1');
      expect((found.single.addressLine2, found.single.postalCode), (null, '01000-000'));
      expect((address.last!.latitude, address.last!.longitude), (-23.5, -46.6));
    });

    test('an outage becomes a 503 AppException', () async {
      await expectLater(
        GrpcAddressSearchService.search(text: 'down'),
        throwsA(isA<AppException>().having((e) => e.statusCode, 'status', 503)),
      );
    });
  });
}
