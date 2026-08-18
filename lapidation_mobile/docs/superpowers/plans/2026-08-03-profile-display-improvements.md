# Profile Display Improvements Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix profile menu display names and post composer avatars to show appropriate person/business information

**Architecture:** Create a centralized display name helper utility that provides pure functions for name formatting. Update ProfileMenu to show person's full name and business profile social names. Update PostComposerSheet avatar logic to select between business logo and person avatar based on active profile mode.

**Tech Stack:** Flutter/Dart, Provider state management, existing Person and BusinessProfile models

## Global Constraints

- Follow existing Dart/Flutter conventions in the codebase
- Use existing Person model with `firstname` and `surname` fields (not `lastname`)
- Use existing BusinessProfile model with `businessName` and optional `socialName` fields
- Maintain existing error handling and fallback patterns (avatar initials, default images)
- Write tests following existing test patterns (unit tests in `test/`, widget tests use flutter_test)

---

### Task 1: Display Name Helper Utility

**Files:**
- Create: `lib/utils/display_name_helper.dart`
- Test: `test/utils/display_name_helper_test.dart`

**Interfaces:**
- Consumes: `Person` model (`firstname: String`, `surname: String`), `BusinessProfile` model (`businessName: String`, `socialName: String?`, `businessType: String`)
- Produces: 
  - `String getPersonFullName(Person person)` - returns "firstname surname"
  - `String getBusinessProfileDisplayName(BusinessProfile profile)` - returns social name or business name
  - `Map<String, String> formatBusinessProfileForMenu(BusinessProfile profile)` - returns {primary, secondary} for menu display

- [ ] **Step 1: Write failing tests for display name helper**

Create `test/utils/display_name_helper_test.dart`:

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/models/person.dart';
import 'package:lapidation_mobile/models/business_profile.dart';
import 'package:lapidation_mobile/utils/display_name_helper.dart';

void main() {
  group('DisplayNameHelper', () {
    group('getPersonFullName', () {
      test('returns full name when both firstname and surname present', () {
        final person = Person(
          id: 1,
          uuid: 'test-uuid',
          firstname: 'John',
          surname: 'Doe',
        );
        expect(DisplayNameHelper.getPersonFullName(person), 'John Doe');
      });

      test('returns firstname only when surname is empty', () {
        final person = Person(
          id: 1,
          uuid: 'test-uuid',
          firstname: 'John',
          surname: '',
        );
        expect(DisplayNameHelper.getPersonFullName(person), 'John');
      });

      test('handles whitespace correctly', () {
        final person = Person(
          id: 1,
          uuid: 'test-uuid',
          firstname: '  John  ',
          surname: '  Doe  ',
        );
        expect(DisplayNameHelper.getPersonFullName(person), 'John Doe');
      });

      test('handles whitespace-only surname as empty', () {
        final person = Person(
          id: 1,
          uuid: 'test-uuid',
          firstname: 'John',
          surname: '   ',
        );
        expect(DisplayNameHelper.getPersonFullName(person), 'John');
      });
    });

    group('getBusinessProfileDisplayName', () {
      test('returns social name when defined', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: 'John\'s Gym',
        );
        expect(
          DisplayNameHelper.getBusinessProfileDisplayName(profile),
          'John\'s Gym',
        );
      });

      test('returns business name when social name is empty', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: '',
        );
        expect(
          DisplayNameHelper.getBusinessProfileDisplayName(profile),
          'Fitness Corp',
        );
      });

      test('returns business name when social name is null', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: null,
        );
        expect(
          DisplayNameHelper.getBusinessProfileDisplayName(profile),
          'Fitness Corp',
        );
      });

      test('handles whitespace-only social name as empty', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: '   ',
        );
        expect(
          DisplayNameHelper.getBusinessProfileDisplayName(profile),
          'Fitness Corp',
        );
      });
    });

    group('formatBusinessProfileForMenu', () {
      test('uses social name as primary when defined', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: 'John\'s Gym',
        );
        final result = DisplayNameHelper.formatBusinessProfileForMenu(profile);
        expect(result['primary'], 'John\'s Gym');
        expect(result['secondary'], 'Fitness Corp');
      });

      test('uses business name as primary when social name is empty', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: '',
        );
        final result = DisplayNameHelper.formatBusinessProfileForMenu(profile);
        expect(result['primary'], 'Fitness Corp');
        expect(result['secondary'], 'Professional');
      });

      test('uses business name as primary when social name is null', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: null,
        );
        final result = DisplayNameHelper.formatBusinessProfileForMenu(profile);
        expect(result['primary'], 'Fitness Corp');
        expect(result['secondary'], 'Professional');
      });

      test('handles whitespace-only social name', () {
        final profile = BusinessProfile(
          ownerId: 1,
          ownerUuid: 'owner-uuid',
          taxId: '123',
          businessName: 'Fitness Corp',
          businessType: 'Professional',
          socialName: '   ',
        );
        final result = DisplayNameHelper.formatBusinessProfileForMenu(profile);
        expect(result['primary'], 'Fitness Corp');
        expect(result['secondary'], 'Professional');
      });
    });
  });
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run:
```bash
flutter test test/utils/display_name_helper_test.dart
```

