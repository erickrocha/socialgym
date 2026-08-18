import 'package:dio/dio.dart';
import '../config/api_config.dart';
import '../models/resource.dart';
import '../utils/dio_client.dart';
import 'base_service.dart';

class ResourceService {
  static final _dio = DioClient().dio;
  static Future<AppResources> fetchResources(String token) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(ApiConfig.resourceEndpoint);
      if (response.statusCode == 200) {
        final data = response.data as Map<String, dynamic>;
        return AppResources.fromJson(data);
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to fetch resources',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to fetch resources');
    }
  }
}
