import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:cached_network_image/cached_network_image.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:socialgym_mobile/l10n/app_localizations.dart';
import 'package:socialgym_mobile/models/business_profile.dart';
import 'package:socialgym_mobile/models/person.dart';
import 'package:socialgym_mobile/providers/auth_provider.dart';
import 'package:socialgym_mobile/providers/feed_provider.dart';
import 'package:socialgym_mobile/providers/person_provider.dart';
import 'package:socialgym_mobile/widgets/post_composer_sheet.dart';

void main() {
  group('PostComposerSheet Reactivity', () {
    late PersonProvider personProvider;
    late AuthProvider authProvider;
    late FeedProvider feedProvider;

    setUp(() {
      SharedPreferences.setMockInitialValues({});
      personProvider = PersonProvider();
      authProvider = AuthProvider();
      feedProvider = FeedProvider();
    });

    testWidgets('avatar updates when switching to business profile', (
      WidgetTester tester,
    ) async {
      // Setup: Create person with business profile
      final person = Person(
        id: 1,
        uuid: 'person-uuid-1',
        firstname: 'John',
        surname: 'Doe',
        gender: 'male',
        avatar: 'https://example.com/avatar.jpg',
        businessProfiles: [
          BusinessProfile(
            uuid: 'biz-123',
            ownerId: 1,
            ownerUuid: 'person-uuid-1',
            taxId: '12345678000190',
            businessName: 'Acme Gym',
            businessType: 'gym',
            logo: 'https://example.com/logo.jpg',
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
              ChangeNotifierProvider<FeedProvider>.value(value: feedProvider),
            ],
            child: Scaffold(body: PostComposerSheet()),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Initial state: personal mode, person avatar should be used
      // The avatar is in _AvatarWidget which checks businessLogo parameter
      // In personal mode, businessLogo should be null, so person.avatar is used

      // Switch to business profile
      personProvider.setActiveBusinessProfile(person.businessProfiles.first);
      await tester.pumpAndSettle();

      // Verify: business logo should now be displayed
      // Check that CachedNetworkImage with business logo URL is present
      expect(
        find.byWidgetPredicate(
          (widget) =>
              widget is CachedNetworkImage &&
              widget.imageUrl == 'https://example.com/logo.jpg',
        ),
        findsOneWidget,
        reason:
            'Business logo should be displayed via CachedNetworkImage when in business mode',
      );

      // Verify: the display name shows business name
      expect(find.text('Acme Gym'), findsOneWidget);
    });

    testWidgets('avatar updates when switching back to personal', (
      WidgetTester tester,
    ) async {
      // Setup: Create person with business profile, start in business mode
      final person = Person(
        id: 1,
        uuid: 'person-uuid-2',
        firstname: 'Jane',
        surname: 'Smith',
        gender: 'female',
        avatar: 'https://example.com/jane.jpg',
        businessProfiles: [
          BusinessProfile(
            uuid: 'biz-456',
            ownerId: 1,
            ownerUuid: 'person-uuid-2',
            taxId: '98765432000180',
            businessName: 'Elite Fitness',
            businessType: 'gym',
            logo: 'https://example.com/elite-logo.jpg',
          ),
        ],
      );

      personProvider.setPersonForTest(person);
      personProvider.setActiveBusinessProfile(person.businessProfiles.first);

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
              ChangeNotifierProvider<FeedProvider>.value(value: feedProvider),
            ],
            child: Scaffold(body: PostComposerSheet()),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Initial state: business mode, business name should be displayed
      expect(find.text('Elite Fitness'), findsOneWidget);

      // Switch to personal mode
      personProvider.setActiveBusinessProfile(null);
      await tester.pumpAndSettle();

      // Verify: person full name should now be displayed
      expect(find.text('Jane Smith'), findsOneWidget);
      expect(find.text('Elite Fitness'), findsNothing);
    });
  });
}
