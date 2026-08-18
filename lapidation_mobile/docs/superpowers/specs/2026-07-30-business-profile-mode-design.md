# Business Profile Mode - Design Specification

**Date:** 2026-07-30  
**Status:** Approved  
**Author:** Copilot (with Erick Rocha)

## Overview

This feature enables users to switch between personal and business profile modes within the Lapidation mobile app. When switched to business mode, the app adapts its UI, theme, and data sources to reflect the active business profile.

## Problem Statement

Currently, users can create business profiles but cannot actively use them to post content or view business-specific feeds. The app needs to support switching between personal and business contexts, with appropriate UI adaptations including:
- Different color theme (professional blue vs. personal teal)
- Business-specific feed from `/timeline/api/business/feed/{businessProfileUuid}`
- Different sidebar menu items (Team, Followers instead of Friends, Messages)
- Business profile images (logo, cover) displayed throughout the UI
- Profile mode persistence across app sessions

## Design Approach

**Architecture Pattern:** State-Based Profile Mode using existing Provider infrastructure.

This approach leverages the existing `PersonProvider` as the single source of truth for profile mode. All components adapt by reading `isProfessional` and `activeBusinessProfile` state.

### Why This Approach?

1. **Fits existing architecture:** Already using Provider pattern, `isProfessional` exists
2. **Minimal duplication:** Single set of pages adapt to context
3. **Theme system ready:** `main.dart` already switches colors based on `isProfessional`
4. **Maintainable:** Centralized state management, one source of truth
5. **Extensible:** Easy to add more profile types later (organization, group, etc.)

## Architecture & State Management

### State Flow

```
User Action (ProfileMenu) 
  → PersonProvider.switchProfile(index, token)
  → Update _activeBusinessProfile
  → Persist to SharedPreferences
  → notifyListeners()
  → All widgets rebuild
  → Feed reloads from business endpoint
  → Sidebar shows business menu items
  → Theme switches to professional colors
```

### Core State Properties

**PersonProvider** (existing, enhanced):
- `isProfessional: bool` → returns `_activeBusinessProfile != null`
- `activeBusinessProfile: BusinessProfile?` → current business profile or null
- `switchProfile(int index, String token)` → switch to business profile by index
- `switchToPersonal()` → return to personal mode

**State Persistence:**
- Stored in SharedPreferences under key `active_profile`
- Restored on app launch via `_loadFromStorage()`
- Cleared on logout

### Theme Integration

The existing theme system (`main.dart` lines 94-96) already adapts:
```dart
colorScheme: ColorScheme.fromSeed(
  seedColor: personProvider.isProfessional
      ? AppColors.professionalSecondary  // Blue (#1B1795)
      : AppColors.primary,                // Teal (#7CB2A8)
  surface: AppColors.background,
),
```

Professional gradient also exists (`AppColors.professionalGradient3`). No changes needed to theme logic.

## UI Components & Navigation

### Sidebar Menu Adaptation

**Personal Mode Menu:**
- Feed
- Gallery
- Profile
- Friends
- Messages (coming soon)
- Notifications
- Settings
- Language
- Logout

**Business Mode Menu:**
- Feed (business feed)
- Gallery (business images)
- Profile (business profile)
- Team (new page)
- Followers (new page)
- Notifications
- Settings
- Language
- Logout

**Implementation:**
- Update `_homeItems()` in `lib/widgets/sidebar_menu.dart`
- Check `context.read<PersonProvider>().isProfessional`
- Return appropriate menu items
- Navigation targets adapt (e.g., `/profile` routes to `/business-profile` in business mode)

### Avatar & Image Display

All avatar/image widgets adapt to show business profile images when `isProfessional`:

**Affected Components:**
- Sidebar header avatar → business logo
- Profile menu button → business logo
- Post composer avatar → business logo
- Post author avatar → business logo (for business posts)

**Implementation Pattern:**
```dart
Widget _buildAvatar(BuildContext context) {
  final personProvider = context.watch<PersonProvider>();
  final imageUrl = personProvider.isProfessional
      ? personProvider.activeBusinessProfile?.logo
      : personProvider.person?.avatar;
  
  // Render avatar with imageUrl
}
```

### Business Mode Indicator

Add visual indicator in sidebar header:
- Show business name below user name when in business mode
- Use professional gradient for header background
- Optional badge: "Business Mode"

### Route Handling

**Strategy:** Keep existing routes, pages adapt based on state.

