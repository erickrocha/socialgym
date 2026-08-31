// This is a basic Flutter widget test.
//
// To perform an interaction with a widget in your test, use the WidgetTester
// utility in the flutter_test package. For example, you can send tap and scroll
// gestures. You can also use WidgetTester to find child widgets in the widget
// tree, read text, and verify that the values of widget properties are correct.

import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'package:lapidation_mobile/main.dart';

void main() {
  testWidgets('App renders sign in page', (WidgetTester tester) async {
    SharedPreferences.setMockInitialValues({});
    await tester.pumpWidget(const LapidationApp());
    await tester.pumpAndSettle();

    // Verify that the sign in page is displayed
    expect(find.text('Sign In'), findsOneWidget);
    expect(find.text('Create new account'), findsOneWidget);
  });
}
