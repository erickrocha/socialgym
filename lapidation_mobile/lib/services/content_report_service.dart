import '../utils/dio_client.dart';

class ContentReportService {
  static Future<void> create({
    required String targetType,
    required String targetId,
    required String postId,
    required String reason,
    String? details,
  }) async {
    final response = await DioClient().dio.post(
      '/timeline/api/reports',
      data: {
        'targetType': targetType,
        'targetId': targetId,
        'postId': postId,
        'reason': reason,
        if (details?.isNotEmpty == true) 'details': details,
      },
    );
    if (response.statusCode != 201) throw StateError('Could not submit report');
  }
}
