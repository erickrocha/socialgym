import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:lapidation_mobile/l10n/app_localizations.dart';
import 'package:lapidation_mobile/models/person.dart';
import 'package:lapidation_mobile/pages/profile/profile_page.dart';
import 'package:lapidation_mobile/providers/auth_provider.dart';
import 'package:lapidation_mobile/providers/locale_provider.dart';
import 'package:lapidation_mobile/providers/notifications_provider.dart';
import 'package:lapidation_mobile/providers/person_provider.dart';
import 'package:lapidation_mobile/providers/resource_provider.dart';
import 'package:lapidation_mobile/providers/settings_provider.dart';

void main() {
  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  testWidgets('ignores empty dropdown values from persisted profile data', (
    tester,
  ) async {
    await tester.binding.setSurfaceSize(const Size(1400, 1200));
    addTearDown(() => tester.binding.setSurfaceSize(null));

    final personProvider = PersonProvider()
      ..setPersonForTest(
        Person(
          id: 1,
          uuid: 'person-1',
          firstname: 'Test',
          surname: 'Person',
          gender: '',
          personInfo: PersonInfo(
            id: 1,
            uuid: 'person-info-1',
            personId: 1,
            relationship: '',
          ),
        ),
      );

    await tester.pumpWidget(
      MultiProvider(
        providers: [
          ChangeNotifierProvider<PersonProvider>.value(value: personProvider),
          ChangeNotifierProvider(create: (_) => AuthProvider()),
          ChangeNotifierProvider(create: (_) => LocaleProvider()),
          ChangeNotifierProvider(create: (_) => NotificationsProvider()),
          ChangeNotifierProvider(create: (_) => ResourceProvider()),
          ChangeNotifierProvider(create: (_) => SettingsProvider()),
        ],
        child: const MaterialApp(
          localizationsDelegates: AppLocalizations.localizationsDelegates,
          supportedLocales: AppLocalizations.supportedLocales,
          home: ProfilePage(),
        ),
      ),
    );
    await tester.pump();
    await tester.tap(find.byTooltip('Edit Profile'));
    await tester.pump();

    expect(tester.takeException(), isNull);
    expect(find.byType(DropdownButtonFormField<String>), findsNWidgets(2));
  });
}
