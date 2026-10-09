import 'dart:convert';

class JwtClaims {
  final String? profileType;
  final int? activeBusinessProfileId;
  final String? activeBusinessProfileUuid;
  final DateTime? expiresAt;

  const JwtClaims({
    this.profileType,
    this.activeBusinessProfileId,
    this.activeBusinessProfileUuid,
    this.expiresAt,
  });

  bool get isBusinessProfile =>
      activeBusinessProfileUuid != null && activeBusinessProfileUuid!.isNotEmpty;

  /// True when the token carries no `exp` or is within [skew] of expiring.
  bool isExpired({Duration skew = const Duration(seconds: 30)}) =>
      expiresAt == null || !DateTime.now().add(skew).isBefore(expiresAt!);
}

class JwtDecoder {
  JwtDecoder._();

  static JwtClaims? decode(String token) {
    try {
      final parts = token.split('.');
      if (parts.length != 3) return null;

      final normalized = base64Url.normalize(parts[1]);
      final payload = utf8.decode(base64Url.decode(normalized));
      final data = jsonDecode(payload) as Map<String, dynamic>;

      return JwtClaims(
        profileType: data['profileType'] as String?,
        activeBusinessProfileId: data['activeBusinessProfileId'] as int?,
        activeBusinessProfileUuid: data['activeBusinessProfileUuid'] as String?,
        expiresAt: data['exp'] is num ? DateTime.fromMillisecondsSinceEpoch((data['exp'] as num).toInt() * 1000) : null,
      );
    } catch (_) {
      return null;
    }
  }
}
