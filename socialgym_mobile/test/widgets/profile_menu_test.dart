import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:socialgym_mobile/models/person.dart';
import 'package:socialgym_mobile/models/business_profile.dart';
import 'package:socialgym_mobile/providers/person_provider.dart';
import 'package:socialgym_mobile/providers/auth_provider.dart';
import 'package:socialgym_mobile/widgets/profile_menu.dart';
import 'package:socialgym_mobile/l10n/app_localizations.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:shared_preferences/shared_preferences.dart';

void main() {
  group('ProfileMenu Display Tests', () {
    late PersonProvider personProvider;
    late AuthProvider authProvider;

    setUp(() {
      SharedPreferences.setMockInitialValues({});
      personProvider = PersonProvider();
      authProvider = AuthProvider();
    });

    Widget createTestWidget(Widget child) {
      return MultiProvider(
        providers: [
          ChangeNotifierProvider<PersonProvider>.value(value: personProvider),
          ChangeNotifierProvider<AuthProvider>.value(value: authProvider),
        ],
        child: MaterialApp(
          localizationsDelegates: const [
            AppLocalizations.delegate,
            GlobalMaterialLocalizations.delegate,
            GlobalWidgetsLocalizations.delegate,
            GlobalCupertinoLocalizations.delegate,
          ],
          supportedLocales: const [Locale('en')],
          home: Scaffold(body: child),
        ),
      );
    }

    testWidgets('uses the female avatar fallback in the top bar', (
      tester,
    ) async {
      personProvider.setPersonForTest(
        Person(
          id: 1,
          uuid: 'test-uuid',
          firstname: '',
          surname: '',
          gender: 'Female',
        ),
      );

      await tester.pumpWidget(createTestWidget(const ProfileMenu()));

      final image = tester.widget<Image>(find.byType(Image));
      expect(
        (image.image as AssetImage).assetName,
        'assets/images/avatar_female.png',
      );
      expect(find.text('?'), findsNothing);
    });

    testWidgets('uses the male avatar fallback in the top bar', (tester) async {
      personProvider.setPersonForTest(
        Person(
          id: 1,
          uuid: 'test-uuid',
          firstname: '',
          surname: '',
          gender: 'Male',
        ),
      );

      await tester.pumpWidget(createTestWidget(const ProfileMenu()));

      final image = tester.widget<Image>(find.byType(Image));
      expect(
        (image.image as AssetImage).assetName,
        'assets/images/avatar_male.png',
      );
      expect(find.text('?'), findsNothing);
    });

    testWidgets(
      'Personal account entry shows person full name in professional mode',
      (tester) async {
        // Setup person with business profiles
        final person = Person(
          id: 1,
          uuid: 'test-uuid',
          firstname: 'John',
          surname: 'Doe',
          businessProfiles: [
            BusinessProfile(
              id: 10,
              uuid: 'biz-uuid',
              ownerId: 1,
              ownerUuid: 'test-uuid',
              taxId: '123',
              businessName: 'Test Gym',
              businessType: 'Professional',
            ),
          ],
        );
        personProvider.setPersonForTest(person);
        personProvider.setActiveBusinessProfile(person.businessProfiles.first);

        await tester.pumpWidget(createTestWidget(ProfileMenu()));

        // Tap the profile menu button to open it
        await tester.tap(find.byType(GestureDetector).first);
        await tester.pumpAndSettle();

        // Verify "John Doe" appears (not "Personal account" label)
        expect(find.text('John Doe'), findsOneWidget);
      },
    );

    testWidgets('Business profile shows social name when defined', (
      tester,
    ) async {
      final person = Person(
        id: 1,
        uuid: 'test-uuid',
        firstname: 'John',
        surname: 'Doe',
        businessProfiles: [
          BusinessProfile(
            id: 10,
            uuid: 'biz-uuid',
            ownerId: 1,
            ownerUuid: 'test-uuid',
            taxId: '123',
            businessName: 'Fitness Corp',
            businessType: 'Professional',
            socialName: 'John\'s Gym',
          ),
        ],
      );
      personProvider.setPersonForTest(person);

      await tester.pumpWidget(createTestWidget(ProfileMenu()));

      await tester.tap(find.byType(GestureDetector).first);
      await tester.pumpAndSettle();

      // Verify social name appears as primary
      expect(find.text('John\'s Gym'), findsOneWidget);
      // Verify business name appears as secondary
      expect(find.text('Fitness Corp'), findsOneWidget);
    });

    testWidgets(
      'Business profile shows business name when social name not defined',
      (tester) async {
        final person = Person(
          id: 1,
          uuid: 'test-uuid',
          firstname: 'John',
          surname: 'Doe',
          businessProfiles: [
            BusinessProfile(
              id: 10,
              uuid: 'biz-uuid',
              ownerId: 1,
              ownerUuid: 'test-uuid',
              taxId: '123',
              businessName: 'Fitness Corp',
              businessType: 'Professional',
              socialName: null,
            ),
          ],
        );
        personProvider.setPersonForTest(person);

        await tester.pumpWidget(createTestWidget(ProfileMenu()));

        await tester.tap(find.byType(GestureDetector).first);
        await tester.pumpAndSettle();

        // Verify business name appears as primary
        expect(find.text('Fitness Corp'), findsOneWidget);
        // Verify business type appears as secondary
        expect(find.text('Professional'), findsOneWidget);
      },
    );
  });
}