- `/feed` → renders personal or business feed based on `isProfessional`
- `/gallery` → filters by current feed (already connected to FeedProvider)
- `/profile` → navigates to `/business-profile` in business mode
- `/team` → new route, only accessible in business mode
- `/followers` → new route, only accessible in business mode

**Why not separate routes?**
- Reduces complexity (no `/business/*` routes needed)
- Leverages existing page infrastructure
- Simpler state management
- Easier to maintain

## Feed Integration & Business Endpoints

### API Configuration

Add to `lib/config/api_config.dart`:
```dart
static const String businessFeedEndpoint = '/timeline/api/business/feed';
```

### FeedService Updates

Add new method to `lib/services/feed_service.dart`:
```dart
static Future<List<FeedPost>> fetchBusinessFeed(
  String businessProfileUuid,
  String token,
  {int page = 0}
) async {
  try {
    DioClient().setAuthToken(token);
    final response = await _dio.get(
      '${ApiConfig.businessFeedEndpoint}/$businessProfileUuid',
      queryParameters: {'page': page},
    );
    if (response.statusCode == 200) {
      final List<dynamic> data = response.data;
      return data.map((e) => FeedPost.fromJson(e as Map<String, dynamic>)).toList();
    } else {
      throw AppException(
        statusCode: response.statusCode ?? 500,
        message: response.data?['message'] ?? 'Failed to load business feed',
      );
    }
  } on DioException catch (e) {
    throw BaseService.handleDioError(e, 'Failed to load business feed');
  }
}
```

### FeedProvider Enhancement

Add to `lib/providers/feed_provider.dart`:
```dart
Future<void> fetchBusinessFeed(String businessProfileUuid, String token) async {
  _loading = true;
  _error = null;
  notifyListeners();

  try {
    final fetchedPosts = await FeedService.fetchBusinessFeed(businessProfileUuid, token, page: 0);
    _posts = fetchedPosts;
    _currentPage = 0;
    _hasMore = fetchedPosts.length == _pageSize;
    _loading = false;
    notifyListeners();
  } on AppException catch (e) {
    _error = e.message;
    _loading = false;
    notifyListeners();
  } catch (_) {
    _error = 'Failed to load business feed. Please try again.';
    _loading = false;
    notifyListeners();
  }
}
```

Keep existing `fetchPosts(String token)` for personal feed.

### Feed Page Adaptation

Update `lib/pages/home/home_page.dart` `_loadPosts()` method:
```dart
void _loadPosts() {
  final token = _token;
  if (token.isEmpty) return;
  
  final personProvider = context.read<PersonProvider>();
  final feedProvider = context.read<FeedProvider>();
  
  if (personProvider.isProfessional) {
    final businessProfileUuid = personProvider.activeBusinessProfile?.uuid;
    if (businessProfileUuid != null) {
      feedProvider.fetchBusinessFeed(businessProfileUuid, token);
    }
  } else {
    feedProvider.fetchPosts(token);
  }
}
```

### Post Creation in Business Mode

When creating a post in business mode:
- Include `businessProfileUuid` in post data payload
- Post attributed to business profile (shows business logo as author)
- Backend associates post with business profile

Update `lib/widgets/post_composer_sheet.dart` to include business context when `isProfessional`.

### Gallery Integration

**No changes needed to GalleryPage.**

Since `GalleryPage` loads images from `FeedProvider.posts`, and the feed is already filtered by profile mode, the gallery automatically shows correct images.

## New Pages

### Team Page

**File:** `lib/pages/team/team_page.dart`

**Purpose:** Manage team members who can post/manage the business profile.

**Initial Implementation:**
- Empty state with "Team management coming soon" message
- Layout structure similar to `FriendsPage`
- Prepared for future team member management features

**Future Features:**
- List team members with avatars and roles
- Add/remove team members
- Set permissions (admin, editor, viewer)
- gRPC service integration for team operations

**UI Structure:**
```dart
MainLayout(
  navSection: NavSection.home,
  currentRoute: '/team',
  child: Scaffold(
    appBar: AppHeader(title: l10n.menuTeam),
    body: Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(Icons.group, size: 64, color: Colors.grey),
          SizedBox(height: 16),
          Text('Team management coming soon'),
        ],
      ),
    ),
  ),
)
```

### Followers Page

**File:** `lib/pages/followers/followers_page.dart`

**Purpose:** Display users following the business profile.

**Initial Implementation:**
- Empty state with "No followers yet" message
- Layout structure similar to `FriendsPage`
- Prepared for follower list integration

