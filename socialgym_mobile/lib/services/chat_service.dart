import 'package:dio/dio.dart';
import 'package:image_picker/image_picker.dart';
import 'package:socialgym_mobile/config/api_config.dart';
import 'package:socialgym_mobile/models/chat_message.dart';
import 'package:socialgym_mobile/models/conversation.dart';
import 'package:socialgym_mobile/services/base_service.dart';
import 'package:socialgym_mobile/services/upload_service.dart';
import 'package:socialgym_mobile/utils/dio_client.dart';

class ChatService {
  static final Dio _dio = DioClient().dio;
  static const String _base = ApiConfig.chatConversationsEndpoint;

  static List<Map<String, dynamic>> _asList(dynamic data) {
    if (data is List) return data.whereType<Map<String, dynamic>>().toList();
    return const [];
  }

  static Future<List<Conversation>> listConversations(
    String token, {
    int page = 0,
  }) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(_base, queryParameters: {'page': page});
      return _asList(response.data).map(Conversation.fromJson).toList();
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to load conversations');
    }
  }

  /// Which of [personUuids] currently hold a chat WebSocket. Returns an empty
  /// set on failure — presence is decoration, never a reason to break a screen.
  static Future<Set<String>> presence(
    String token,
    List<String> personUuids,
  ) async {
    if (personUuids.isEmpty) return const {};
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        ApiConfig.chatPresenceEndpoint,
        queryParameters: {'uuids': personUuids.join(',')},
      );
      final data = response.data;
      if (data is! Map) return const {};
      return (data['online'] as List? ?? const [])
          .whereType<String>()
          .toSet();
    } on DioException {
      return const {};
    }
  }

  static Future<Conversation> createDirect(
    String token,
    String targetPersonUuid,
  ) async {
    return _createConversation(token, '$_base/direct', {
      'targetPersonUuid': targetPersonUuid,
    });
  }

  static Future<Conversation> createTeamGroup(
    String token,
    String businessProfileUuid,
  ) async {
    return _createConversation(token, '$_base/business-team', {
      'businessProfileUuid': businessProfileUuid,
    });
  }

  static Future<Conversation> createBusinessDirect(
    String token,
    String businessProfileUuid, {
    String? memberPersonUuid,
  }) async {
    return _createConversation(token, '$_base/business-direct', {
      'businessProfileUuid': businessProfileUuid,
      if (memberPersonUuid != null) 'memberPersonUuid': memberPersonUuid,
    });
  }

  static Future<Conversation> _createConversation(
    String token,
    String path,
    Map<String, dynamic> body,
  ) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.post(path, data: body);
      return Conversation.fromJson(response.data as Map<String, dynamic>);
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to open conversation');
    }
  }

  static Future<List<ChatMessage>> listMessages(
    String token,
    String conversationUuid, {
    int page = 0,
    int? since,
  }) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        '$_base/$conversationUuid/messages',
        queryParameters: since != null ? {'since': since} : {'page': page},
      );
      return _asList(response.data).map(ChatMessage.fromJson).toList();
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to load messages');
    }
  }

  static Future<ChatMessage> sendMessage(
    String token,
    String conversationUuid, {
    String body = '',
    List<Map<String, dynamic>> media = const [],
    required String clientMessageId,
  }) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.post(
        '$_base/$conversationUuid/messages',
        data: {
          'body': body,
          'media': media,
          'clientMessageId': clientMessageId,
        },
      );
      return ChatMessage.fromJson(response.data as Map<String, dynamic>);
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to send message');
    }
  }

  static Future<void> markRead(
    String token,
    String conversationUuid,
    String lastReadMessageUuid,
  ) async {
    try {
      DioClient().setAuthToken(token);
      await _dio.put(
        '$_base/$conversationUuid/read',
        data: {'lastReadMessageUuid': lastReadMessageUuid},
      );
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to mark conversation read');
    }
  }

  /// Uploads picked images to S3 (shared media-upload endpoint) and returns the
  /// `{mediaType, objectKey}` maps ready for [sendMessage].
  static Future<List<Map<String, dynamic>>> uploadImages(
    String token,
    List<XFile> files,
  ) async {
    final media = <Map<String, dynamic>>[];
    for (final file in files) {
      final presigned = await UploadService.uploadPostMedia(token, file, 'chat');
      media.add({'mediaType': 'Image', 'objectKey': presigned.objectKey});
    }
    return media;
  }
}
