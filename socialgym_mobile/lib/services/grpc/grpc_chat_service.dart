import 'package:fixnum/fixnum.dart';
import 'package:image_picker/image_picker.dart';

import '../../models/chat_message.dart';
import '../../models/conversation.dart';
import '../../src/generated/grpc/timeline/chat.pbgrpc.dart' as $chat;
import '../upload_service.dart';
import 'grpc_timeline.dart';

/// Chat operations over gRPC, replacing the REST `ChatService`. The real-time part is
/// `GrpcChatStream`.
class GrpcChatService {
  GrpcChatService._();

  static $chat.ChatServiceClient get _client =>
      $chat.ChatServiceClient(GrpcTimeline.channel, interceptors: GrpcTimeline.interceptors);

  static Future<List<Conversation>> listConversations({int page = 0}) => GrpcTimeline.run(() async {
        final response = await _client.listConversations(
          $chat.ListConversationsRequest(page: page),
          options: GrpcTimeline.options,
        );
        return response.conversations.map((c) => Conversation.fromJson(GrpcTimeline.json(c))).toList();
      }, 'Failed to load conversations');

  /// Which of [personUuids] are online. Returns an empty set on failure: presence is decoration,
  /// never a reason to break a screen.
  static Future<Set<String>> presence(List<String> personUuids) async {
    if (personUuids.isEmpty) return const {};
    try {
      final response = await _client.getPresence(
        $chat.GetPresenceRequest(personUuids: personUuids),
        options: GrpcTimeline.options,
      );
      return response.online.toSet();
    } catch (_) {
      return const {};
    }
  }

  static Future<Conversation> createDirect(String targetPersonUuid) => GrpcTimeline.run(() async {
        final conversation = await _client.createDirectConversation(
          $chat.CreateDirectConversationRequest(targetPersonUuid: targetPersonUuid),
          options: GrpcTimeline.options,
        );
        return Conversation.fromJson(GrpcTimeline.json(conversation));
      }, 'Failed to open conversation');

  static Future<Conversation> createTeamGroup(String businessProfileUuid) => GrpcTimeline.run(() async {
        final conversation = await _client.createBusinessTeamGroup(
          $chat.CreateBusinessTeamGroupRequest(businessProfileUuid: businessProfileUuid),
          options: GrpcTimeline.options,
        );
        return Conversation.fromJson(GrpcTimeline.json(conversation));
      }, 'Failed to open conversation');

  static Future<Conversation> createBusinessDirect(
    String businessProfileUuid, {
    String? memberPersonUuid,
  }) => GrpcTimeline.run(() async {
        final conversation = await _client.createBusinessDirectConversation(
          $chat.CreateBusinessDirectConversationRequest(
            businessProfileUuid: businessProfileUuid,
            memberPersonUuid: memberPersonUuid,
          ),
          options: GrpcTimeline.options,
        );
        return Conversation.fromJson(GrpcTimeline.json(conversation));
      }, 'Failed to open conversation');

  /// A page of messages, or with [since] (epoch ms) the messages newer than it.
  static Future<List<ChatMessage>> listMessages(
    String conversationUuid, {
    int page = 0,
    int? since,
  }) => GrpcTimeline.run(() async {
        final response = await _client.listMessages(
          $chat.ListMessagesRequest(
            conversationUuid: conversationUuid,
            page: page,
            sinceEpochMs: since == null ? null : Int64(since),
          ),
          options: GrpcTimeline.options,
        );
        return response.messages.map((m) => ChatMessage.fromJson(GrpcTimeline.json(m))).toList();
      }, 'Failed to load messages');

  static Future<ChatMessage> sendMessage(
    String conversationUuid, {
    String body = '',
    List<Map<String, dynamic>> media = const [],
    required String clientMessageId,
  }) => GrpcTimeline.run(() async {
        final message = await _client.sendMessage(
          $chat.SendMessageRequest(
            conversationUuid: conversationUuid,
            body: body,
            media: media.map(mediaOf),
            clientMessageId: clientMessageId,
          ),
          options: GrpcTimeline.options,
        );
        return ChatMessage.fromJson(GrpcTimeline.json(message));
      }, 'Failed to send message');

  static Future<void> markRead(String conversationUuid, String lastReadMessageUuid) =>
      GrpcTimeline.run(() async {
        await _client.markConversationRead(
          $chat.MarkConversationReadRequest(
            conversationUuid: conversationUuid,
            lastReadMessageUuid: lastReadMessageUuid,
          ),
          options: GrpcTimeline.options,
        );
      }, 'Failed to mark conversation read');

  /// `{mediaType, objectKey}` as the screens build it, to the message type.
  static $chat.MessageMedia mediaOf(Map<String, dynamic> media) => $chat.MessageMedia(
        mediaType: '${media['mediaType'] ?? ''}',
        objectKey: '${media['objectKey'] ?? ''}',
      );

  /// Uploads picked images to S3 (the MediaService pre-signed URL) and returns the
  /// `{mediaType, objectKey}` maps ready for [sendMessage].
  static Future<List<Map<String, dynamic>>> uploadImages(List<XFile> files) async {
    final media = <Map<String, dynamic>>[];
    for (final file in files) {
      final presigned = await UploadService.uploadPostMedia(file, 'chat');
      media.add({'mediaType': 'Image', 'objectKey': presigned.objectKey});
    }
    return media;
  }
}
