import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:lapidation_mobile/config/nav_section.dart';
import 'package:lapidation_mobile/l10n/app_localizations.dart';
import 'package:lapidation_mobile/models/business_profile.dart';
import 'package:lapidation_mobile/models/person.dart';
import 'package:lapidation_mobile/providers/auth_provider.dart';
import 'package:lapidation_mobile/providers/locale_provider.dart';
import 'package:lapidation_mobile/providers/person_provider.dart';
import 'package:lapidation_mobile/widgets/sidebar_menu.dart';

void main() {
  group('SidebarMenu Reactivity', () {
    late PersonProvider personProvider;
    late AuthProvider authProvider;
    late LocaleProvider localeProvider;

    setUp(() {
      SharedPreferences.setMockInitialValues({});
      personProvider = PersonProvider();
      authProvider = AuthProvider();
      localeProvider = LocaleProvider();
    });

    testWidgets('header avatar updates when switching to business profile', (
      WidgetTester tester,
    ) async {
      // Setup: Create person with business profile
      final person = Person(
        id: 1,
        uuid: 'person-uuid-3',
        firstname: 'Carlos',
        surname: 'Silva',
        gender: 'male',
        avatar: 'https://example.com/carlos.jpg',
        businessProfiles: [
          BusinessProfile(
            uuid: 'biz-789',
            ownerId: 1,
            ownerUuid: 'person-uuid-3',
            taxId: '11223344000199',
            businessName: 'Power Gym',
            businessType: 'gym',
            logo: 'https://example.com/power-logo.jpg',
          ),
        ],
      );

      personProvider.setPersonForTest(person);

      // Build widget tree
      await tester.pumpWidget(
        MaterialApp(
          localizationsDelegates: AppLocalizations.localizationsDelegates,
          supportedLocales: AppLocalizations.supportedLocales,
          home: MultiProvider(
            providers: [
              ChangeNotifierProvider<PersonProvider>.value(
                value: personProvider,
              ),
              ChangeNotifierProvider<AuthProvider>.value(value: authProvider),
              ChangeNotifierProvider<LocaleProvider>.value(
                value: localeProvider,
              ),
            ],
            child: Scaffold(
              body: SidebarMenu(
                navSection: NavSection.home,
                currentRoute: '/feed',
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Initial state: personal mode
      // Verify person name is displayed
      expect(find.text('Carlos Silva'), findsOneWidget);

      // Switch to business profile
      personProvider.setActiveBusinessProfile(person.businessProfiles.first);
      await tester.pumpAndSettle();

      // Verify: business name should now be displayed in header
      expect(find.text('Power Gym'), findsOneWidget);
      // Person name should still be visible (it's above business name)
      expect(find.text('Carlos Silva'), findsOneWidget);
    });

    testWidgets('menu items update when switching to business profile', (
      WidgetTester tester,
    ) async {
      // Setup: Create person with business profile
      final person = Person(
        id: 1,
        uuid: 'person-uuid-4',
        firstname: 'Maria',
        surname: 'Santos',
        gender: 'female',
        businessProfiles: [
          BusinessProfile(
            uuid: 'biz-101',
            ownerId: 1,
            ownerUuid: 'person-uuid-4',
            taxId: '55667788000111',
            businessName: 'Fit Studio',
            businessType: 'studio',
          ),
        ],
      );

      personProvider.setPersonForTest(person);

      // Build widget tree
      await tester.pumpWidget(
        MaterialApp(
          localizationsDelegates: AppLocalizations.localizationsDelegates,
          supportedLocales: AppLocalizations.supportedLocales,
          home: MultiProvider(
            providers: [
              ChangeNotifierProvider<PersonProvider>.value(
                value: personProvider,
              ),
              ChangeNotifierProvider<AuthProvider>.value(value: authProvider),
              ChangeNotifierProvider<LocaleProvider>.value(
                value: localeProvider,
              ),
            ],
            child: Scaffold(
              body: SidebarMenu(
                navSection: NavSection.home,
                currentRoute: '/feed',
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Initial state: personal mode
      // "Team" is reachable by every user (it also drives the person-side
      // received-invites view), so it should already be visible here.
      expect(
        find.text('Team'),
        findsOneWidget,
        reason: 'Team menu should appear in personal mode too',
      );

      // Switch to business profile
      personProvider.setActiveBusinessProfile(person.businessProfiles.first);
      await tester.pumpAndSettle();

      // Verify: business-specific menu items should now be visible
      // After switching to business profile, professional menu items should appear
      expect(
        find.text('Team'),
        findsOneWidget,
        reason: 'Team menu should still appear in professional mode',
      );

      // Also verify other professional-only items to confirm menu actually changed
      expect(
        find.text('Followers'),
        findsOneWidget,
        reason: 'Followers menu should appear in professional mode',
      );
    });
  });
}
