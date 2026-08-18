# Profile Switch Reactivity Fix Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix profile avatar and display name not updating instantly when switching between personal and business accounts

**Architecture:** Convert `context.read<PersonProvider>()` to `context.watch<PersonProvider>()` in widgets that display profile-dependent data, following Flutter's standard Provider pattern for reactive UI updates

**Tech Stack:** Flutter 3.x, Provider package

## Global Constraints

- Use `context.watch<PersonProvider>()` for display logic that needs to rebuild on state changes
- Use `context.read<PersonProvider>()` only for action handlers (button clicks, callbacks)
- Follow existing code style and patterns in the codebase
- No breaking changes to PersonProvider API
- All existing tests must continue to pass

---

## Task 1: Fix Post Composer Reactivity

**Files:**
- Modify: `lib/widgets/post_composer_sheet.dart:460-550` (build method)
- Test: Manual verification (widget test added in Task 3)

**Interfaces:**
- Consumes: `PersonProvider` (existing, no changes)
- Produces: Reactive post composer that updates instantly on profile switch

- [ ] **Step 1: Locate the build method**

Navigate to `lib/widgets/post_composer_sheet.dart` and find the `build()` method (starts around line 460).

Current code pattern around line 529-544:
```dart
child: Row(
  children: [
    _AvatarWidget(
      person: person,
      businessLogo: (_isBusinessMode &&
              _businessProfileLogo != null &&
              _businessProfileLogo!.isNotEmpty)
          ? _businessProfileLogo
          : null,
    ),
    const SizedBox(width: 12),
    Text(
      _displayName(person),
      // ...
    ),
  ],
),
```

The problem: `_isBusinessMode`, `_businessProfileLogo`, and `_businessProfileName` are getters (lines 95-104) that call `context.read<PersonProvider>()`, so they don't establish listeners.

- [ ] **Step 2: Add PersonProvider watch at top of build method**

Find the `build()` method (around line 460). After the existing variable declarations (like `final l10n = AppLocalizations.of(context)!;`), add:

```dart
@override
Widget build(BuildContext context) {
  final l10n = AppLocalizations.of(context)!;
  final person = context.watch<PersonProvider>().person;
  final personProvider = context.watch<PersonProvider>();
  final isBusinessMode = personProvider.isProfessional;
  final businessProfileLogo = personProvider.activeBusinessProfile?.logo;
  final businessProfileName = personProvider.activeBusinessProfile?.businessName;
  final businessProfileUuid = personProvider.activeBusinessProfile?.uuid;
```

Note: We're calling `watch()` which establishes a listener, so when `PersonProvider` calls `notifyListeners()` after profile switch, this widget will rebuild.

- [ ] **Step 3: Update avatar widget instantiation**

Find the `_AvatarWidget` instantiation (around line 529-536) and update it to use the local variables:

```dart
_AvatarWidget(
  person: person,
  businessLogo: (isBusinessMode &&
          businessProfileLogo != null &&
          businessProfileLogo.isNotEmpty)
      ? businessProfileLogo
      : null,
),
```

- [ ] **Step 4: Update display name call**

Find the `_displayName(person)` call (around line 539) and update the method to accept parameters instead of using getters:

First, update the call site:
```dart
Text(
  _displayName(person, isBusinessMode, businessProfileName),
  style: const TextStyle(
    fontWeight: FontWeight.bold,
    fontSize: 15,
  ),
),
```

- [ ] **Step 5: Update _displayName method signature**

Find the `_displayName` method (around line 682-688) and update it:

```dart
String _displayName(Person? p, bool isBusinessMode, String? businessProfileName) {
  if (isBusinessMode && (businessProfileName?.isNotEmpty ?? false)) {
    return businessProfileName!;
  }
  if (p == null) return '';
  return p.fullName.trim().isEmpty ? p.firstname : p.fullName;
}
```

- [ ] **Step 6: Update _submit method to use local variables**

Find the `_submit` method (around line 360-430). It currently uses `_isBusinessMode`, `_businessProfileLogo`, `_businessProfileUuid` getters. 

We need to keep these getters for now since `_submit` is an async method and can't easily get the watched values. The getters using `context.read()` are fine in action handlers (they trigger changes, don't display state).

