import '../utils/dio_client.dart';
import 'legal_document_service.dart';

class ConsentService {
  static final _dio = DioClient().dio;

  static Future<bool> hasActive(String document) async {
    final response = await _dio.get('/workout/api/people/me/consents');
    if (response.statusCode != 200) return false;
    final rows = response.data as List<dynamic>;
    return rows.cast<Map<String, dynamic>>().any(
      (row) => row['document'] == document && row['revokedAt'] == null,
    );
  }

  static Future<void> accept(String document) async {
    final legal = await LegalDocumentService.get(document);
    final response = await _dio.post(
      '/workout/api/people/me/consents',
      data: {'document': document, 'version': legal.version, 'accepted': true},
    );
    if (response.statusCode != 200) {
      throw StateError('Consent could not be saved');
    }
  }
}
