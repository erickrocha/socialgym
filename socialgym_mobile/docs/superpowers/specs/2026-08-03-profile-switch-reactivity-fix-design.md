# Profile Switch Reactivity Fix - Design Specification

**Date:** 2026-08-03  
**Status:** Approved  
**Related Issue:** Profile avatar and info not updating when switching between personal and business accounts

## Problem Statement

When users switch from personal account to business profile (or vice versa) using the profile menu, the post composer and sidebar menu continue to display stale profile information. The avatar and display name don't update until the user manually closes and reopens these components.

**Root Cause (identified via systematic debugging):**
- `post_composer_sheet.dart` uses `context.read<PersonProvider>()` in getters (lines 93-104)
- `sidebar_menu.dart` uses `context.read<PersonProvider>()` in helper methods (lines 173, 391-394)
- `.read()` doesn't establish a listener, so widgets don't rebuild when `PersonProvider` state changes
- `ProfileMenu` works correctly because it uses `context.watch<PersonProvider>()`

## Goals

1. **Instant updates:** Profile avatar and info update immediately when user switches profiles
2. **Correct reactivity pattern:** Use `context.watch()` for display logic, `context.read()` for actions
3. **Minimal changes:** Surgical fixes only to affected widgets
4. **No performance regression:** Only rebuild widgets that display profile-dependent data

## Non-Goals

- Refactoring unrelated code
- Optimizing with `Selector` (premature optimization)
- Scanning/fixing all files in codebase (only post_composer and sidebar have the issue)

## Architecture

### Flutter Provider Pattern

**Standard pattern:**
- `context.watch<T>()` - Establishes listener, widget rebuilds when provider changes (use for display)
- `context.read<T>()` - One-time read, no listener (use for actions/callbacks)

**Current state:**
- `profile_menu.dart` ✅ - Correctly uses `.watch()` for display, `.read()` for actions
- `post_composer_sheet.dart` ❌ - Uses `.read()` in getters called during build
- `sidebar_menu.dart` ⚠️ - Mixed: `.watch()` in build(), but `.read()` in helper methods

### Data Flow

**Before fix:**
1. User clicks profile switch → `PersonProvider.switchProfile()` called
2. PersonProvider updates state and calls `notifyListeners()`
3. ProfileMenu rebuilds ✅
4. Post composer does NOT rebuild ❌
5. Sidebar does NOT rebuild ❌
6. User sees stale data

**After fix:**
1. User clicks profile switch → `PersonProvider.switchProfile()` called
2. PersonProvider updates state and calls `notifyListeners()`
3. All widgets using `context.watch<PersonProvider>()` rebuild:
   - ProfileMenu ✅
   - Post composer ✅
   - Sidebar ✅
4. User sees updated avatar/name instantly

## Implementation Details

### 1. Post Composer Sheet Fix

**File:** `lib/widgets/post_composer_sheet.dart`

**Current implementation (problematic):**
```dart
// Lines 93-104: Getters use context.read()
Person? get _person => context.read<PersonProvider>().person;
bool get _isBusinessMode => context.read<PersonProvider>().isProfessional;
String? get _businessProfileUuid => context.read<PersonProvider>().activeBusinessProfile?.uuid;
String? get _businessProfileName => context.read<PersonProvider>().activeBusinessProfile?.businessName;
String? get _businessProfileLogo => context.read<PersonProvider>().activeBusinessProfile?.logo;
```

**Fix approach:**
- Add `context.watch<PersonProvider>()` call once at the top of the `build()` method
- Replace getters with local variables derived from the watched provider instance
- This establishes the listener so widget rebuilds when profile switches

**Why this works:**
- `context.watch()` called during `build()` registers the widget with PersonProvider's listener list
- When `switchProfile()` calls `notifyListeners()`, Flutter rebuilds this widget
- Local variables get fresh values from the updated provider

### 2. Sidebar Menu Fix

**File:** `lib/widgets/sidebar_menu.dart`

**Current implementation (partially correct):**
```dart
// Line 107: Correctly watches provider
@override
Widget build(BuildContext context) {
  final personProvider = context.watch<PersonProvider>();
  // ...
}

// Line 173: Helper method uses context.read() - breaks reactivity
List<Widget> _homeItems(BuildContext context, AppLocalizations l10n) {
  final isProfessional = context.read<PersonProvider>().isProfessional;
  // ...
}

// Line 391-394: Helper method uses context.read() - breaks reactivity
Widget _buildAvatar(BuildContext context, Person? person, {bool isCollapsed = false}) {
  final activeBusinessProfile = context.read<PersonProvider>().activeBusinessProfile;
  // ...
}
```