Actually, looking at the code structure, the `_submit` method is a callback, not build-time code, so using `context.read()` there is correct. No changes needed to `_submit`.

- [ ] **Step 7: Remove old getters that are no longer needed in build**

The getters `_isBusinessMode`, `_businessProfileLogo`, `_businessProfileName`, `_businessProfileUuid` (lines 95-104) are still needed for the `_submit` method, so **DO NOT DELETE THEM**.

They use `context.read()` which is correct for action handlers like `_submit`.

- [ ] **Step 8: Verify the changes**

Check that:
1. The `build()` method uses `context.watch<PersonProvider>()` once at the top
2. Local variables (`isBusinessMode`, `businessProfileLogo`, `businessProfileName`) are used in the avatar and display name rendering
3. The `_displayName` method takes parameters instead of using getters
4. The `_submit` method still uses the existing getters (no changes needed)

- [ ] **Step 9: Test manually**

Run the app:
```bash
cd /home/erocha/workspace/lapidation_project/lapidation_mobile
flutter run
```

Manual test:
1. Open post composer in personal mode → verify person avatar shown
2. Switch to business profile (without closing composer) → verify avatar updates to business logo
3. Verify display name also updates
4. Switch back to personal → verify avatar reverts to person avatar

Expected: Avatar and name update instantly without closing the composer.

---

## Task 2: Fix Sidebar Menu Reactivity

**Files:**
- Modify: `lib/widgets/sidebar_menu.dart:160-280` (helper methods)
- Modify: `lib/widgets/sidebar_menu.dart:383-442` (_buildAvatar method)

**Interfaces:**
- Consumes: `PersonProvider` (existing, watched in build method at line 107)
- Produces: Reactive sidebar that updates avatar instantly on profile switch

- [ ] **Step 1: Review current implementation**

The `build()` method (line 104-157) already correctly uses `context.watch<PersonProvider>()` at line 107:

```dart
@override
Widget build(BuildContext context) {
  final l10n = AppLocalizations.of(context)!;
  final authProvider = context.watch<AuthProvider>();
  final personProvider = context.watch<PersonProvider>();  // ✅ Correct
```

The problem: Helper methods use `context.read<PersonProvider>()` which breaks reactivity:
- Line 173: `_homeItems` uses `context.read<PersonProvider>()`
- Line 391-394: `_buildAvatar` uses `context.read<PersonProvider>()`

Fix: Pass `personProvider` from `build()` to these helper methods.

- [ ] **Step 2: Update _buildSectionItems method**

Find `_buildSectionItems` (line 161-170) and add `personProvider` parameter:

```dart
List<Widget> _buildSectionItems(
  BuildContext context,
  AppLocalizations l10n,
  PersonProvider personProvider,
) {
  switch (navSection) {
    case NavSection.home:
      return _homeItems(context, l10n, personProvider);
    case NavSection.gallery:
      return _homeItems(context, l10n, personProvider);
    case NavSection.workout:
      return _workoutItems(context, l10n, personProvider);
  }
}
```

- [ ] **Step 3: Update _buildSectionItems call in build method**

Find the call to `_buildSectionItems` in the `build()` method (around line 125) and pass `personProvider`:

```dart
children: [
  ..._buildSectionItems(context, l10n, personProvider),
  const Divider(indent: 16, endIndent: 16),
  ..._buildAlwaysItems(context, l10n),
],
```

- [ ] **Step 4: Update _homeItems method**

Find `_homeItems` method (starts at line 172) and update it:

```dart
List<Widget> _homeItems(
  BuildContext context,
  AppLocalizations l10n,
  PersonProvider personProvider,
) {
  final isProfessional = personProvider.isProfessional;  // Changed from context.read()
  if (isProfessional) {
    return [
      // ... rest of the method unchanged
```

- [ ] **Step 5: Update _workoutItems method**

Find `_workoutItems` method (search for `List<Widget> _workoutItems`). Check if it uses `PersonProvider`. If it does, add the parameter:

```dart
List<Widget> _workoutItems(
  BuildContext context,
  AppLocalizations l10n,
  PersonProvider personProvider,
) {
  // Update any context.read<PersonProvider>() calls to use personProvider parameter
```

If `_workoutItems` doesn't use `PersonProvider`, you still need to add the parameter to match the signature expected by `_buildSectionItems`, but you don't need to use it:

