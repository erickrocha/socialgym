import '../models/pending_consent.dart';
import '../utils/dio_client.dart';
import 'legal_document_service.dart';

class ConsentService {
  static final _dio = DioClient().dio;

  /// Whether the person has an active consent for [document] **at its current
  /// version**. A consent accepted at an older version does not count — the
  /// backend enforces the current version, so the client must too.
  static Future<bool> hasActive(String document) async {
    final legal = await LegalDocumentService.get(document);
    final response = await _dio.get('/workout/api/people/me/consents');
    if (response.statusCode != 200) return false;
    final rows = (response.data as List<dynamic>).cast<Map<String, dynamic>>();
    return rows.any(
      (row) =>
          row['document'] == document &&
          row['revokedAt'] == null &&
          row['version'] == legal.version,
    );
  }

  /// Legal documents whose current version the person has not accepted.
  /// Empty means nothing is blocking them.
  static Future<List<PendingConsent>> pending() async {
    final response = await _dio.get('/workout/api/people/me/consents/pending');
    if (response.statusCode != 200) {
      throw StateError('Could not load pending consents');
    }
    return (response.data as List<dynamic>)
        .cast<Map<String, dynamic>>()
        .map(PendingConsent.fromJson)
        .toList();
  }

  static Future<void> accept(String document) async {
    final legal = await LegalDocumentService.get(document);
    final response = await _dio.post(
      '/workout/api/people/me/consents',
      data: {'document': document, 'version': legal.version, 'accepted': true},
    );
    // 200 = saved; 409 = already active at this version (idempotent success).
    if (response.statusCode != 200 && response.statusCode != 409) {
      throw StateError('Consent could not be saved');
    }
  }
}
