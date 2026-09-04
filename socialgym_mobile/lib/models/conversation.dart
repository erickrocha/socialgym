class ConversationParticipant {
  final String personUuid;
  final String role;
  final DateTime? lastReadAt;
  final String? lastReadMessageUuid;

  const ConversationParticipant({
    required this.personUuid,
    required this.role,
    this.lastReadAt,
    this.lastReadMessageUuid,
  });

  factory ConversationParticipant.fromJson(Map<String, dynamic> json) =>
      ConversationParticipant(
        personUuid: (json['personUuid'] ?? json['person_uuid'] ?? '') as String,
        role: (json['role'] ?? 'member') as String,
        lastReadAt: _parseDateOrNull(json['lastReadAt'] ?? json['last_read_at']),
        lastReadMessageUuid:
            json['lastReadMessageUuid'] ?? json['last_read_message_uuid'],
      );

  static DateTime? _parseDateOrNull(dynamic value) {
    if (value is String && value.isNotEmpty) {
      try {
        return DateTime.parse(value);
      } catch (_) {}
    }
    return null;
  }
}

class LastMessagePreview {
  final String messageUuid;
  final String senderPersonUuid;
  final String senderDisplayName;
  final String snippet;
  final DateTime sentAt;
  final bool hasMedia;

  const LastMessagePreview({
    required this.messageUuid,
    required this.senderPersonUuid,
    required this.senderDisplayName,
    required this.snippet,
    required this.sentAt,
    required this.hasMedia,
  });

  factory LastMessagePreview.fromJson(Map<String, dynamic> json) =>
      LastMessagePreview(
        messageUuid:
            (json['messageUuid'] ?? json['message_uuid'] ?? '') as String,
        senderPersonUuid:
            (json['senderPersonUuid'] ?? json['sender_person_uuid'] ?? '')
                as String,
        senderDisplayName:
            (json['senderDisplayName'] ?? json['sender_display_name'] ?? '')
                as String,
        snippet: (json['snippet'] ?? '') as String,
        sentAt: _parseDate(json['sentAt'] ?? json['sent_at']),
        hasMedia: (json['hasMedia'] ?? json['has_media'] ?? false) as bool,
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

class Conversation {
  final String uuid;
  final String conversationType;
  final String? businessProfileUuid;
  final String? businessProfileName;
  final String? businessProfileLogoUrl;
  final List<String> participantPersonUuids;
  final List<ConversationParticipant> participants;
  final LastMessagePreview? lastMessage;
  final bool unread;
  final DateTime updatedAt;

  const Conversation({
    required this.uuid,
    required this.conversationType,
    this.businessProfileUuid,
    this.businessProfileName,
    this.businessProfileLogoUrl,
    required this.participantPersonUuids,
    required this.participants,
    this.lastMessage,
    required this.unread,
    required this.updatedAt,
  });

  bool get isGroup => conversationType == 'BusinessTeamGroup';
  bool get isDirect => conversationType == 'DirectPerson';

  /// The other person in a direct conversation, or null for group/business
  /// conversations. Mirrors the backend's `other_direct_participant`
  /// (timeline/business/src/use_cases/chat_use_case.rs).
  String? counterpartUuidFor(String myPersonUuid) {
    if (!isDirect) return null;
    for (final uuid in participantPersonUuids) {
      if (uuid != myPersonUuid) return uuid;
    }
    return null;
  }

  factory Conversation.fromJson(Map<String, dynamic> json) => Conversation(
    uuid: (json['uuid'] ?? '') as String,
    conversationType:
        (json['conversationType'] ?? json['conversation_type'] ?? '') as String,
    businessProfileUuid:
        json['businessProfileUuid'] ?? json['business_profile_uuid'],
    businessProfileName:
        json['businessProfileName'] ?? json['business_profile_name'],
    businessProfileLogoUrl:
        json['businessProfileLogoUrl'] ?? json['business_profile_logo_url'],
    participantPersonUuids:
        ((json['participantPersonUuids'] ?? json['participant_person_uuids'])
                    as List<dynamic>? ??
                const [])
            .map((e) => e.toString())
            .toList(),
    participants:
        ((json['participants'] as List<dynamic>?) ?? const [])
            .whereType<Map<String, dynamic>>()
            .map(ConversationParticipant.fromJson)
            .toList(),
    lastMessage: (json['lastMessage'] ?? json['last_message']) is Map
        ? LastMessagePreview.fromJson(
            (json['lastMessage'] ?? json['last_message']) as Map<String, dynamic>,
          )
        : null,
    unread: (json['unread'] ?? false) as bool,
    updatedAt: _parseDate(json['updatedAt'] ?? json['updated_at']),
  );

  Conversation copyWith({
    LastMessagePreview? lastMessage,
    bool? unread,
    DateTime? updatedAt,
  }) => Conversation(
    uuid: uuid,
    conversationType: conversationType,
    businessProfileUuid: businessProfileUuid,
    businessProfileName: businessProfileName,
    businessProfileLogoUrl: businessProfileLogoUrl,
    participantPersonUuids: participantPersonUuids,
    participants: participants,
    lastMessage: lastMessage ?? this.lastMessage,
    unread: unread ?? this.unread,
    updatedAt: updatedAt ?? this.updatedAt,
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
