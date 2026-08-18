import 'dart:convert';

class JwtClaims {
  final String? profileType;
  final int? activeBusinessProfileId;
  final String? activeBusinessProfileUuid;

  const JwtClaims({
    this.profileType,
    this.activeBusinessProfileId,
    this.activeBusinessProfileUuid,
  });

  bool get isBusinessProfile =>
      activeBusinessProfileUuid != null && activeBusinessProfileUuid!.isNotEmpty;
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
      );
    } catch (_) {
      return null;
    }
  }
}
