import 'package:dio/dio.dart';
import '../config/api_config.dart';

/// Centralized Dio client with authentication interceptor
class DioClient {
  static final DioClient _instance = DioClient._internal();
  late Dio _dio;

  /// Invoked whenever any API call comes back with `403 CONSENT_REQUIRED`
  /// (a missing/stale terms, privacy or health_data consent). Wired up once
  /// in `main.dart` to route the user to the pending-consents gate.
  static void Function()? onConsentRequired;

  factory DioClient() {
    return _instance;
  }

  DioClient._internal() {
    _dio = Dio(
      BaseOptions(
        baseUrl: ApiConfig.baseUrl,
        connectTimeout: ApiConfig.timeout,
        receiveTimeout: ApiConfig.timeout,
        sendTimeout: ApiConfig.timeout,
        validateStatus: (status) => status != null && status < 500,
      ),
    );

    // Add interceptors
    _dio.interceptors.add(AuthInterceptor());
    _dio.interceptors.add(ConsentInterceptor());
  }

  Dio get dio => _dio;

  /// Set the authentication token from the provider or storage
  void setAuthToken(String token) {
    _dio.options.headers['Authorization'] = 'Bearer $token';
  }

  /// Clear the authentication token
  void clearAuthToken() {
    _dio.options.headers.remove('Authorization');
  }
}

/// Interceptor to add Bearer token and default content-type to all requests
class AuthInterceptor extends Interceptor {
  @override
  void onRequest(RequestOptions options, RequestInterceptorHandler handler) {
    // Token is already set in DioClient.setAuthToken()
    // Set default JSON content-type if not already set
    if (!options.headers.containsKey('Content-Type')) {
      options.headers['Content-Type'] = 'application/json';
    }
    super.onRequest(options, handler);
  }
}

/// Detects `403 CONSENT_REQUIRED` on any response (the client's
/// `validateStatus` lets < 500 through as a normal response, so this also
/// checks `onError` for completeness) and notifies
/// [DioClient.onConsentRequired]. The response/error is passed through
/// untouched so existing call sites keep their own handling.
class ConsentInterceptor extends Interceptor {
  static bool _isConsentRequired(Response? response) {
    if (response?.statusCode != 403) return false;
    final data = response?.data;
    if (data is! Map) return false;
    final key = data['errorKey'];
    return key is String &&
        key.replaceAll('-', '_').toUpperCase() == 'CONSENT_REQUIRED';
  }

  @override
  void onResponse(Response response, ResponseInterceptorHandler handler) {
    if (_isConsentRequired(response)) {
      DioClient.onConsentRequired?.call();
    }
    super.onResponse(response, handler);
  }

  @override
  void onError(DioException err, ErrorInterceptorHandler handler) {
    if (_isConsentRequired(err.response)) {
      DioClient.onConsentRequired?.call();
    }
    super.onError(err, handler);
  }
}