Expected: Multiple FAIL messages with "The getter 'getPersonFullName' isn't defined for the class 'DisplayNameHelper'"

- [ ] **Step 3: Create display name helper implementation**

Create `lib/utils/display_name_helper.dart`:

```dart
import '../models/person.dart';
import '../models/business_profile.dart';

/// Utility class for consistent display name formatting across the app
class DisplayNameHelper {
  /// Returns person's full name: "firstname surname"
  /// Falls back to firstname only if surname is missing or empty
  static String getPersonFullName(Person person) {
    final first = person.firstname.trim();
    final last = person.surname.trim();
    if (last.isEmpty) return first;
    return '$first $last';
  }

  /// Returns the display name for a business profile
  /// Uses social name if defined and non-empty, otherwise business name
  static String getBusinessProfileDisplayName(BusinessProfile profile) {
    final social = profile.socialName?.trim() ?? '';
    return social.isNotEmpty ? social : profile.businessName;
  }

  /// Returns formatted data for menu display with primary and secondary text
  /// When social name is defined: primary = social name, secondary = business name
  /// When social name is empty/null: primary = business name, secondary = business type
  static Map<String, String> formatBusinessProfileForMenu(
    BusinessProfile profile,
  ) {
    final social = profile.socialName?.trim() ?? '';
    final hasSocialName = social.isNotEmpty;

    return {
      'primary': hasSocialName ? social : profile.businessName,
      'secondary': hasSocialName ? profile.businessName : profile.businessType,
    };
  }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run:
```bash
flutter test test/utils/display_name_helper_test.dart
```

Expected: All tests PASS (16 tests)

- [ ] **Step 5: Commit display name helper**

```bash
git add lib/utils/display_name_helper.dart test/utils/display_name_helper_test.dart
git commit -m "feat: add display name helper utility

- Pure functions for person and business profile name formatting
- Handles social name precedence for business profiles
- Full test coverage for edge cases (whitespace, empty, null)

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 2: Update Profile Menu Display

**Files:**
- Modify: `lib/widgets/profile_menu.dart:1-332` (update imports, personal account entry, business profile entries)

**Interfaces:**
- Consumes: `DisplayNameHelper.getPersonFullName(Person)`, `DisplayNameHelper.formatBusinessProfileForMenu(BusinessProfile)`
- Produces: Updated ProfileMenu widget with correct display names

- [ ] **Step 1: Add import to profile_menu.dart**

Open `lib/widgets/profile_menu.dart` and add after line 9 (after other imports):

```dart
import '../utils/display_name_helper.dart';
```

- [ ] **Step 2: Update "Personal account" entry to show person's full name**

In `lib/widgets/profile_menu.dart`, find line 294 in the `_buildMenuItems` method:

Replace:
```dart
Text(l10n.businessProfileSwitchToPersonal),
```

With:
```dart
Text(
  person != null
      ? DisplayNameHelper.getPersonFullName(person)
      : l10n.businessProfileSwitchToPersonal,
),
```

- [ ] **Step 3: Update business profile entries to use social name**

In `lib/widgets/profile_menu.dart`, find lines 229-280 where business profiles are mapped.

Replace the entire map closure (lines 229-281):

