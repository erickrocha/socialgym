import 'package:socialgym_mobile/models/legal_document.dart';
import 'package:socialgym_mobile/services/grpc/grpc_workout_channel.dart';
import 'package:socialgym_mobile/src/generated/grpc/legal.pbgrpc.dart' as $legal;

/// gRPC client façade for the LegalDocumentService (public: no access token needed).
class GrpcLegalDocumentService {
  GrpcLegalDocumentService._();

  static $legal.LegalDocumentServiceClient? _override;

  /// Built per call on the shared channel: sign-out shuts the channels down, and a cached client
  /// would keep pointing at the dead one.
  static $legal.LegalDocumentServiceClient _ensureClient() =>
      _override ?? $legal.LegalDocumentServiceClient(GrpcWorkoutChannel.channel(), interceptors: GrpcWorkoutChannel.interceptors);

  /// Replaces the client (tests point it at a fake server).
  static void useClient($legal.LegalDocumentServiceClient? client) => _override = client;

  static LegalDocument _map($legal.LegalDocument d) =>
      LegalDocument(document: d.document, version: d.version, title: d.title, content: d.content);

  static Future<LegalDocument> get(String document) =>
      GrpcWorkoutChannel.guard('Legal document unavailable', () async {
        final response = await _ensureClient().getLegalDocument(
          $legal.GetLegalDocumentRequest(document: document),
          options: GrpcWorkoutChannel.options,
        );
        return _map(response);
      });

  /// All legal documents with their current versions (unauthenticated).
  static Future<List<LegalDocument>> list() =>
      GrpcWorkoutChannel.guard('Legal documents unavailable', () async {
        final response = await _ensureClient().listLegalDocuments(
          $legal.ListLegalDocumentsRequest(),
          options: GrpcWorkoutChannel.options,
        );
        return response.documents.map(_map).toList();
      });
}
