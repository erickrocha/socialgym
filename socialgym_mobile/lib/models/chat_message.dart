class ChatMessageMedia {
  final String mediaType;
  final String objectKey;
  final String url;

  const ChatMessageMedia({
    required this.mediaType,
    required this.objectKey,
    required this.url,
  });

  factory ChatMessageMedia.fromJson(Map<String, dynamic> json) =>
      ChatMessageMedia(
        mediaType: (json['mediaType'] ?? json['media_type'] ?? 'Image') as String,
        objectKey: (json['objectKey'] ?? json['object_key'] ?? '') as String,
        url: (json['url'] ?? '') as String,
      );

  Map<String, dynamic> toJson() => {
    'mediaType': mediaType,
    'objectKey': objectKey,
  };
}

class ChatMessage {
  final String uuid;
  final String conversationUuid;
  final String senderPersonUuid;
  final String senderKind; // "Person" | "BusinessProfile"
  final String senderDisplayName;
  final String? senderAvatarUrl;
  final String? senderBusinessProfileUuid;
  final String body;
  final List<ChatMessageMedia> media;
  final String clientMessageId;
  final DateTime sentAt;

  /// Local-only delivery state. Never parsed from JSON: anything the server
  /// hands back is, by definition, delivered.
  final bool pending;
  final bool failed;

  const ChatMessage({
    required this.uuid,
    required this.conversationUuid,
    required this.senderPersonUuid,
    required this.senderKind,
    required this.senderDisplayName,
    this.senderAvatarUrl,
    this.senderBusinessProfileUuid,
    required this.body,
    required this.media,
    required this.clientMessageId,
    required this.sentAt,
    this.pending = false,
    this.failed = false,
  });

  /// An optimistic message rendered the instant the user hits send, before any
  /// I/O. Replaced by the server's copy via [clientMessageId] dedupe.
  factory ChatMessage.pending({
    required String conversationUuid,
    required String senderPersonUuid,
    required String senderDisplayName,
    required String body,
    required List<ChatMessageMedia> media,
    required String clientMessageId,
  }) => ChatMessage(
    uuid: '',
    conversationUuid: conversationUuid,
    senderPersonUuid: senderPersonUuid,
    senderKind: 'Person',
    senderDisplayName: senderDisplayName,
    body: body,
    media: media,
    clientMessageId: clientMessageId,
    sentAt: DateTime.now(),
    pending: true,
  );

  ChatMessage copyWith({bool? pending, bool? failed}) => ChatMessage(
    uuid: uuid,
    conversationUuid: conversationUuid,
    senderPersonUuid: senderPersonUuid,
    senderKind: senderKind,
    senderDisplayName: senderDisplayName,
    senderAvatarUrl: senderAvatarUrl,
    senderBusinessProfileUuid: senderBusinessProfileUuid,
    body: body,
    media: media,
    clientMessageId: clientMessageId,
    sentAt: sentAt,
    pending: pending ?? this.pending,
    failed: failed ?? this.failed,
  );

  bool get isFromBusiness => senderKind == 'BusinessProfile';

  factory ChatMessage.fromJson(Map<String, dynamic> json) => ChatMessage(
    uuid: (json['uuid'] ?? '') as String,
    conversationUuid:
        (json['conversationUuid'] ?? json['conversation_uuid'] ?? '') as String,
    senderPersonUuid:
        (json['senderPersonUuid'] ?? json['sender_person_uuid'] ?? '') as String,
    senderKind: (json['senderKind'] ?? json['sender_kind'] ?? 'Person') as String,
    senderDisplayName:
        (json['senderDisplayName'] ?? json['sender_display_name'] ?? '')
            as String,
    senderAvatarUrl: json['senderAvatarUrl'] ?? json['sender_avatar_url'],
    senderBusinessProfileUuid:
        json['senderBusinessProfileUuid'] ?? json['sender_business_profile_uuid'],
    body: (json['body'] ?? '') as String,
    media: ((json['media'] as List<dynamic>?) ?? const [])
        .whereType<Map<String, dynamic>>()
        .map(ChatMessageMedia.fromJson)
        .toList(),
    clientMessageId:
        (json['clientMessageId'] ?? json['client_message_id'] ?? '') as String,
    sentAt: _parseDate(json['sentAt'] ?? json['sent_at']),
  );

  static DateTime _parseDate(dynamic value) {
    if (value is String && value.isNotEmpty) {
      try {
        return DateTime.parse(value);
      } catch (_) {}
    }
    return DateTime.now();
  }
}
