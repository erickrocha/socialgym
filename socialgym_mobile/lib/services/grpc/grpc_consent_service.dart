import 'package:grpc/grpc.dart' as grpc;
import 'package:socialgym_mobile/models/pending_consent.dart';
import 'package:socialgym_mobile/services/grpc/grpc_legal_document_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_workout_channel.dart';
import 'package:socialgym_mobile/src/generated/grpc/consent.pbgrpc.dart' as $consent;

/// gRPC client façade for the ConsentService.
class GrpcConsentService {
  GrpcConsentService._();

  static $consent.ConsentServiceClient? _override;

  /// Built per call on the shared channel: sign-out shuts the channels down, and a cached client
  /// would keep pointing at the dead one.
  static $consent.ConsentServiceClient _ensureClient() =>
      _override ?? $consent.ConsentServiceClient(GrpcWorkoutChannel.channel(), interceptors: GrpcWorkoutChannel.interceptors);

  /// Replaces the client (tests point it at a fake server).
  static void useClient($consent.ConsentServiceClient? client) => _override = client;

  /// Whether the person has an active consent for [document] **at its current version**. A consent
  /// accepted at an older version does not count: the backend enforces the current version, so the
  /// client must too.
  static Future<bool> hasActive(String document) async {
    final legal = await GrpcLegalDocumentService.get(document);
    return GrpcWorkoutChannel.guard('Could not load consents', () async {
      final response = await _ensureClient().listConsents(
        $consent.ListConsentsRequest(),
        options: GrpcWorkoutChannel.options,
      );
      return response.consents.any(
        (c) => c.document == document && !c.hasRevokedAt() && c.version == legal.version,
      );
    });
  }

  /// Legal documents whose current version the person has not accepted. Empty means nothing is blocking.
  static Future<List<PendingConsent>> pending() =>
      GrpcWorkoutChannel.guard('Could not load pending consents', () async {
        final response = await _ensureClient().listPendingConsents(
          $consent.ListPendingConsentsRequest(),
          options: GrpcWorkoutChannel.options,
        );
        return response.pending
            .map(
              (p) => PendingConsent(
                document: p.document,
                currentVersion: p.currentVersion,
                acceptedVersion: p.hasAcceptedVersion() ? p.acceptedVersion : null,
              ),
            )
            .toList();
      });

  static Future<void> accept(String document) async {
    final legal = await GrpcLegalDocumentService.get(document);
    try {
      await _ensureClient().acceptConsent(
        $consent.AcceptConsentRequest(document: document, version: legal.version, accepted: true),
        options: GrpcWorkoutChannel.options,
      );
    } on grpc.GrpcError catch (e) {
      // Already active at this version: idempotent success, as REST's 409 was.
      if (e.code == grpc.StatusCode.alreadyExists) return;
      throw GrpcWorkoutChannel.toAppException(e, 'Consent could not be saved');
    }
  }
}
