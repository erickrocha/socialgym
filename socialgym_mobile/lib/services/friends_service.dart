import 'package:dio/dio.dart';
import '../config/api_config.dart';
import '../models/friends_data.dart';
import '../models/person.dart';
import '../utils/dio_client.dart';
import 'base_service.dart';

class FriendsService {
  static final _dio = DioClient().dio;

  /// Fetch friends data including suggestions, friends, and requests.
  ///
  /// When [latitude]/[longitude] are given, suggestions are centered on that
  /// point (e.g. the device's current GPS position) instead of the person's
  /// saved home address.
  static Future<FriendsData> fetchFriends(
    String token, {
    double? latitude,
    double? longitude,
  }) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        ApiConfig.friendsEndpoint,
        queryParameters: {
          'latitude': ?latitude,
          'longitude': ?longitude,
        },
      );
      if (response.statusCode == 200) {
        final data = response.data as Map<String, dynamic>;
        return FriendsData.fromJson(data);
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to fetch friends',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to fetch friends');
    }
  }

  /// Combined "find friends" search: a name/username query, a location
  /// filter, or both together.
  static Future<List<Person>> searchFriends({
    required String token,
    String? query,
    double? latitude,
    double? longitude,
    double? radiusKm,
    int? limit,
  }) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        ApiConfig.friendsSearchEndpoint,
        queryParameters: {
          'query': ?query,
          'latitude': ?latitude,
          'longitude': ?longitude,
          'radius_km': ?radiusKm,
          'limit': ?limit,
        },
      );
      if (response.statusCode == 200) {
        return (response.data as List<dynamic>)
            .map((e) => Person.fromJson(e as Map<String, dynamic>))
            .toList();
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to search friends',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to search friends');
    }
  }

  /// Send a friend request to a person.
  static Future<void> sendFriendRequest(int personId, String token) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.put('${ApiConfig.friendsRequestEndpoint}/$personId');
      if (response.statusCode != 200 && response.statusCode != 201) {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to send friend request',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to send friend request');
    }
  }

  /// Accept a friend request.
  static Future<void> acceptFriendRequest(int personId, String token) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.put('${ApiConfig.friendsAcceptEndpoint}/$personId');
      if (response.statusCode != 200 && response.statusCode != 201) {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to accept friend request',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to accept friend request');
    }
  }

  /// Reject/decline a friend request.
  static Future<void> rejectFriendRequest(int personId, String token) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.put('${ApiConfig.friendsRejectEndpoint}/$personId');
      if (response.statusCode != 200 && response.statusCode != 201) {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to reject friend request',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to reject friend request');
    }
  }

  /// Cancel a sent friend request.
  static Future<void> cancelFriendRequest(int personId, String token) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.delete('${ApiConfig.friendsCancelEndpoint}/$personId');
      if (response.statusCode != 200 && response.statusCode != 204) {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to cancel friend request',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to cancel friend request');
    }
  }

  /// Remove a friend (unfriend).
  static Future<void> removeFriend(int personId, String token) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.delete('${ApiConfig.friendsEndpoint}/$personId');
      if (response.statusCode != 200 && response.statusCode != 204) {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to remove friend',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to remove friend');
    }
  }
}
