
import 'package:lapidation_mobile/models/enums.dart';

class Settings {
  final int? id;
  final String? uuid;
  final int? ownerId;
  final String? ownerUuid;
  final String? language;
  final String? theme;
  final bool? notificationsEnabled;
  final ContextMenuPosition? contextMenuPosition;
  final Pages? homePage;
  final DateTime createdAt;
  final DateTime updatedAt;

  Settings({
    this.id = 0,
    this.uuid = '',
    this.ownerId = 0,
    this.ownerUuid = '',
    this.language = 'en',
    this.theme = "default",
    this.notificationsEnabled = true,
    this.contextMenuPosition = ContextMenuPosition.left,
    this.homePage = Pages.feed,
    DateTime? createdAt,
    DateTime? updatedAt,
  }) : createdAt = createdAt ?? DateTime.now(),
        updatedAt = updatedAt ?? DateTime.now();

  factory Settings.fromJson(Map<String, dynamic> json) {
    return Settings(
      id: json['id'] as int?,
      uuid: json['uuid'] as String?,
      ownerId: json['personId'] as int?,
      ownerUuid: json['personUuid'] as String?,
      language: json['language'] as String?,
      theme: json['theme'] as String?,
      notificationsEnabled: json['notificationsEnabled'] as bool?,
      homePage: Pages.fromString(json['homePage'] as String?),
      contextMenuPosition: ContextMenuPosition.fromString(json['contextMenuPosition'] as String?),
      createdAt: json['createdAt'] != null ? DateTime.parse(json['createdAt'] as String) : null,
      updatedAt: json['updatedAt'] != null ? DateTime.parse(json['updatedAt'] as String) : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'uuid': uuid,
      'personId': ownerId,
      'personUuid': ownerUuid,
      'language': language,
      'theme': theme,
      'notificationsEnabled': notificationsEnabled,
      'contextMenuPosition': contextMenuPosition?.toStringValue(),
      'homePage': homePage?.toStringValue(),
      'createdAt': createdAt.toIso8601String(),
      'updatedAt': updatedAt.toIso8601String(),
    };
  }

  Settings copyWith({
    int? id,
    String? uuid,
    int? personId,
    String? personUuid,
    String? language,
    String? theme,
    bool? notificationsEnabled,
    Pages? homePage,
    ContextMenuPosition? contextMenuPosition,
  }) {
    return Settings(
      id: id ?? this.id,
      uuid: uuid ?? this.uuid,
      ownerId: personId ?? this.ownerId,
      ownerUuid: personUuid ?? this.ownerUuid,
      language: language ?? this.language,
      theme: theme ?? this.theme,
      notificationsEnabled: notificationsEnabled ?? this.notificationsEnabled,
      homePage: homePage ?? this.homePage,
      contextMenuPosition: contextMenuPosition ?? this.contextMenuPosition,
      createdAt: createdAt,
      updatedAt: updatedAt,
    );
  }
}