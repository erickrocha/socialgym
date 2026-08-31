import 'package:dio/dio.dart';

import '../config/api_config.dart';
import '../models/auth_response.dart';
import '../utils/dio_client.dart';
import 'base_service.dart';

/// REST calls that switch the authenticated JWT's business-profile context.
/// Distinct from grpc/grpc_business_profile_service.dart, which manages the
/// business profile record itself (name, address, images).
class BusinessProfileRestService {
  static final _dio = DioClient().dio;

  /// Activate a business profile, returning a new token scoped to it.
  static Future<AuthResponse> activate({
    required String businessProfileUuid,
    required String token,
  }) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.post(
        ApiConfig.authProfileActivateEndpoint(businessProfileUuid),
        options: Options(headers: {'Content-Type': 'application/json'}),
      );
      if (response.statusCode == 200) {
        return AuthResponse.fromJson(response.data as Map<String, dynamic>);
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message:
              response.data?['message'] ??
              'Failed to activate business profile',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(
        e,
        'Failed to activate business profile',
      );
    }
  }

  /// Deactivate the current business profile, returning a new token back in
  /// personal context.
  static Future<AuthResponse> deactivate({required String token}) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.post(
        ApiConfig.authProfileDeactivateEndpoint,
        options: Options(headers: {'Content-Type': 'application/json'}),
      );

      if (response.statusCode == 200) {
        return AuthResponse.fromJson(response.data as Map<String, dynamic>);
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message:
              response.data?['message'] ??
              'Failed to deactivate business profile',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(
        e,
        'Failed to deactivate business profile',
      );
    }
  }
}