**Future Features:**
- Grid/list of follower avatars with names
- Pull-to-refresh support
- Pagination for large follower lists
- View follower profiles
- API endpoint: `/workout/api/business-profiles/{uuid}/followers`

**UI Structure:**
```dart
MainLayout(
  navSection: NavSection.home,
  currentRoute: '/followers',
  child: Scaffold(
    appBar: AppHeader(title: l10n.menuFollowers),
    body: Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(Icons.people, size: 64, color: Colors.grey),
          SizedBox(height: 16),
          Text('No followers yet'),
        ],
      ),
    ),
  ),
)
```

### Route Registration

Add to `lib/main.dart` routes:
```dart
'/team': (context) => const TeamPage(),
'/followers': (context) => const FollowersPage(),
```

### Localization Keys

Add to `lib/l10n/app_en.arb`:
```json
"menuTeam": "Team",
"menuFollowers": "Followers"
```

(Repeat for other locales: pt, es, fr, nl)

## Error Handling & Edge Cases

### Profile Switching Flow

1. User selects business profile from ProfileMenu
2. `PersonProvider.switchProfile(index, token)` validates index
3. Update `_activeBusinessProfile` and persist to SharedPreferences
4. `notifyListeners()` triggers UI rebuild
5. Automatic navigation to `/feed` with business context
6. Feed loads business-specific data

**Error Cases:**
- Invalid index: Show error toast, stay in current mode
- Network error: State switches (UI adapts), feed shows error state with retry
- Missing business profile: Fallback to personal mode

### Edge Cases

**No Business Profiles:**
- ProfileMenu only shows "Add New Profile" option
- Business mode menu items never appear

**Business Profile Deleted:**
- On app launch, check if stored UUID exists in `person.businessProfiles`
- If not found: clear stored preference, fallback to personal mode
- Show toast: "Business profile no longer available"

**Network Error During Feed Load:**
- Show error state in feed with retry button
- Don't break UI or revert profile mode
- User can retry or navigate away

**Invalid Business Profile UUID:**
- Clear stored `activeBusinessProfile` from SharedPreferences
- Switch to personal mode
- Log error for debugging

**Switching While Feed Is Loading:**
- Cancel current feed request (abort controller)
- Clear current posts immediately
- Start new feed request for target mode
- Show loading indicator

### State Restoration on App Launch

```dart
PersonProvider._loadFromStorage() {
  // Load person from storage
  final personJson = prefs.getString('person');
  if (personJson != null) {
    _person = Person.fromJson(jsonDecode(personJson));
  }
  
  // Load active business profile
  final activeProfileJson = prefs.getString('active_profile');
  if (activeProfileJson != null) {
    final storedProfile = BusinessProfile.fromJson(jsonDecode(activeProfileJson));
    
    // Validate: does this profile still exist?
    if (_person != null && _person!.businessProfiles.any((p) => p.uuid == storedProfile.uuid)) {
      _activeBusinessProfile = storedProfile;
    } else {
      // Profile no longer exists, clear storage
      await prefs.remove('active_profile');
      _activeBusinessProfile = null;
    }
  }
  
  notifyListeners();
}
```

### Token Refresh

- Maintain current profile mode during token refresh
- No state reset needed
- Continue showing current feed/UI

### Data Consistency

**Problem:** Stale feed data visible during mode switch.

**Solution:**
- Show loading indicator immediately on switch
- Clear old posts from provider before loading new ones
- Use `_posts = []` then `_loading = true` in provider

**Alternative:** Keep old posts visible with loading overlay until new posts arrive (less jarring UX).

## Testing Strategy

### Unit Tests

**PersonProvider Tests:**
- `switchProfile()` updates state correctly
- `switchToPersonal()` clears business profile
- State persistence to SharedPreferences
- State restoration validates stored profile exists
- Error handling for invalid indices

**FeedProvider Tests:**
- `fetchBusinessFeed()` calls correct endpoint
- Personal vs business feed routing logic
- Error handling for network failures

### Widget Tests

**Sidebar Menu Tests:**
- Shows personal menu items when `isProfessional = false`
- Shows business menu items when `isProfessional = true`
- Navigation targets adapt to mode

**Avatar Display Tests:**
- Shows personal avatar in personal mode
- Shows business logo in business mode
- Handles missing images gracefully

### Integration Tests

**Profile Switch Flow:**
1. Start in personal mode
2. Open ProfileMenu, select business profile
3. Verify navigation to `/feed`
4. Verify business feed loads
5. Verify sidebar shows business menu items
6. Verify theme switches to professional colors
7. Switch back to personal
8. Verify personal feed loads

