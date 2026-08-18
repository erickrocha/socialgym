# Profile Display Improvements — Design

**Date:** 2026-08-03
**Scope:** Display name and avatar corrections for profile menu and post composer

## Goal

Fix display inconsistencies in the profile menu and post composer:
1. Profile menu should show person's full name (not "Personal account" label) when switching back to personal mode
2. Business profile entries should display social name (when defined) as primary, with business name as subtitle
3. Post composer should show the appropriate avatar based on active profile mode (business logo for professional mode, person avatar for personal mode)

## Background

From the existing business profile implementation:
- `PersonProvider` already tracks `isProfessional` and `activeBusinessProfile`
- `ProfileMenu` has a "Personal account" entry using `l10n.businessProfileSwitchToPersonal` (currently shows localized label, not person name)
- Business profile entries currently display only `businessName` and `businessType`
- `BusinessProfile` model has both `businessName` and optional `socialName` fields
- `PostComposerSheet` has access to business mode state but needs avatar selection logic

## Decisions

1. **Display name helper utility**: Centralize display logic in a testable helper rather than duplicating in widgets
2. **Social name precedence**: When social name is defined and non-empty, use it as primary; business name becomes subtitle
3. **Avatar selection**: Business logo takes precedence in professional mode; fall back to person avatar
4. **Full name format**: Person's full name is "firstname lastname" with space separator

## Architecture

### New File

**`lib/utils/display_name_helper.dart`**

Pure utility functions for display name logic:

```dart
class DisplayNameHelper {
  /// Returns person's full name: "firstname lastname"
  /// Falls back to firstname only if lastname is missing/empty
  static String getPersonFullName(Person person) {
    final first = person.firstname.trim();
    final last = person.lastname.trim();
    if (last.isEmpty) return first;
    return '$first $last';
  }

  /// Returns the display name for a business profile
  /// Uses social name if defined, otherwise business name
  static String getBusinessProfileDisplayName(BusinessProfile profile) {
    final social = profile.socialName?.trim() ?? '';
    return social.isNotEmpty ? social : profile.businessName;
  }

  /// Returns formatted data for menu display
  /// primary: social name or business name
  /// secondary: business name (when social is used) or business type
  static Map<String, String> formatBusinessProfileForMenu(BusinessProfile profile) {
    final social = profile.socialName?.trim() ?? '';
    final hasSocialName = social.isNotEmpty;
    
    return {
      'primary': hasSocialName ? social : profile.businessName,
      'secondary': hasSocialName ? profile.businessName : profile.businessType,
    };
  }
}
```

### Modified Files

#### `lib/widgets/profile_menu.dart`

**Change 1: Personal account entry display name** (around line 294)

Replace:
```dart
Text(l10n.businessProfileSwitchToPersonal)
```

With:
```dart
Text(DisplayNameHelper.getPersonFullName(person))
```

**Change 2: Business profile entries display** (around lines 263-274)

Replace:
```dart
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
```

With:
```dart
final displayData = DisplayNameHelper.formatBusinessProfileForMenu(businessProfile);
Text(
  displayData['primary']!,
  style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w600),
  maxLines: 1,
  overflow: TextOverflow.ellipsis,
),
Text(
  displayData['secondary']!,
  style: const TextStyle(fontSize: 12, color: Colors.grey),
  maxLines: 1,
  overflow: TextOverflow.ellipsis,
),
```

Add import:
```dart
import '../utils/display_name_helper.dart';
```

#### `lib/widgets/post_composer_sheet.dart`

The post composer needs to show the correct avatar in its header. After viewing the file, I'll locate the avatar rendering section and apply this logic:

**Avatar selection logic** (to be applied in the build method where avatar is rendered):

```dart
// Determine which avatar to display
String? displayAvatar;
String fallbackInitial;

if (_isBusinessMode && _businessProfileLogo != null && _businessProfileLogo!.isNotEmpty) {
  displayAvatar = _businessProfileLogo;
  fallbackInitial = _businessProfileName?.isNotEmpty == true 
      ? _businessProfileName![0].toUpperCase() 
      : '?';
} else {
  displayAvatar = _person?.avatar;
  fallbackInitial = _person?.firstname.isNotEmpty == true
      ? _person!.firstname[0].toUpperCase()
      : '?';
}
```

