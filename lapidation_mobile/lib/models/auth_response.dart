class PendingAccountDeletion {
  final DateTime requestedAt;
  final DateTime scheduledAt;

  PendingAccountDeletion({
    required this.requestedAt,
    required this.scheduledAt,
  });

  factory PendingAccountDeletion.fromJson(Map<String, dynamic> json) {
    return PendingAccountDeletion(
      requestedAt: DateTime.parse(json['requestedAt'] as String),
      scheduledAt: DateTime.parse(json['scheduledAt'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'requestedAt': requestedAt.toIso8601String(),
      'scheduledAt': scheduledAt.toIso8601String(),
    };
  }
}

class AuthResponse {
  final String accessToken;
  final String tokenType;
  final int expireIn;
  final String? refreshToken;
  final String username;
  final String uuid;
  final String name;
  final int personId;
  final String personAvatar;
  final String? personUuid;
  final String? personObjectKey;
  final int? activeBusinessProfileId;
  final String? activeBusinessProfileUuid;
  final PendingAccountDeletion? pendingAccountDeletion;

  AuthResponse({
    required this.accessToken,
    required this.tokenType,
    required this.expireIn,
    this.refreshToken,
    required this.username,
    required this.uuid,
    required this.name,
    required this.personId,
    required this.personAvatar,
    this.personUuid,
    this.personObjectKey,
    this.activeBusinessProfileId,
    this.activeBusinessProfileUuid,
    this.pendingAccountDeletion,
  });

  factory AuthResponse.fromJson(Map<String, dynamic> json) {
    return AuthResponse(
      accessToken: json['accessToken'] ?? '',
      tokenType: json['tokenType'] ?? 'Bearer',
      expireIn: json['expireIn'] ?? 0,
      refreshToken: json['refreshToken'],
      username: json['username'] ?? '',
      uuid: json['uuid'] ?? '',
      name: json['name'] ?? '',
      personId: json['personId'] ?? 0,
      personAvatar: json['personAvatar'] ?? '',
      personUuid: json['personUuid'],
      personObjectKey: json['personObjectKey'],
      activeBusinessProfileId: json['activeBusinessProfileId'],
      activeBusinessProfileUuid: json['activeBusinessProfileUuid'],
      pendingAccountDeletion: json['pendingAccountDeletion'] != null
          ? PendingAccountDeletion.fromJson(
              json['pendingAccountDeletion'] as Map<String, dynamic>,
            )
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'accessToken': accessToken,
      'tokenType': tokenType,
      'expireIn': expireIn,
      'refreshToken': refreshToken,
      'username': username,
      'uuid': uuid,
      'name': name,
      'personId': personId,
      'personAvatar': personAvatar,
      'personUuid': personUuid,
      'personObjectKey': personObjectKey,
      'activeBusinessProfileId': activeBusinessProfileId,
      'activeBusinessProfileUuid': activeBusinessProfileUuid,
      'pendingAccountDeletion': pendingAccountDeletion?.toJson(),
    };
  }

  AuthResponse copyWith({String? refreshToken}) {
    return AuthResponse(
      accessToken: accessToken,
      tokenType: tokenType,
      expireIn: expireIn,
      refreshToken: refreshToken ?? this.refreshToken,
      username: username,
      uuid: uuid,
      name: name,
      personId: personId,
      personAvatar: personAvatar,
      personUuid: personUuid,
      personObjectKey: personObjectKey,
      activeBusinessProfileId: activeBusinessProfileId,
      activeBusinessProfileUuid: activeBusinessProfileUuid,
      pendingAccountDeletion: pendingAccountDeletion,
    );
  }

  /// Returns a copy with [pendingAccountDeletion] cleared, used after a
  /// successful cancel-deletion call.
  AuthResponse withoutPendingAccountDeletion() {
    return AuthResponse(
      accessToken: accessToken,
      tokenType: tokenType,
      expireIn: expireIn,
      refreshToken: refreshToken,
      username: username,
      uuid: uuid,
      name: name,
      personId: personId,
      personAvatar: personAvatar,
      personUuid: personUuid,
      personObjectKey: personObjectKey,
      activeBusinessProfileId: activeBusinessProfileId,
      activeBusinessProfileUuid: activeBusinessProfileUuid,
      pendingAccountDeletion: null,
    );
  }
}
