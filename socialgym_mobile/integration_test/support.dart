import 'dart:async';
import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:socialgym_mobile/models/auth_response.dart';
import 'package:socialgym_mobile/models/sign_up_request.dart';
import 'package:socialgym_mobile/services/grpc/grpc_auth_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_channel_factory.dart';
import 'package:socialgym_mobile/services/grpc/grpc_consent_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_friend_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_legal_document_service.dart';
import 'package:socialgym_mobile/services/grpc/grpc_person_service.dart';

/// A person signed up on the test stack. The app reads the signed-in person from the stored
/// `auth`, so acting as someone means storing their sign-in first ([as]).
class Account {
  Account(this.auth, this.personUuid, {this.email = '', this.password = ''});
  final AuthResponse auth;
  final String personUuid;
  final String email;
  final String password;

  int get personId => auth.personId;
}

Future<void> startApp() async {
  await GrpcChannelFactory.initialize(certAssetPath: 'assets/certs/server.crt');
}

/// Makes [auth] the signed-in session, as the app does after sign-in or a profile switch.
Future<void> storeSession(AuthResponse auth) => _store(auth);

Future<void> _store(AuthResponse auth) async {
  final preferences = await SharedPreferences.getInstance();
  await preferences.setString('auth', jsonEncode(auth.toJson()));
}

/// Signs a new person up through the app's own sign-up call and leaves them signed in.
Future<Account> register(String name) async {
  final stamp = DateTime.now().microsecondsSinceEpoch;
  final terms = await GrpcLegalDocumentService.get('terms');
  final privacy = await GrpcLegalDocumentService.get('privacy');
  final email = '$name-$stamp@e2e.test';
  final password = 'Passw0rd!$stamp';
  final auth = await GrpcAuthService.signUp(
    SignUpRequest(
      firstname: name,
      surname: 'E2E',
      email: email,
      password: password,
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
  await GrpcConsentService.accept('health_data');
  return Account(auth, me.uuid, email: email, password: password);
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