```dart
List<Widget> _workoutItems(
  BuildContext context,
  AppLocalizations l10n,
  PersonProvider personProvider,
) {
  // existing code unchanged
```

- [ ] **Step 6: Update _buildAvatar method signature**

Find `_buildAvatar` method (starts around line 384-388) and add `personProvider` parameter:

```dart
Widget _buildAvatar(
  BuildContext context,
  Person? person,
  PersonProvider personProvider, {
  bool isCollapsed = false,
}) {
  final radius = isCollapsed ? 20.0 : 32.0;
  final size = radius * 2;
  final activeBusinessProfile = personProvider.activeBusinessProfile;  // Changed from context.read()
  final businessLogo = activeBusinessProfile?.logo;
  // ... rest unchanged
```

- [ ] **Step 7: Update _buildAvatar calls**

Find all calls to `_buildAvatar`. There should be two in `_buildDrawerHeader` method (around lines 474 and 499):

```dart
// First call (collapsed header, around line 474):
_buildAvatar(context, person, personProvider, isCollapsed: true),

// Second call (expanded header, around line 499):
_buildAvatar(context, person, personProvider),
```

- [ ] **Step 8: Check for other context.read usages**

Search the file for any remaining `context.read<PersonProvider>()` calls that are used for display logic (not action handlers):

```bash
grep -n "context.read<PersonProvider>" lib/widgets/sidebar_menu.dart
```

Expected remaining usages:
- Line 147: `context.read<PersonProvider>().clear()` - This is an action handler, correct usage ✅
- Line 253: Inside `_buildAlwaysItems` method - Check if this is display or action

If line 253 is in `_buildAlwaysItems` and is used for conditional display, update `_buildAlwaysItems` to accept `personProvider` parameter and pass it from `build()`.

- [ ] **Step 9: Update _buildAlwaysItems if needed**

Check if `_buildAlwaysItems` uses `context.read<PersonProvider>()` for display logic. If it does (around line 253):

```dart
List<Widget> _buildAlwaysItems(
  BuildContext context,
  AppLocalizations l10n,
  PersonProvider personProvider,  // Add parameter
) {
  // Replace context.read<PersonProvider>() with personProvider
```

And update the call in `build()` method (around line 127):

```dart
..._buildAlwaysItems(context, l10n, personProvider),
```

- [ ] **Step 10: Verify the changes**

Check that:
1. `build()` method still uses `context.watch<PersonProvider>()` (line 107) ✅
2. All helper methods that need PersonProvider accept it as a parameter
3. No `context.read<PersonProvider>()` calls remain in display logic (methods called during build)
4. `context.read<PersonProvider>()` is only used in action handlers like logout (line 147) ✅

- [ ] **Step 11: Test manually**

Run the app:
```bash
flutter run
```

Manual test:
1. Open sidebar in personal mode → verify person avatar in header
2. Switch to business profile → verify header avatar updates to business logo instantly (without closing sidebar)
3. Verify business name appears under person name
4. Switch back to personal → verify avatar reverts and business name disappears
5. Test collapsed sidebar → verify avatar updates work there too

Expected: Avatar updates instantly without closing the sidebar.

---

## Task 3: Add Widget Tests for Reactivity

**Files:**
- Create: `test/widgets/post_composer_reactivity_test.dart`
- Create: `test/widgets/sidebar_menu_reactivity_test.dart`

**Interfaces:**
- Consumes: Fixed post_composer_sheet.dart and sidebar_menu.dart from Tasks 1-2
- Produces: Automated tests verifying profile switch reactivity

- [ ] **Step 1: Create post composer reactivity test file**

