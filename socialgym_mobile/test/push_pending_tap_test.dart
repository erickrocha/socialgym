import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/models/auth_response.dart';
import 'package:socialgym_mobile/services/push_registration_service.dart';

AuthResponse _auth() => AuthResponse(
  accessToken: 'token',
  tokenType: 'Bearer',
  expireIn: 0,
  username: 'alice@example.test',
  uuid: 'user-1',
  name: 'Alice',
  personId: 1,
  personAvatar: '',
);

/// Every route renders its name and arguments so the test can tell them apart.
Widget _app(GlobalKey<NavigatorState> key) => MaterialApp(
  navigatorKey: key,
  initialRoute: '/', // plays the sign-in page
  onGenerateRoute: (settings) => MaterialPageRoute<void>(
    settings: settings,
    builder: (_) => Scaffold(body: Text('page:${settings.name}:${settings.arguments}')),
  ),
);

void main() {
  late GlobalKey<NavigatorState> key;

  setUp(() async {
    PushRegistrationService.resetForTest();
    key = GlobalKey<NavigatorState>();
    // No Firebase options in tests: this only records the navigator key.
    await PushRegistrationService.initialize(key);
  });

  testWidgets('a tap received while signed out opens its target after sign-in navigates home', (
    tester,
  ) async {
    await tester.pumpWidget(_app(key));
    expect(find.text('page:/:null'), findsOneWidget);

    PushRegistrationService.handleTapForTest({
      'targetType': 'friendship_request',
      'targetUuid': 'request-1',
    });
    await tester.pumpAndSettle();
    expect(find.text('page:/:null'), findsOneWidget, reason: 'nothing opens while signed out');

    // AuthProvider.login binds the user first. It must not push the target yet,
    // because the sign-in page replaces the top route right afterwards.
    await PushRegistrationService.bindAuthenticatedUser(_auth());
    await tester.pumpAndSettle();
    expect(find.text('page:/:null'), findsOneWidget);
    expect(find.textContaining('page:/friends'), findsNothing);

    // What SignInPage does after login.
    key.currentState!.pushReplacementNamed('/feed');
    PushRegistrationService.openPendingTap();
    await tester.pumpAndSettle();

    expect(find.textContaining('page:/friends:{friendshipUuid: request-1}'), findsOneWidget);
    key.currentState!.pop();
    await tester.pumpAndSettle();
    expect(find.textContaining('page:/feed'), findsOneWidget, reason: 'the target sits on top of home');
    expect(key.currentState!.canPop(), isFalse, reason: 'sign-in was replaced, so Back leaves the app');
  });

  testWidgets('opening the pending tap twice navigates only once', (tester) async {
    await tester.pumpWidget(_app(key));
    PushRegistrationService.handleTapForTest({'targetType': 'post', 'targetUuid': 'post-1'});
    await PushRegistrationService.bindAuthenticatedUser(_auth());
    key.currentState!.pushReplacementNamed('/feed');

    PushRegistrationService.openPendingTap();
    PushRegistrationService.openPendingTap();
    await tester.pumpAndSettle();

    expect(find.textContaining('page:/feed:{postUuid: post-1}'), findsOneWidget);
    key.currentState!.pop();
    await tester.pumpAndSettle();
    expect(find.textContaining('page:/feed:null'), findsOneWidget);
    expect(key.currentState!.canPop(), isFalse);
  });

  testWidgets('a tap while already signed in opens immediately', (tester) async {
    await tester.pumpWidget(_app(key));
    await PushRegistrationService.bindAuthenticatedUser(_auth());

    PushRegistrationService.handleTapForTest({'targetType': 'post', 'targetUuid': 'post-9'});
    await tester.pumpAndSettle();

    expect(find.textContaining('page:/feed:{postUuid: post-9}'), findsOneWidget);
  });

  testWidgets('an unknown target falls back to the notifications page', (tester) async {
    await tester.pumpWidget(_app(key));
    await PushRegistrationService.bindAuthenticatedUser(_auth());

    PushRegistrationService.handleTapForTest({'targetType': 'mystery', 'targetUuid': 'x'});
    await tester.pumpAndSettle();

    expect(find.textContaining('page:/notifications'), findsOneWidget);
  });
}
