import 'package:dio/dio.dart';
import 'package:lapidation_mobile/config/api_config.dart';
import 'package:lapidation_mobile/models/notification.dart';
import 'package:lapidation_mobile/services/base_service.dart';
import 'package:lapidation_mobile/utils/dio_client.dart';

class NotificationService {
  static final Dio _dio = DioClient().dio;

  static Future<List<Notification>> fetchNotifications(String token,String ownerUuid,{bool unreadOnly = false,int limit = 50,}) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        '${ApiConfig.notificationsEndpoint}/$ownerUuid',
        queryParameters: {'unread_only': unreadOnly, 'limit': limit},
      );

      if (response.statusCode == 200) {
        final notifications = _extractNotificationList(response.data);
        return notifications
            .map((item) => Notification.fromJson(item))
            .toList();
      }

      throw AppException(
        statusCode: response.statusCode ?? 500,
        message: BaseService.extractErrorFromDio(
          response.data,
          'Failed to load notifications',
        ),
      );
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to load notifications');
    }
  }

  static Future<Notification> markNotificationAsRead(String token,String ownerUuid,String idempotencyKey) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.put(
        '${ApiConfig.notificationsEndpoint}/$ownerUuid/read/$idempotencyKey',
        data: {'read': true},
      );

      if (response.statusCode == 200) {
        final notification = _extractNotificationMap(response.data);
        return Notification.fromJson(notification);
      }

      throw AppException(
        statusCode: response.statusCode ?? 500,
        message: BaseService.extractErrorFromDio(
          response.data,
          'Failed to mark notification as read',
        ),
      );
    } on DioException catch (e) {
      throw BaseService.handleDioError(
        e,
        'Failed to mark notification as read',
      );
    }
  }

  static List<Map<String, dynamic>> _extractNotificationList(dynamic data) {
    if (data is List) {
      return data.whereType<Map<String, dynamic>>().toList();
    }

    if (data is Map<String, dynamic>) {
      final nested =
          data['notifications'] ??
          data['items'] ??
          data['data'] ??
          data['content'];
      if (nested is List) {
        return nested.whereType<Map<String, dynamic>>().toList();
      }
    }

    return const [];
  }

  static Map<String, dynamic> _extractNotificationMap(dynamic data) {
    if (data is Map<String, dynamic>) {
      final nested = data['notification'] ?? data['data'];
      if (nested is Map<String, dynamic>) {
        return nested;
      }
      return data;
    }

    return const {};
  }
}