```dart
...profiles.asMap().entries.map((entry) {
  final businessProfile = entry.value;
  return PopupMenuItem(
    value: 'profile_${entry.key}',
    child: Row(
      children: [
        if (businessProfile.logo?.isNotEmpty ?? false)
          CircleAvatar(
            radius: 12,
            backgroundImage: CachedNetworkImageProvider(
              businessProfile.logo!,
            ),
          )
        else
          CircleAvatar(
            radius: 12,
            backgroundColor: AppColors.primary,
            child: Text(
              businessProfile.businessName.isNotEmpty
                  ? businessProfile.businessName[0].toUpperCase()
                  : '?',
              style: const TextStyle(
                color: Colors.white,
                fontSize: 10,
                fontWeight: FontWeight.bold,
              ),
            ),
          ),
        const SizedBox(width: 12),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                businessProfile.businessName,
                style: const TextStyle(fontSize: 14),
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
              ),
              Text(
                businessProfile.businessType,
                style: const TextStyle(fontSize: 12, color: Colors.grey),
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
              ),
            ],
          ),
        ),
      ],
    ),
  );
}),
```

With:

```dart
...profiles.asMap().entries.map((entry) {
  final businessProfile = entry.value;
  final displayData =
      DisplayNameHelper.formatBusinessProfileForMenu(businessProfile);
  return PopupMenuItem(
    value: 'profile_${entry.key}',
    child: Row(
      children: [
        if (businessProfile.logo?.isNotEmpty ?? false)
          CircleAvatar(
            radius: 12,
            backgroundImage: CachedNetworkImageProvider(
              businessProfile.logo!,
            ),
          )
        else
          CircleAvatar(
            radius: 12,
            backgroundColor: AppColors.primary,
            child: Text(
              businessProfile.businessName.isNotEmpty
                  ? businessProfile.businessName[0].toUpperCase()
                  : '?',
              style: const TextStyle(
                color: Colors.white,
                fontSize: 10,
                fontWeight: FontWeight.bold,
              ),
            ),
          ),
        const SizedBox(width: 12),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                displayData['primary']!,
                style: const TextStyle(
                  fontSize: 14,
                  fontWeight: FontWeight.w600,
                ),
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
              ),
              Text(
                displayData['secondary']!,
                style: const TextStyle(fontSize: 12, color: Colors.grey),
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
              ),
            ],
          ),
        ),
      ],
    ),
  );
}),
```

- [ ] **Step 4: Run app and manually verify profile menu changes**

Run:
```bash
flutter run
```

Manual verification steps:
1. Log in with an account that has business profiles
2. Click profile menu in header
3. Verify "Personal account" entry shows person's full name (e.g., "John Doe")
4. Verify business profile entries show social name if defined, with business name as subtitle
5. Verify business profile entries without social name show business name with type as subtitle

- [ ] **Step 5: Commit profile menu changes**

```bash
git add lib/widgets/profile_menu.dart
git commit -m "feat: update profile menu to show person full name and social names

- Personal account entry shows 'Firstname Surname' instead of label
- Business profiles show social name (when defined) as primary
- Business name appears as subtitle when social name is used
- Business type appears as subtitle when no social name

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 3: Update Post Composer Avatar Display

**Files:**
- Modify: `lib/widgets/post_composer_sheet.dart:529-534` (update _AvatarWidget instantiation to pass correct logo based on mode)

**Interfaces:**
- Consumes: `_isBusinessMode: bool`, `_businessProfileLogo: String?`, `_person.avatar: String?`
- Produces: Updated PostComposerSheet with avatar showing business logo in professional mode, person avatar in personal mode

- [ ] **Step 1: Verify current avatar widget logic**

The `_AvatarWidget` class (lines 796-850) already has the correct logic:
- If `businessLogo` is non-null and non-empty, it shows the business logo
- Otherwise, it shows the person's avatar
- Falls back to gender-based default avatar image

The issue is that `businessLogo` is currently passed when `_isBusinessMode` is true, but we need to ensure it's only passed when the logo actually exists.

- [ ] **Step 2: Update avatar widget instantiation**

In `lib/widgets/post_composer_sheet.dart`, find line 529-534 where `_AvatarWidget` is instantiated:

Replace:
```dart
_AvatarWidget(
  person: person,
  businessLogo: _isBusinessMode
      ? _businessProfileLogo
      : null,
),
```

With:
```dart
_AvatarWidget(
  person: person,
  businessLogo: (_isBusinessMode &&
          _businessProfileLogo != null &&
          _businessProfileLogo!.isNotEmpty)
      ? _businessProfileLogo
      : null,
),
```

- [ ] **Step 3: Run app and manually verify post composer avatar**

Run:
```bash
flutter run
```

Manual verification steps:
1. Log in as personal mode (not business profile active)
2. Open post composer (tap "What's on your mind?" or create post button)
3. Verify person's avatar is shown
4. Close composer
5. Switch to business profile from profile menu
6. Open post composer again
7. Verify business logo is shown (if business has a logo)
8. Switch to business profile without logo
9. Verify fallback default avatar is shown

- [ ] **Step 4: Commit post composer avatar changes**

```bash
git add lib/widgets/post_composer_sheet.dart
git commit -m "feat: show appropriate avatar in post composer based on profile mode