**State Persistence:**
1. Switch to business mode
2. Close app
3. Reopen app
4. Verify still in business mode
5. Verify business feed loads

## Performance Considerations

**Profile Switch:**
- Local state change (instant)
- SharedPreferences write is async (non-blocking)
- Feed reload happens after switch (async)

**Image Caching:**
- `CachedNetworkImage` handles caching
- Business logo cached separately from personal avatar
- No re-download on mode switch

**Feed Loading:**
- Current posts cleared immediately on switch
- Loading indicator shown
- New posts loaded async
- No blocking operations

**Memory:**
- Single feed list in memory (not duplicated per mode)
- Old feed cleared before loading new one
- Image cache managed by `cached_network_image` package

## Data Model Extensions

### No Changes to Existing Models

- `Person` model: already has `businessProfiles` list
- `BusinessProfile` model: already has `uuid`, `logo`, `coverImage`
- `FeedPost` model: no changes needed (backend handles attribution)

### PersonProvider Extensions

Already exists:
- `activeBusinessProfile: BusinessProfile?`
- `isProfessional: bool`
- `switchProfile(int index, String token)`
- `switchToPersonal()`

Already persisted:
- Active profile stored in SharedPreferences
- Restored on app launch

**No new models needed.**

## Security & Permissions

### Authorization

- Backend validates business profile access on `/timeline/api/business/feed/{uuid}`
- JWT token includes user identity
- User must own the business profile to post as it
- Team member permissions handled by backend (future feature)

### Client-Side Validation

- Only show business profiles the user owns in ProfileMenu
- Don't allow switching to profiles not in `person.businessProfiles`
- Clear stored profile if validation fails on launch

### Data Isolation

- Personal and business feeds are separate backend endpoints
- No mixing of data sources on client side
- Clear separation of personal vs business posts

## Rollout & Migration

### No Migration Needed

- Existing users have `person.businessProfiles` already
- New state (`activeBusinessProfile`) defaults to null (personal mode)
- No database schema changes
- No data migration required

### Gradual Rollout

1. Deploy backend changes first (ensure `/timeline/api/business/feed/{uuid}` exists)
2. Deploy mobile app update
3. Users can immediately switch to business mode
4. Team and Followers pages show "coming soon" until backend ready

### Backward Compatibility

- App works without business profiles (personal mode only)
- App works if backend business feed endpoint doesn't exist yet (shows error)
- No breaking changes to existing functionality

## Future Enhancements

### Phase 2: Team Management
- Full team member management in TeamPage
- Role-based permissions (admin, editor, viewer)
- gRPC service for team operations
- Invite team members via email/username

### Phase 3: Follower Management
- Full follower list in FollowersPage
- Search/filter followers
- View follower profiles
- Export follower list

### Phase 4: Analytics
- Business profile analytics dashboard
- Post engagement metrics
- Follower growth tracking
- Best posting times

### Phase 5: Multi-Profile Improvements
- Quick profile switcher in header (dropdown)
- Profile-specific notification badges
- Profile-specific settings
- Scheduled posts per profile

## Open Questions

1. **Backend Readiness:** Is `/timeline/api/business/feed/{businessProfileUuid}` already implemented?
2. **Post Attribution:** Does backend automatically attribute posts to business profile when `businessProfileUuid` is included?
3. **Team Member API:** What's the gRPC service method for team member management?
4. **Followers API:** What's the endpoint for business profile followers?
5. **Notifications:** Should business profile notifications be separate from personal?

## Success Criteria

✅ User can switch between personal and business profiles  
✅ Theme changes to professional colors in business mode  
✅ Business feed loads from `/timeline/api/business/feed/{uuid}`  
✅ Sidebar menu shows Team and Followers in business mode  
✅ Business logo appears throughout UI in business mode  
✅ Profile mode persists across app sessions  
✅ Gallery shows business images in business mode  
✅ Posts created in business mode attributed to business  
✅ Switching back to personal mode works correctly  
✅ No crashes or errors during mode switching  

## Conclusion

This design provides a clean, maintainable approach to business profile mode switching by leveraging the existing Provider architecture. The state-based pattern ensures a single source of truth, minimal code duplication, and easy extensibility for future profile types.

The implementation requires updates to existing components (FeedProvider, Sidebar, avatars) but introduces no breaking changes. New pages (Team, Followers) start with empty states, ready for future feature development.

The design handles edge cases gracefully, persists user preference, and maintains data consistency during mode switches. Performance impact is minimal due to efficient state management and image caching.