Create `test/widgets/post_composer_reactivity_test.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:lapidation_mobile/l10n/app_localizations.dart';
import 'package:lapidation_mobile/models/business_profile.dart';
import 'package:lapidation_mobile/models/person.dart';
import 'package:lapidation_mobile/providers/auth_provider.dart';
import 'package:lapidation_mobile/providers/feed_provider.dart';
import 'package:lapidation_mobile/providers/person_provider.dart';
import 'package:lapidation_mobile/widgets/post_composer_sheet.dart';

void main() {
  group('PostComposerSheet Reactivity', () {
    late PersonProvider personProvider;
    late AuthProvider authProvider;
    late FeedProvider feedProvider;

    setUp(() {
      personProvider = PersonProvider();
      authProvider = AuthProvider();
      feedProvider = FeedProvider();
    });

    testWidgets('avatar updates when switching to business profile',
        (WidgetTester tester) async {
      // Setup: Create person with business profile
      final person = Person(
        id: 1,
        firstname: 'John',
        surname: 'Doe',
        username: 'johndoe',
        email: 'john@example.com',
        gender: 'male',
        avatar: 'https://example.com/avatar.jpg',
        businessProfiles: [
          BusinessProfile(
            uuid: 'biz-123',
            businessName: 'Acme Gym',
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
              ChangeNotifierProvider<PersonProvider>.value(value: personProvider),
              ChangeNotifierProvider<AuthProvider>.value(value: authProvider),
              ChangeNotifierProvider<FeedProvider>.value(value: feedProvider),
            ],
            child: Scaffold(
              body: PostComposerSheet(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Initial state: personal mode, person avatar should be used
      // The avatar is in _AvatarWidget which checks businessLogo parameter
      // In personal mode, businessLogo should be null, so person.avatar is used

      // Switch to business profile
      personProvider.setActiveBusinessProfile(person.businessProfiles![0]);
      await tester.pumpAndSettle();

      // Verify: business logo should now be displayed
      // We can't easily verify the image URL directly, but we can verify the widget rebuilt
      // by checking that the post composer sheet is still present
      expect(find.byType(PostComposerSheet), findsOneWidget);

      // Additional verification: The display name should show business name
      expect(find.text('Acme Gym'), findsOneWidget);
    });

    testWidgets('avatar updates when switching back to personal',
        (WidgetTester tester) async {
      // Setup: Create person with business profile, start in business mode
      final person = Person(
        id: 1,
        firstname: 'Jane',
        surname: 'Smith',
        username: 'janesmith',
        email: 'jane@example.com',
        gender: 'female',
        avatar: 'https://example.com/jane.jpg',
        businessProfiles: [
          BusinessProfile(
            uuid: 'biz-456',
            businessName: 'Elite Fitness',
            logo: 'https://example.com/elite-logo.jpg',
          ),
        ],
      );

      personProvider.setPersonForTest(person);
      personProvider.setActiveBusinessProfile(person.businessProfiles![0]);

      // Build widget tree
      await tester.pumpWidget(
        MaterialApp(
          localizationsDelegates: AppLocalizations.localizationsDelegates,
          supportedLocales: AppLocalizations.supportedLocales,
          home: MultiProvider(
            providers: [
              ChangeNotifierProvider<PersonProvider>.value(value: personProvider),
              ChangeNotifierProvider<AuthProvider>.value(value: authProvider),
              ChangeNotifierProvider<FeedProvider>.value(value: feedProvider),
            ],
            child: Scaffold(
              body: PostComposerSheet(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Initial state: business mode, business name should be displayed
      expect(find.text('Elite Fitness'), findsOneWidget);

      // Switch to personal mode
      personProvider.switchToPersonal();
      await tester.pumpAndSettle();

      // Verify: person full name should now be displayed
      expect(find.text('Jane Smith'), findsOneWidget);
      expect(find.text('Elite Fitness'), findsNothing);
    });
  });
}
```

- [ ] **Step 2: Run post composer reactivity tests**

```bash
flutter test test/widgets/post_composer_reactivity_test.dart
```

Expected: Tests may fail initially due to missing test helpers in PersonProvider (setActiveBusinessProfile, switchToPersonal might not have `@visibleForTesting` methods). If they fail, proceed to next step.

- [ ] **Step 3: Add test helper to PersonProvider if needed**

If tests fail because `setActiveBusinessProfile` doesn't exist, check `lib/providers/person_provider.dart`. 

According to the summary, there should already be a `@visibleForTesting setActiveBusinessProfile(BusinessProfile)` method added in a previous task (commit edb7cdc).

If it's missing, add it:

```dart
@visibleForTesting
void setActiveBusinessProfile(BusinessProfile profile) {
  _activeBusinessProfile = profile;
  notifyListeners();
}
```

Also verify `switchToPersonal()` method exists. If not, add:

```dart
void switchToPersonal() {
  _activeBusinessProfile = null;
  notifyListeners();
}
```

- [ ] **Step 4: Re-run post composer tests**

