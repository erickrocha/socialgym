import 'package:dio/dio.dart';

import '../config/api_config.dart';
import '../models/auth_response.dart';
import '../models/sign_up_request.dart';
import '../utils/dio_client.dart';
import 'base_service.dart';

class AuthService {
  static final _dio = DioClient().dio;

  /// Sign in with email and password.
  /// The web app sends form-urlencoded data to POST /login.
  static Future<AuthResponse> signIn({
    required String email,
    required String password,
  }) async {
    try {
      final response = await _dio.post(
        ApiConfig.loginEndpoint,
        options: Options(contentType: 'application/x-www-form-urlencoded'),
        data: 'email=$email&password=$password',
      );

      if (response.statusCode == 200) {
        final data = response.data as Map<String, dynamic>;
        return AuthResponse.fromJson(data);
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Login failed',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Login failed');
    }
  }

  /// Sign up with user information.
  /// The web app sends JSON data to POST /signup.
  static Future<AuthResponse> signUp(SignUpRequest request) async {
    try {
      final response = await _dio.post(
        ApiConfig.signUpEndpoint,
        options: Options(contentType: 'application/json'),
        data: request.toJson(),
      );

      if (response.statusCode == 200 || response.statusCode == 201) {
        final data = response.data as Map<String, dynamic>;
        return AuthResponse.fromJson(data);
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Sign up failed',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Sign up failed');
    }
  }
}