- Business logo shown when in professional mode with logo defined
- Person avatar shown in personal mode or when business has no logo
- Maintains existing fallback behavior to gender-based default images

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 4: Widget Tests for Profile Menu

**Files:**
- Create: `test/widgets/profile_menu_test.dart`

**Interfaces:**
- Consumes: `ProfileMenu` widget, `PersonProvider`, `AuthProvider`
- Produces: Widget tests verifying display name behavior

- [ ] **Step 1: Write widget tests for profile menu display**

Create `test/widgets/profile_menu_test.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:lapidation_mobile/models/person.dart';
import 'package:lapidation_mobile/models/business_profile.dart';
import 'package:lapidation_mobile/providers/person_provider.dart';
import 'package:lapidation_mobile/providers/auth_provider.dart';
import 'package:lapidation_mobile/widgets/profile_menu.dart';
import 'package:lapidation_mobile/l10n/app_localizations.dart';
import 'package:flutter_localizations/flutter_localizations.dart';

void main() {
  group('ProfileMenu Display Tests', () {
    late PersonProvider personProvider;
    late AuthProvider authProvider;

    setUp(() {
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
          home: Scaffold(
            body: child,
          ),
        ),
      );
    }

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
        personProvider.setPerson(person);
        personProvider.setActiveBusinessProfile(person.businessProfiles.first);

        await tester.pumpWidget(
          createTestWidget(
            ProfileMenu(
              contextMenuPosition: ContextMenuPosition.appHeader,
            ),
          ),
        );

        // Tap the profile menu button to open it
        await tester.tap(find.byType(GestureDetector).first);
        await tester.pumpAndSettle();

        // Verify "John Doe" appears (not "Personal account" label)
        expect(find.text('John Doe'), findsOneWidget);
      },
    );

    testWidgets(
      'Business profile shows social name when defined',
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
              socialName: 'John\'s Gym',
            ),
          ],
        );
        personProvider.setPerson(person);

        await tester.pumpWidget(
          createTestWidget(
            ProfileMenu(
              contextMenuPosition: ContextMenuPosition.appHeader,
            ),
          ),
        );

        await tester.tap(find.byType(GestureDetector).first);
        await tester.pumpAndSettle();

        // Verify social name appears as primary
        expect(find.text('John\'s Gym'), findsOneWidget);
        // Verify business name appears as secondary
        expect(find.text('Fitness Corp'), findsOneWidget);
      },
    );

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
        personProvider.setPerson(person);

        await tester.pumpWidget(
          createTestWidget(
            ProfileMenu(
              contextMenuPosition: ContextMenuPosition.appHeader,
            ),
          ),
        );

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
```

- [ ] **Step 2: Run widget tests**

Run:
```bash
flutter test test/widgets/profile_menu_test.dart
```

Expected: All tests PASS (3 tests)

If tests fail due to widget tree navigation issues, adjust the test setup or widget finding logic.

- [ ] **Step 3: Commit widget tests**

```bash
git add test/widgets/profile_menu_test.dart
git commit -m "test: add widget tests for profile menu display names

- Verify personal account shows full name in professional mode
- Verify business profiles show social name when defined
- Verify business profiles show business name when social name absent

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

## Implementation Complete

All tasks implement the display improvements specified in the design document:
1. ✅ Centralized display name helper utility
2. ✅ Profile menu shows person full name and business social names
3. ✅ Post composer shows appropriate avatar based on profile mode
4. ✅ Comprehensive unit tests for helper utility
5. ✅ Widget tests for profile menu behavior

**Final verification:**
```bash
flutter test
flutter run
```

All tests should pass, and the app should display correct names and avatars in both personal and professional modes.
