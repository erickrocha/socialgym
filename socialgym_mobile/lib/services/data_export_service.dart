import '../utils/dio_client.dart';

class DataExportJob {
  final String id;
  final String status;
  final DateTime createdAt;
  final DateTime? expiresAt;
  const DataExportJob({
    required this.id,
    required this.status,
    required this.createdAt,
    this.expiresAt,
  });
  factory DataExportJob.fromJson(Map<String, dynamic> json) => DataExportJob(
    id: json['id'] as String,
    status: json['status'] as String,
    createdAt: DateTime.parse(json['createdAt'] as String),
    expiresAt: json['expiresAt'] == null
        ? null
        : DateTime.parse(json['expiresAt'] as String),
  );
}

class DataExportService {
  static final _dio = DioClient().dio;
  static Future<List<DataExportJob>> list() async {
    final response = await _dio.get('/workout/api/people/me/data-exports');
    if (response.statusCode != 200) throw StateError('Could not load exports');
    return (response.data as List<dynamic>)
        .map((e) => DataExportJob.fromJson(e as Map<String, dynamic>))
        .toList();
  }

  static Future<DataExportJob> create() async {
    final response = await _dio.post('/workout/api/people/me/data-exports');
    if (response.statusCode != 200) throw StateError('Could not create export');
    return DataExportJob.fromJson(response.data as Map<String, dynamic>);
  }

  static Future<String> downloadUrl(String id) async {
    final response = await _dio.get(
      '/workout/api/people/me/data-exports/$id/download',
    );
    if (response.statusCode != 200) throw StateError('Export not ready');
    return (response.data as Map<String, dynamic>)['url'] as String;
  }
}
