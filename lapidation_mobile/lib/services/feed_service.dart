import 'package:dio/dio.dart';
import '../config/api_config.dart';
import '../models/feed_post.dart';
import '../utils/dio_client.dart';
import 'base_service.dart';

class FeedService {
  static final _dio = DioClient().dio;

  static Future<List<FeedPost>> fetchPosts(String token, {int page = 0}) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        ApiConfig.feedEndpoint,
        queryParameters: {'page': page},
      );
      if (response.statusCode == 200) {
        final List<dynamic> data = response.data;
        return data
            .map((e) => FeedPost.fromJson(e as Map<String, dynamic>))
            .toList();
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to load feed',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to load feed');
    }
  }

  static Future<List<FeedPost>> fetchBusinessFeed(
    String token,
    String businessProfileUuid, {
    int page = 0,
  }) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        '${ApiConfig.businessFeedEndpoint}/$businessProfileUuid',
        queryParameters: {'page': page},
      );
      if (response.statusCode == 200) {
        final List<dynamic> data = response.data;
        return data
            .map((e) => FeedPost.fromJson(e as Map<String, dynamic>))
            .toList();
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to load business feed',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to load business feed');
    }
  }

  static Future<FeedPost> createPost(
    Map<String, dynamic> data,
    String token,
  ) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.post(ApiConfig.postEndpoint, data: data);
      if (response.statusCode == 200 || response.statusCode == 201) {
        return FeedPost.fromJson(response.data as Map<String, dynamic>);
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to create post',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to create post');
    }
  }

  static Future<FeedPost> addComment(
    Map<String, dynamic> data,
    String token,
  ) async {
    try {
      DioClient().setAuthToken(token);
      final postId = data['postUuid'];
      final response = await _dio.post(
        '${ApiConfig.postEndpoint}/$postId/comments',
        data: data,
      );
      if (response.statusCode == 200 || response.statusCode == 201) {
        return FeedPost.fromJson(response.data as Map<String, dynamic>);
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to add comment',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to add comment');
    }
  }

  static Future<void> addReaction(
    String postId,
    String reactionType,
    String authorId,
    String authorName,
    String token,
  ) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.post(
        '${ApiConfig.postEndpoint}/$postId/reactions',
        data: {
          'reactionType': reactionType,
          'authorId': authorId,
          'authorName': authorName,
        },
      );
      if (response.statusCode != 200 && response.statusCode != 201) {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to add reaction',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to add reaction');
    }
  }
}