```bash
flutter test test/widgets/post_composer_reactivity_test.dart
```

Expected: Both tests pass.

- [ ] **Step 5: Create sidebar menu reactivity test file**

Create `test/widgets/sidebar_menu_reactivity_test.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:lapidation_mobile/config/nav_section.dart';
import 'package:lapidation_mobile/l10n/app_localizations.dart';
import 'package:lapidation_mobile/models/business_profile.dart';
import 'package:lapidation_mobile/models/person.dart';
import 'package:lapidation_mobile/providers/auth_provider.dart';
import 'package:lapidation_mobile/providers/person_provider.dart';
import 'package:lapidation_mobile/widgets/sidebar_menu.dart';

void main() {
  group('SidebarMenu Reactivity', () {
    late PersonProvider personProvider;
    late AuthProvider authProvider;

    setUp(() {
      personProvider = PersonProvider();
      authProvider = AuthProvider();
    });

    testWidgets('header avatar updates when switching to business profile',
        (WidgetTester tester) async {
      // Setup: Create person with business profile
      final person = Person(
        id: 1,
        firstname: 'Carlos',
        surname: 'Silva',
        username: 'carlossilva',
        email: 'carlos@example.com',
        gender: 'male',
        avatar: 'https://example.com/carlos.jpg',
        businessProfiles: [
          BusinessProfile(
            uuid: 'biz-789',
            businessName: 'Power Gym',
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
              ChangeNotifierProvider<PersonProvider>.value(value: personProvider),
              ChangeNotifierProvider<AuthProvider>.value(value: authProvider),
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
      personProvider.setActiveBusinessProfile(person.businessProfiles![0]);
      await tester.pumpAndSettle();

      // Verify: business name should now be displayed in header
      expect(find.text('Power Gym'), findsOneWidget);
      // Person name should still be visible (it's above business name)
      expect(find.text('Carlos Silva'), findsOneWidget);
    });

    testWidgets('menu items update when switching to business profile',
        (WidgetTester tester) async {
      // Setup: Create person with business profile
      final person = Person(
        id: 1,
        firstname: 'Maria',
        surname: 'Santos',
        username: 'mariasantos',
        email: 'maria@example.com',
        gender: 'female',
        businessProfiles: [
          BusinessProfile(
            uuid: 'biz-101',
            businessName: 'Fit Studio',
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
              ChangeNotifierProvider<PersonProvider>.value(value: personProvider),
              ChangeNotifierProvider<AuthProvider>.value(value: authProvider),
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
      // Business-specific menu items should NOT be visible
      // (Check for "Team" menu item which appears in professional mode - line 203)

      // Switch to business profile
      personProvider.setActiveBusinessProfile(person.businessProfiles![0]);
      await tester.pumpAndSettle();

      // Verify: business-specific menu items should now be visible
      // The sidebar should have rebuilt and show professional menu
      expect(find.byType(SidebarMenu), findsOneWidget);
    });
  });
}
```

- [ ] **Step 6: Run sidebar menu reactivity tests**

```bash
flutter test test/widgets/sidebar_menu_reactivity_test.dart
```

Expected: Both tests pass.

- [ ] **Step 7: Run all widget tests**

```bash
flutter test test/widgets/
```

Expected: All widget tests pass, including the new reactivity tests.

- [ ] **Step 8: Run full test suite**

```bash
flutter test
```

Expected: All tests in the project pass.

---

## Verification Checklist

After completing all tasks:

- [ ] Post composer avatar updates instantly when switching profiles (manual test)
- [ ] Sidebar menu avatar updates instantly when switching profiles (manual test)
- [ ] Display names update correctly in both widgets (manual test)
- [ ] All new widget tests pass (`flutter test test/widgets/`)
- [ ] All existing tests still pass (`flutter test`)
- [ ] No console errors or warnings when switching profiles
- [ ] No unexpected widget rebuilds (verify with Flutter DevTools if needed)

## Notes

- **Git commits:** The user will handle git commits manually after implementation is complete
- **Provider pattern:** We're following Flutter's standard pattern: `watch()` for display, `read()` for actions
- **Performance:** Only widgets that actually display profile data rebuild on switch - no cascade rebuilds
- **Backward compatibility:** No API changes to PersonProvider, so no other code needs updates
