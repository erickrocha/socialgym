import 'dart:async';
import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:socialgym_mobile/models/auth_response.dart';
import 'package:socialgym_mobile/models/sign_up_request.dart';
import 'package:socialgym_mobile/services/auth_service.dart';
import 'package:socialgym_mobile/services/consent_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_channel_factory.dart';
import 'package:socialgym_mobile/services/grpc/grpc_friend_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_person_service.dart';
import 'package:socialgym_mobile/services/legal_document_service.dart';
import 'package:socialgym_mobile/utils/dio_client.dart';

/// A person signed up on the test stack. The app reads the signed-in person from the stored
/// `auth`, so acting as someone means storing their sign-in first ([as]).
class Account {
  Account(this.auth, this.personUuid);
  final AuthResponse auth;
  final String personUuid;

  int get personId => auth.personId;
}

Future<void> startApp() async {
  await GrpcChannelFactory.initialize(certAssetPath: 'assets/certs/server.crt');
}

Future<void> _store(AuthResponse auth) async {
  final preferences = await SharedPreferences.getInstance();
  await preferences.setString('auth', jsonEncode(auth.toJson()));
  DioClient().setAuthToken(auth.accessToken);
}

/// Signs a new person up through the app's own sign-up call and leaves them signed in.
Future<Account> register(String name) async {
  final stamp = DateTime.now().microsecondsSinceEpoch;
  final terms = await LegalDocumentService.get('terms');
  final privacy = await LegalDocumentService.get('privacy');
  final auth = await AuthService.signUp(
    SignUpRequest(
      firstname: name,
      surname: 'E2E',
      email: '$name-$stamp@e2e.test',
      password: 'Passw0rd!$stamp',
      dateOfBirth: '1990-01-01',
      gender: 'MALE',
      termsVersion: terms.version,
      privacyVersion: privacy.version,
      termsAccepted: true,
      privacyAccepted: true,
    ),
  );
  await _store(auth);
  final me = await GrpcPersonService.getMe();
  await ConsentService.accept('health_data');
  return Account(auth, me.uuid);
}

/// Runs [body] as [account] and goes back to whoever was signed in.
Future<T> asUser<T>(Account account, Future<T> Function() body, {required Account back}) async {
  await _store(account.auth);
  try {
    return await body();
  } finally {
    await _store(back.auth);
  }
}

Future<void> befriend(Account a, Account b) async {
  await asUser(a, () => GrpcFriendService.sendFriendRequest(personId: b.personId), back: a);
  await asUser(b, () => GrpcFriendService.acceptFriendRequest(personId: a.personId), back: a);
}

/// Polls until [condition] holds or [timeout] passes.
Future<bool> eventually(
  FutureOr<bool> Function() condition, {
  Duration timeout = const Duration(seconds: 10),
  Duration every = const Duration(milliseconds: 200),
}) async {
  final deadline = DateTime.now().add(timeout);
  while (DateTime.now().isBefore(deadline)) {
    if (await condition()) return true;
    await Future<void>.delayed(every);
  }
  return false;
}

/// A line the host script watches in the device log to trigger an outside action.
void marker(String text) => debugPrint('E2E_MARKER $text');
