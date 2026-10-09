import 'dart:convert';

import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:socialgym_mobile/models/business_profile.dart';
import 'package:socialgym_mobile/models/person.dart';
import 'package:socialgym_mobile/providers/auth_provider.dart';
import 'package:socialgym_mobile/providers/person_provider.dart';
import 'package:socialgym_mobile/utils/jwt_decoder.dart';

String _jwt(Map<String, dynamic> claims) {
  String enc(Object o) => base64Url.encode(utf8.encode(jsonEncode(o)));
  return '${enc({'alg': 'none'})}.${enc(claims)}.sig';
}

Map<String, dynamic> _auth(Map<String, dynamic> claims) => {
  'accessToken': _jwt(claims),
  'tokenType': 'Bearer',
  'expireIn': 600,
  'username': 'u',
  'uuid': 'u',
  'name': 'n',
  'personId': 1,
  'personAvatar': '',
};

int _inSeconds(int s) => DateTime.now().add(Duration(seconds: s)).millisecondsSinceEpoch ~/ 1000;

BusinessProfile _profile({String? logo}) => BusinessProfile(
  uuid: 'biz-1',
  ownerId: 1,
  ownerUuid: 'p-1',
  taxId: '1',
  businessName: 'Gym',
  businessType: 'Company',
  logo: logo,
);

void main() {
  setUp(() => SharedPreferences.setMockInitialValues({}));

  test('syncBusinessProfile updates list and active profile', () async {
    final provider = PersonProvider();
    provider.setPersonForTest(
      Person(id: 1, uuid: 'p-1', firstname: 'A', surname: 'B', businessProfiles: [_profile()]),
    );
    provider.setActiveBusinessProfile(_profile());

    await provider.syncBusinessProfile(_profile(logo: 'https://x/logo.png'));

    expect(provider.activeBusinessProfile?.logo, 'https://x/logo.png');
    expect(provider.person!.businessProfiles.single.logo, 'https://x/logo.png');
  });

  test('JwtClaims.isExpired: future valid, past and missing exp expired', () {
    expect(JwtDecoder.decode(_jwt({'exp': _inSeconds(600)}))!.isExpired(), isFalse);
    expect(JwtDecoder.decode(_jwt({'exp': _inSeconds(-5)}))!.isExpired(), isTrue);
    expect(JwtDecoder.decode(_jwt({}))!.isExpired(), isTrue);
  });

  test('restoreSession is false with no stored session', () async {
    final auth = AuthProvider();
    await Future<void>.delayed(Duration.zero);
    expect(await auth.restoreSession(), isFalse);
  });

  test('restoreSession keeps a still-valid token without refreshing', () async {
    SharedPreferences.setMockInitialValues({'auth': jsonEncode(_auth({'exp': _inSeconds(600)}))});
    final auth = AuthProvider();
    await Future<void>.delayed(const Duration(milliseconds: 50));
    expect(await auth.restoreSession(), isTrue);
  });

  test('restoreSession signs out when expired and no refresh token', () async {
    SharedPreferences.setMockInitialValues({'auth': jsonEncode(_auth({'exp': 1}))});
    final auth = AuthProvider();
    await Future<void>.delayed(const Duration(milliseconds: 50));
    expect(await auth.restoreSession(), isFalse);
    expect(auth.isAuthenticated, isFalse);
  });
}
