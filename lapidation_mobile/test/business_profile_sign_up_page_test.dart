import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:lapidation_mobile/l10n/app_localizations.dart';
import 'package:lapidation_mobile/pages/business_profile/business_profile_sign_up_page.dart';
import 'package:lapidation_mobile/providers/auth_provider.dart';
import 'package:lapidation_mobile/providers/business_profile_provider.dart';
import 'package:lapidation_mobile/providers/person_provider.dart';

void main() {
  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  testWidgets('shows validation errors when required fields are empty', (tester) async {
    await tester.pumpWidget(
      MultiProvider(
        providers: [
          ChangeNotifierProvider(create: (_) => AuthProvider()),
          ChangeNotifierProvider(create: (_) => PersonProvider()),
          ChangeNotifierProvider(create: (_) => BusinessProfileProvider()),
        ],
        child: MaterialApp(
          localizationsDelegates: AppLocalizations.localizationsDelegates,
          supportedLocales: AppLocalizations.supportedLocales,
          home: Builder(
            builder: (context) {
              WidgetsBinding.instance.addPostFrameCallback((_) {
                Navigator.of(context).pushNamed('/add-profile/form', arguments: 'Professional');
              });
              return const Scaffold(body: SizedBox.shrink());
            },
          ),
          onGenerateRoute: (settings) => MaterialPageRoute(
            settings: settings,
            builder: (_) => const BusinessProfileSignUpPage(),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();

    await tester.tap(find.widgetWithText(ElevatedButton, 'Create profile'));
    await tester.pumpAndSettle();

    expect(find.text('Please enter a business name'), findsOneWidget);
    expect(find.text('Please enter a social name'), findsOneWidget);
    expect(find.text('Please enter a tax ID'), findsOneWidget);
  });
}
