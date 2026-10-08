import 'package:grpc/grpc.dart' as grpc;

abstract class BaseService {
  /// Invoked whenever a call is refused because a required legal consent (terms, privacy or
  /// health_data) is missing or out of date. Wired up once in `main.dart` to route the user to the
  /// pending-consents gate.
  static void Function()? onConsentRequired;

  /// Convert gRPC error to AppException
  static AppException handleGrpcError(grpc.GrpcError error, String fallback) {
    final message = error.message ?? fallback;
    final consentRequired = error.code == grpc.StatusCode.permissionDenied && message.contains('consent is required');
    // The gate lists the terms and privacy documents; `health_data` is asked for where it is needed
    // (the evolution page), so it does not pop the gate.
    if (consentRequired && (message.startsWith('terms ') || message.startsWith('privacy '))) {
      onConsentRequired?.call();
    }
    return AppException(
      statusCode: _grpcStatusToHttpStatus(error.code),
      message: message,
      // The timeline answers a missing legal consent as PERMISSION_DENIED with
      // "<document> consent is required"; REST carried it as the CONSENT_REQUIRED error key.
      errorKey: consentRequired ? 'CONSENT_REQUIRED' : null,
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

  /// Machine-readable error code from the API response body (`errorKey`),
  /// when present. Stable across locales, unlike [message].
  final String? errorKey;

  AppException({
    required this.statusCode,
    required this.message,
    this.errorKey,
  });

  /// True when the backend rejected the request because a required legal
  /// consent (terms / privacy / health_data) is missing or out of date.
  /// Normalizes both `CONSENT_REQUIRED` (timeline) and `consent-required`
  /// (older workout responses).
  bool get isConsentRequired =>
      errorKey?.replaceAll('-', '_').toUpperCase() == 'CONSENT_REQUIRED';

  @override
  String toString() => 'AppException($statusCode): $message';
}
