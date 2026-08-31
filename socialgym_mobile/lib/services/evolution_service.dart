import 'package:dio/dio.dart';

import '../config/api_config.dart';
import '../models/evolution_check_in.dart';
import '../utils/dio_client.dart';
import 'base_service.dart';

class EvolutionService {
  static final _dio = DioClient().dio;

  /// Fetch evolution check-ins for a person within a date range.
  ///
  /// * startDate: ISO 8601 format (e.g. 2026-03-25T00:00:00)
  /// * endDate: ISO 8601 format (e.g. 2026-04-01T23:59:59)
  static Future<List<EvolutionCheckIn>> fetchEvolutionCheckIns({
    required DateTime startDate,
    required DateTime endDate,
    required String token,
  }) async {
    try {
      DioClient().setAuthToken(token);

      final startStr = startDate.toIso8601String();
      final endStr = endDate.toIso8601String();

      final response = await _dio.get(
        '${ApiConfig.baseUrl}/timeline/api/evolution-checkin',
        queryParameters: {'startDate': startStr, 'endDate': endStr},
        options: Options(contentType: 'application/json'),
      );

      if (response.statusCode == 200) {
        final List<dynamic> data = response.data;
        return data.map((e) => EvolutionCheckIn.fromJson(e as Map<String, dynamic>)).toList();
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to load evolution data',
          errorKey: response.data?['errorKey'] as String?,
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to load evolution data');
    }
  }

  /// Create a new evolution check-in.
  static Future<EvolutionCheckIn> createEvolutionCheckIn({
    required Map<String, dynamic> payload,
    required String token,
  }) async {
    try {
      DioClient().setAuthToken(token);

      final response = await _dio.post(
        '${ApiConfig.baseUrl}${ApiConfig.evolutionCheckInEndpoint}',
        data: payload,
        options: Options(contentType: 'application/json'),
      );

      if (response.statusCode == 200 || response.statusCode == 201) {
        final data = response.data as Map<String, dynamic>;
        return EvolutionCheckIn.fromJson(data);
      }

      throw AppException(
        statusCode: response.statusCode ?? 500,
        message: response.data?['message'] ?? 'Failed to create evolution check-in',
        errorKey: response.data?['errorKey'] as String?,
      );
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to create evolution check-in');
    }
  }
}
