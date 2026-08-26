import 'package:dio/dio.dart';

import '../config/api_config.dart';
import '../models/account_deletion_status.dart';
import '../utils/dio_client.dart';
import 'base_service.dart';

/// Account deletion (App Store / Play Store requirement): request deletion
/// (immediate or after the server's grace period) and cancel a pending one.
class AccountDeletionService {
  static final _dio = DioClient().dio;

  static Future<AccountDeletionStatus> requestDeletion({
    required bool immediate,
    required String token,
  }) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.post(
        ApiConfig.accountDeleteEndpoint,
        data: {'immediate': immediate},
      );
      if (response.statusCode == 200) {
        return AccountDeletionStatus.fromJson(
          response.data as Map<String, dynamic>,
        );
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to delete account',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to delete account');
    }
  }

  static Future<void> cancelDeletion({required String token}) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.post(ApiConfig.accountCancelDeletionEndpoint);
      if (response.statusCode != 200) {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message:
              response.data?['message'] ?? 'Failed to cancel account deletion',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to cancel account deletion');
    }
  }
}