**Fix approach:**
- Keep `context.watch<PersonProvider>()` in `build()` (line 107)
- Pass the watched `personProvider` instance to helper methods as a parameter
- Helper methods use the passed parameter instead of calling `context.read()`

**Methods requiring updates:**
- `_buildSectionItems()` - pass personProvider down
- `_homeItems()` - add personProvider parameter
- `_workoutItems()` - add personProvider parameter (if it needs it)
- `_buildAlwaysItems()` - check if it needs personProvider
- `_buildAvatar()` - add personProvider parameter
- `_buildDrawerHeader()` - check if it needs personProvider

**Why this works:**
- The `build()` method establishes the listener once
- Helper methods receive the fresh provider instance via parameters
- All code paths use the same watched instance, maintaining consistency

### 3. Profile Menu (No Changes)

**File:** `lib/widgets/profile_menu.dart`

**Current implementation (already correct):**
- Line 34: Uses `context.watch<PersonProvider>()` for display
- Lines 163, 167, 178, 203: Uses `context.read<PersonProvider>()` for actions

**Why no changes needed:**
- Display logic correctly uses `.watch()` and rebuilds on profile switch
- Action handlers correctly use `.read()` since they trigger changes, not display state

## Testing Strategy

### Widget Tests

**Test 1: Post Composer Reactivity**
- Setup: Render post composer in personal account mode
- Action: Call `personProvider.switchProfile()` to activate business profile
- Assert: Avatar widget displays business logo (not person avatar)
- Assert: Display name shows business name (not person name)

**Test 2: Sidebar Reactivity**
- Setup: Render sidebar menu in personal account mode
- Action: Call `personProvider.switchProfile()` to activate business profile
- Assert: Header avatar displays business logo
- Assert: Business menu items appear

**Test 3: Profile Menu (Regression)**
- Verify existing profile menu tests still pass
- No new tests needed (already has correct reactivity)

### Manual Testing Checklist

1. **Post Composer:**
   - Open post composer in personal mode → verify person avatar shown
   - Switch to business profile → verify composer avatar updates to business logo immediately (without closing)
   - Switch back to personal → verify composer avatar updates to person avatar
   - Verify display name also updates correctly

2. **Sidebar Menu:**
   - Open sidebar in personal mode → verify person avatar in header
   - Switch to business profile → verify header avatar updates to business logo
   - Verify menu items change (business-specific items appear)
   - Switch back to personal → verify everything reverts

3. **Profile Menu (Regression):**
   - Verify profile switching still works
   - Verify menu shows correct profiles

## Error Handling

**No new error cases introduced.** The change is purely about establishing proper Provider listeners. Existing edge case handling remains:

- **Null business profile:** Existing null checks (`activeBusinessProfile?.logo`) remain unchanged
- **Empty logo string:** Existing `isNotEmpty` checks remain unchanged
- **Profile switch failure:** Handled by `PersonProvider.switchProfile()`, widgets display current state

## Performance Considerations

**Rebuild scope:**
- Only widgets that call `context.watch<PersonProvider>()` rebuild when profile switches
- Post composer: Single widget rebuild, minimal impact
- Sidebar: Single widget rebuild, helper methods are just function calls (not separate widgets)
- No cascade rebuilds - parent widgets don't rebuild unless they also watch PersonProvider

**Why Selector is unnecessary:**
- No performance issue with current approach
- Post composer and sidebar legitimately need to rebuild on profile switch (they display profile data)
- `Selector` would add complexity without measurable benefit

## Migration Notes

**No breaking changes:**
- No API changes to PersonProvider
- No changes to existing component interfaces
- Purely internal implementation changes to establish proper listeners

**Files modified:**
1. `lib/widgets/post_composer_sheet.dart` - Replace getters with watched provider
2. `lib/widgets/sidebar_menu.dart` - Pass watched provider to helper methods
3. `test/widgets/profile_menu_test.dart` - Extend with reactivity tests (or create new test file for post_composer/sidebar)

## Success Criteria

1. ✅ Profile avatar updates instantly in post composer when switching profiles
2. ✅ Profile avatar updates instantly in sidebar when switching profiles
3. ✅ Display names update instantly in both widgets
4. ✅ No manual refresh/reopen required
5. ✅ All existing tests pass
6. ✅ New widget tests verify reactivity behavior
7. ✅ No performance regression (measured by lack of unexpected rebuilds)

## References

- Flutter Provider documentation: https://pub.dev/packages/provider
- Existing implementation: `lib/widgets/profile_menu.dart` (correct pattern example)
- Root cause analysis: Systematic debugging session 2026-08-03
