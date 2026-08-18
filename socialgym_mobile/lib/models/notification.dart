class Notification {
  final String uuid;
  final String notificationType;
  final String recipientPersonUuid;
  final String actorPersonUuid;
  final String actorName;
  final String postUuid;
  final String entityType;
  final String snippet;
  final bool read;
  final DateTime createdAt;
  final DateTime updatedAt;

  const Notification({
    required this.uuid,
    required this.notificationType,
    required this.recipientPersonUuid,
    required this.actorPersonUuid,
    required this.actorName,
    required this.postUuid,
    required this.entityType,
    required this.snippet,
    required this.read,
    required this.createdAt,
    required this.updatedAt,
  });

  factory Notification.fromJson(Map<String, dynamic> json) => Notification(
    uuid: _readString(json, ['uuid', 'idempotencyKey', 'idempotency_key']),
    notificationType: _readString(json, [
      'notificationType',
      'notification_type',
    ]),
    recipientPersonUuid: _readString(json, [
      'recipientPersonUuid',
      'recipient_person_uuid',
    ]),
    actorPersonUuid: _readString(json, [
      'actorPersonUuid',
      'actor_person_uuid',
    ]),
    actorName: _readString(json, ['actorName', 'actor_name']),
    postUuid: _readString(json, ['postUuid', 'post_uuid']),
    entityType: _readString(json, ['entityType', 'entity_type']),
    snippet: _readString(json, ['snippet']),
    read: _parseBool(json['read'] ?? json['isRead'] ?? json['is_read']),
    createdAt: _parseDate(json['createdAt'] ?? json['created_at']),
    updatedAt: _parseDate(json['updatedAt'] ?? json['updated_at']),
  );

  Map<String, dynamic> toJson() => {
    'uuid': uuid,
    'notificationType': notificationType,
    'recipientPersonUuid': recipientPersonUuid,
    'actorPersonUuid': actorPersonUuid,
    'actorName': actorName,
    'postUuid': postUuid,
    'entityType': entityType,
    'snippet': snippet,
    'read': read,
    'createdAt': createdAt.toIso8601String(),
    'updatedAt': updatedAt.toIso8601String(),
  };

  Notification copyWith({
    String? uuid,
    String? notificationType,
    String? recipientPersonUuid,
    String? actorPersonUuid,
    String? actorName,
    String? postUuid,
    String? entityType,
    String? snippet,
    bool? read,
    DateTime? createdAt,
    DateTime? updatedAt,
  }) => Notification(
    uuid: uuid ?? this.uuid,
    notificationType: notificationType ?? this.notificationType,
    recipientPersonUuid: recipientPersonUuid ?? this.recipientPersonUuid,
    actorPersonUuid: actorPersonUuid ?? this.actorPersonUuid,
    actorName: actorName ?? this.actorName,
    postUuid: postUuid ?? this.postUuid,
    entityType: entityType ?? this.entityType,
    snippet: snippet ?? this.snippet,
    read: read ?? this.read,
    createdAt: createdAt ?? this.createdAt,
    updatedAt: updatedAt ?? this.updatedAt,
  );

  bool get isUnread => !read;

  static DateTime _parseDate(dynamic value) {
    if (value is String && value.isNotEmpty) {
      try {
        return DateTime.parse(value);
      } catch (_) {}
    }
    return DateTime.now();
  }

  static bool _parseBool(dynamic value) {
    if (value is bool) return value;
    if (value is String) {
      return value.toLowerCase() == 'true' || value == '1';
    }
    if (value is num) return value != 0;
    return false;
  }

  static String _readString(Map<String, dynamic> json, List<String> keys) {
    for (final key in keys) {
      final value = json[key];
      if (value is String) return value;
    }
    return '';
  }
}
