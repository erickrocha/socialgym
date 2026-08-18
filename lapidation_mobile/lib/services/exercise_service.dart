import 'package:dio/dio.dart';
import '../config/api_config.dart';
import '../models/paginated_exercise_response.dart';
import '../utils/dio_client.dart';
import 'base_service.dart';

class ExerciseService extends BaseService {
  static final _dio = DioClient().dio;


  /// Delete an exercise by ID.
  static Future<void> deleteExercise(int exerciseId, String token) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.delete('${ApiConfig.exercisesEndpoint}/$exerciseId');
      if (response.statusCode != 200 && response.statusCode != 204) {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to delete exercise',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to delete exercise');
    }
  }

  /// Fetch paginated exercises with optional filters using a POST request.
  static Future<PaginatedExerciseResponse> fetchPaginatedExercises({
    required String token,
    List<int>? owners,
    String category = 'Force',
    String visibility = 'Public',
    int pageNumber = 1,
    int pageSize = 20,
    String sortBy = 'created_at_desc',
  }) async {
    try {
      DioClient().setAuthToken(token);
      final body = {
        'category': category,
        'owners': owners ?? [],
        'visibility': visibility,
        'pageNumber': pageNumber,
        'pageSize': pageSize,
        'sortBy': sortBy,
      };
      final response = await _dio.post(ApiConfig.exercisesQueryEndpoint, data: body);
      if (response.statusCode == 200) {
        final data = response.data as Map<String, dynamic>;
        return PaginatedExerciseResponse.fromJson(data);
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to load exercises',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to load exercises');
    }
  }
}
