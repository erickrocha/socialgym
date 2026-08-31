import 'dart:convert';

import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/utils/jwt_decoder.dart';

String _fakeToken(Map<String, dynamic> payload) {
  final header = base64Url
      .encode(utf8.encode(jsonEncode({'alg': 'none'})))
      .replaceAll('=', '');
  final body = base64Url
      .encode(utf8.encode(jsonEncode(payload)))
      .replaceAll('=', '');
  return '$header.$body.signature';
}

void main() {
  group('JwtDecoder.decode', () {
    test('decodes profileType and active business profile claims', () {
      final token = _fakeToken({
        'profileType': 'Professional',
        'activeBusinessProfileId': 42,
        'activeBusinessProfileUuid': 'bp-uuid-1',
      });

      final claims = JwtDecoder.decode(token);

      expect(claims, isNotNull);
      expect(claims!.profileType, 'Professional');
      expect(claims.activeBusinessProfileId, 42);
      expect(claims.activeBusinessProfileUuid, 'bp-uuid-1');
      expect(claims.isBusinessProfile, isTrue);
    });

    test('missing claims degrade to personal mode', () {
      final token = _fakeToken({'sub': 'someone'});

      final claims = JwtDecoder.decode(token);

      expect(claims, isNotNull);
      expect(claims!.activeBusinessProfileUuid, isNull);
      expect(claims.isBusinessProfile, isFalse);
    });

    test('malformed token returns null', () {
      expect(JwtDecoder.decode('not-a-jwt'), isNull);
      expect(JwtDecoder.decode(''), isNull);
    });
  });
}
