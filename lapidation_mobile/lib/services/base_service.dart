import 'package:dio/dio.dart';
import 'package:grpc/grpc.dart' as grpc;

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

  /// Convert gRPC error to AppException
  static AppException handleGrpcError(grpc.GrpcError error, String fallback) {
    return AppException(
      statusCode: _grpcStatusToHttpStatus(error.code),
      message: error.message ?? fallback,
    );
  }

  static int _grpcStatusToHttpStatus(int code) {
    switch (code) {
      case grpc.StatusCode.invalidArgument:
      case grpc.StatusCode.failedPrecondition:
      case grpc.StatusCode.outOfRange:
        return 400;
      case grpc.StatusCode.unauthenticated:
        return 401;
      case grpc.StatusCode.permissionDenied:
        return 403;
      case grpc.StatusCode.notFound:
        return 404;
      case grpc.StatusCode.aborted:
      case grpc.StatusCode.alreadyExists:
        return 409;
      case grpc.StatusCode.resourceExhausted:
        return 429;
      case grpc.StatusCode.unimplemented:
        return 501;
      case grpc.StatusCode.unavailable:
        return 503;
      case grpc.StatusCode.deadlineExceeded:
        return 504;
      default:
        return 500;
    }
  }
}

class AppException implements Exception {
  final int statusCode;
  final String message;

  AppException({required this.statusCode, required this.message});

  @override
  String toString() => 'AppException($statusCode): $message';
}
