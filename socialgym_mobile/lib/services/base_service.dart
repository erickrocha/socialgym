import 'package:dio/dio.dart';

abstract class BaseService {
  /// Extract error message from Dio response
  static String extractErrorFromDio(dynamic response, String fallback) {
    try {
      if (response is Map<String, dynamic>) {
        return response['message'] ?? fallback;
      }
      return fallback;
    } catch (_) {
      return fallback;
    }
  }

  /// Convert Dio exception to AppException
  static AppException handleDioError(DioException error, String fallback) {
    int statusCode = error.response?.statusCode ?? 0;
    String message = fallback;

    // Try to extract message from response body
    if (error.response?.data is Map<String, dynamic>) {
      final data = error.response!.data as Map<String, dynamic>;
      message = data['message'] ?? error.message ?? fallback;
    } else if (error.message != null) {
      message = error.message!;
    }

    return AppException(statusCode: statusCode, message: message);
  }
}

class AppException implements Exception {
  final int statusCode;
  final String message;

  AppException({required this.statusCode, required this.message});

  @override
  String toString() => 'AppException($statusCode): $message';
}