Then use `displayAvatar` and `fallbackInitial` in the CircleAvatar rendering.

## Testing

### Unit Tests

**`test/utils/display_name_helper_test.dart`**

```dart
group('DisplayNameHelper', () {
  group('getPersonFullName', () {
    test('returns full name when both present', () {
      final person = Person(firstname: 'John', lastname: 'Doe', ...);
      expect(DisplayNameHelper.getPersonFullName(person), 'John Doe');
    });

    test('returns firstname only when lastname empty', () {
      final person = Person(firstname: 'John', lastname: '', ...);
      expect(DisplayNameHelper.getPersonFullName(person), 'John');
    });

    test('handles whitespace correctly', () {
      final person = Person(firstname: '  John  ', lastname: '  Doe  ', ...);
      expect(DisplayNameHelper.getPersonFullName(person), 'John Doe');
    });
  });

  group('getBusinessProfileDisplayName', () {
    test('returns social name when defined', () {
      final profile = BusinessProfile(
        businessName: 'Fitness Corp',
        socialName: 'John\'s Gym',
        ...
      );
      expect(DisplayNameHelper.getBusinessProfileDisplayName(profile), 'John\'s Gym');
    });

    test('returns business name when social name empty', () {
      final profile = BusinessProfile(
        businessName: 'Fitness Corp',
        socialName: '',
        ...
      );
      expect(DisplayNameHelper.getBusinessProfileDisplayName(profile), 'Fitness Corp');
    });

    test('returns business name when social name null', () {
      final profile = BusinessProfile(
        businessName: 'Fitness Corp',
        socialName: null,
        ...
      );
      expect(DisplayNameHelper.getBusinessProfileDisplayName(profile), 'Fitness Corp');
    });
  });

  group('formatBusinessProfileForMenu', () {
    test('uses social name as primary when defined', () {
      final profile = BusinessProfile(
        businessName: 'Fitness Corp',
        socialName: 'John\'s Gym',
        businessType: 'Professional',
        ...
      );
      final result = DisplayNameHelper.formatBusinessProfileForMenu(profile);
      expect(result['primary'], 'John\'s Gym');
      expect(result['secondary'], 'Fitness Corp');
    });

    test('uses business name as primary when social name empty', () {
      final profile = BusinessProfile(
        businessName: 'Fitness Corp',
        socialName: '',
        businessType: 'Professional',
        ...
      );
      final result = DisplayNameHelper.formatBusinessProfileForMenu(profile);
      expect(result['primary'], 'Fitness Corp');
      expect(result['secondary'], 'Professional');
    });
  });
});
```

### Widget Tests

**`test/widgets/profile_menu_test.dart`** (update or add):

```dart
testWidgets('Personal account entry shows person full name', (tester) async {
  // Setup PersonProvider with person data and isProfessional = true
  // Build ProfileMenu
  // Verify "John Doe" appears instead of localized label
});

testWidgets('Business profile entry shows social name when defined', (tester) async {
  // Setup PersonProvider with business profile having social name
  // Build ProfileMenu
  // Verify social name in primary, business name in subtitle
});
```

**`test/widgets/post_composer_sheet_test.dart`** (update or add):

```dart
testWidgets('Shows business logo in professional mode', (tester) async {
  // Setup PersonProvider with isProfessional = true and business logo
  // Build PostComposerSheet
  // Verify business logo URL is used in avatar
});

testWidgets('Shows person avatar in personal mode', (tester) async {
  // Setup PersonProvider with isProfessional = false
  // Build PostComposerSheet
  // Verify person avatar URL is used
});
```

## Edge Cases

1. **Missing lastname**: Show only firstname without trailing space
2. **Whitespace-only social name**: Treat as empty, use business name
3. **Missing avatar/logo**: Fall back to initial from appropriate name
4. **Empty firstname**: Use '?' as fallback initial

## Out of Scope

- No changes to data models (Person, BusinessProfile)
- No changes to API contracts or gRPC services
- No changes to profile switching logic or state management
- No changes to post submission logic
- No localization updates (except removing usage of `businessProfileSwitchToPersonal` label)

## Implementation Notes

All changes are display-only and non-breaking. The helper utility has no dependencies on Flutter widgets or providers, making it testable with simple unit tests. Avatar rendering changes maintain existing fallback behavior for missing images.
