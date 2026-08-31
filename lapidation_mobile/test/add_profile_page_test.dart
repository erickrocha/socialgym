import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:lapidation_mobile/l10n/app_localizations.dart';
import 'package:lapidation_mobile/pages/business_profile/add_profile_page.dart';
import 'package:lapidation_mobile/providers/auth_provider.dart';
import 'package:lapidation_mobile/providers/locale_provider.dart';
import 'package:lapidation_mobile/providers/notifications_provider.dart';
import 'package:lapidation_mobile/providers/person_provider.dart';
import 'package:lapidation_mobile/providers/settings_provider.dart';
import 'package:shared_preferences/shared_preferences.dart';

class _RecordingObserver extends NavigatorObserver {
  final List<Route<dynamic>> pushed = [];

  @override
  void didPush(Route<dynamic> route, Route<dynamic>? previousRoute) {
    pushed.add(route);
  }
}

void main() {
  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  testWidgets(
    'renders both Personal Trainer and Gym cards and navigates with the right businessType',
    (tester) async {
      final observer = _RecordingObserver();

      await tester.pumpWidget(
        MultiProvider(
          providers: [
            ChangeNotifierProvider(create: (_) => AuthProvider()),
            ChangeNotifierProvider(create: (_) => LocaleProvider()),
            ChangeNotifierProvider(create: (_) => PersonProvider()),
            ChangeNotifierProvider(create: (_) => SettingsProvider()),
            ChangeNotifierProvider(create: (_) => NotificationsProvider()),
          ],
          child: MaterialApp(
            localizationsDelegates: AppLocalizations.localizationsDelegates,
            supportedLocales: AppLocalizations.supportedLocales,
            navigatorObservers: [observer],
            home: const AddProfilePage(),
            routes: {
              '/add-profile/form': (context) =>
                  const Scaffold(body: Text('form')),
            },
          ),
        ),
      );
      await tester.pumpAndSettle();

      expect(find.text('Personal Trainer'), findsOneWidget);
      expect(find.text('Gym'), findsOneWidget);

      await tester.tap(
        find.byKey(const ValueKey('business_profile_type_card_Professional')),
      );
      await tester.pumpAndSettle();

      final pushedRoute = observer.pushed.last;
      expect(pushedRoute.settings.name, '/add-profile/form');
      expect(pushedRoute.settings.arguments, 'Professional');
    },
  );
}
