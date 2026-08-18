import 'package:dio/dio.dart';
import '../config/api_config.dart';
import '../models/workout_session.dart';
import '../utils/dio_client.dart';
import 'base_service.dart';

class WorkoutSessionService {
  static final _dio = DioClient().dio;
  static Future<Map<String, dynamic>> saveWorkoutSession(
    Map<String, dynamic> sessionData,
    String token,
  ) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.post(ApiConfig.workoutSessionsEndpoint, data: sessionData);
      if (response.statusCode == 200 || response.statusCode == 201) {
        return response.data as Map<String, dynamic>;
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to save workout session',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to save workout session');
    }
  }

  static Future<List<WorkoutSession>> fetchWorkoutSessions({
    required String personUuid,
    required DateTime startDate,
    required DateTime endDate,
    required String token,
  }) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        ApiConfig.workoutSessionsEndpoint,
        queryParameters: {
          'startDate': _toIsoDateTime(startDate, endOfDay: false),
          'endDate': _toIsoDateTime(endDate, endOfDay: true),
        },
      );
      if (response.statusCode == 200) {
        final List<dynamic> data = response.data;
        return data.map((e) => WorkoutSession.fromJson(e as Map<String, dynamic>)).toList();
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to fetch workout sessions',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to fetch workout sessions');
    }
  }

  static String _toIsoDateTime(DateTime date, {bool endOfDay = false}) {
    final y = date.year.toString().padLeft(4, '0');
    final m = date.month.toString().padLeft(2, '0');
    final d = date.day.toString().padLeft(2, '0');
    final time = endOfDay ? 'T23:59:59' : 'T00:00:00';
    return '$y-$m-$d$time';
  }
}
