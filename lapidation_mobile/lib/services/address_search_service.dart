import 'package:dio/dio.dart';

import '../config/api_config.dart';
import '../models/address_candidate.dart';
import '../utils/dio_client.dart';
import 'base_service.dart';

/// Address search backed by the workout service's Google Places-powered
/// endpoint (GET /workout/api/address/search).
class AddressSearchService {
  static final _dio = DioClient().dio;

  static Future<List<AddressCandidate>> search({
    required String text,
    required String token,
    double? latitude,
    double? longitude,
  }) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        ApiConfig.addressSearchEndpoint,
        queryParameters: {
          'text': text,
          'latitude': ?latitude,
          'longitude': ?longitude,
        },
      );

      if (response.statusCode == 200) {
        return (response.data as List<dynamic>)
            .map((e) => AddressCandidate.fromJson(e as Map<String, dynamic>))
            .toList();
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to search address',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to search address');
    }
  }
}
