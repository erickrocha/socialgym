import '../utils/dio_client.dart';

class LegalDocument {
  final String document;
  final String version;
  final String title;
  final String content;

  const LegalDocument({
    required this.document,
    required this.version,
    required this.title,
    required this.content,
  });

  factory LegalDocument.fromJson(Map<String, dynamic> json) => LegalDocument(
    document: json['document'] as String,
    version: json['version'] as String,
    title: json['title'] as String,
    content: json['content'] as String,
  );
}

class LegalDocumentService {
  static Future<LegalDocument> get(String document) async {
    final response = await DioClient().dio.get('/legal/documents/$document');
    if (response.statusCode != 200) {
      throw StateError('Legal document unavailable');
    }
    return LegalDocument.fromJson(response.data as Map<String, dynamic>);
  }
}
