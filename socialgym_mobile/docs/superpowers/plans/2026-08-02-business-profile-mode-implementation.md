# Business Profile Mode Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement business profile mode so switching profiles changes feed source, sidebar options, and profile imagery consistently across feed, gallery, and composer flows.

**Architecture:** Keep `PersonProvider` as the mode source of truth (`isProfessional` + `activeBusinessProfile`). Extend `FeedService`/`FeedProvider` with business feed API methods and make UI call mode-aware provider methods. Reuse existing pages/routes where possible, adding only `TeamPage` and `FollowersPage` scaffolds for business-only navigation.

**Tech Stack:** Flutter, Provider, Dio, gRPC client wrappers, Flutter localization (`.arb` + generated localization classes), flutter_test

## Global Constraints

- Use existing professional color scheme (`AppColors.professionalSecondary`) for business mode.
- Business feed endpoint must be `/timeline/api/business/feed/{businessProfileUuid}`.
- Business mode sidebar must show: Feed, Gallery, Profile, Team, Followers, Notifications.
- Business mode images should come from active business profile data (`logo` / `coverImage`), not personal profile.
- Keep profile mode persistent via `SharedPreferences` using current `PersonProvider` persistence flow.
- Team and Followers pages are in scope as working routes/pages; advanced backend features can remain minimal UI until APIs are finalized.

---

### Task 1: Add business feed API support in service layer

**Files:**
- Modify: `lib/config/api_config.dart`
- Modify: `lib/services/feed_service.dart`
- Create: `test/feed_service_business_feed_test.dart`

**Interfaces:**
- Consumes: `ApiConfig.feedEndpoint`, `FeedPost.fromJson(Map<String, dynamic>)`
- Produces:
  - `ApiConfig.businessFeedEndpoint: String`
  - `FeedService.fetchBusinessFeed(String token, String businessProfileUuid, {int page = 0}): Future<List<FeedPost>>`

- [ ] **Step 1: Write failing service tests for business endpoint URL and response parsing**

```dart
test('fetchBusinessFeed calls /timeline/api/business/feed/{uuid} with page', () async {
  // Arrange mocked Dio to capture request path + query
  // Act FeedService.fetchBusinessFeed('token', 'bp-123', page: 2)
  // Assert path == '/timeline/api/business/feed/bp-123' and query['page'] == 2
});

test('fetchBusinessFeed maps response array into FeedPost list', () async {
  // Arrange response.data as List<Map<String, dynamic>>
  // Assert parsed list length and first uuid
});
```

- [ ] **Step 2: Run test to verify failure**

Run: `flutter test test/feed_service_business_feed_test.dart`  
Expected: FAIL with missing `businessFeedEndpoint` and/or `fetchBusinessFeed`

- [ ] **Step 3: Implement business feed endpoint constant and service method**

```dart
// lib/config/api_config.dart
static const String businessFeedEndpoint = '/timeline/api/business/feed';

// lib/services/feed_service.dart
static Future<List<FeedPost>> fetchBusinessFeed(
  String token,
  String businessProfileUuid, {
  int page = 0,
}) async {
  DioClient().setAuthToken(token);
  final response = await _dio.get(
    '${ApiConfig.businessFeedEndpoint}/$businessProfileUuid',
    queryParameters: {'page': page},
  );
  if (response.statusCode == 200) {
    final List<dynamic> data = response.data;
    return data.map((e) => FeedPost.fromJson(e as Map<String, dynamic>)).toList();
  }
  throw AppException(
    statusCode: response.statusCode ?? 500,
    message: response.data?['message'] ?? 'Failed to load business feed',
  );
}
```

- [ ] **Step 4: Run tests to verify pass**

Run: `flutter test test/feed_service_business_feed_test.dart`  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add lib/config/api_config.dart lib/services/feed_service.dart test/feed_service_business_feed_test.dart
git commit -m "feat(feed): add business profile feed endpoint support"
```

### Task 2: Extend FeedProvider for business mode fetch + pagination

**Files:**
- Modify: `lib/providers/feed_provider.dart`
- Create: `test/feed_provider_business_mode_test.dart`

**Interfaces:**
- Consumes:
  - `FeedService.fetchPosts(String token, {int page = 0})`
  - `FeedService.fetchBusinessFeed(String token, String businessProfileUuid, {int page = 0})`
- Produces:
  - `FeedProvider.fetchPostsForProfile(String token, {String? businessProfileUuid}): Future<void>`
  - `FeedProvider.loadMorePostsForProfile(String token, {String? businessProfileUuid}): Future<void>`

- [ ] **Step 1: Write failing provider tests for mode-aware fetch behavior**

```dart
test('fetchPostsForProfile calls personal feed when businessProfileUuid is null', () async {});
test('fetchPostsForProfile calls business feed when businessProfileUuid is provided', () async {});
test('loadMorePostsForProfile preserves pagination and de-duplication in business mode', () async {});
```

- [ ] **Step 2: Run tests to verify failure**

Run: `flutter test test/feed_provider_business_mode_test.dart`  
Expected: FAIL with missing profile-aware methods

- [ ] **Step 3: Implement profile-aware provider methods**

```dart
Future<void> fetchPostsForProfile(String token, {String? businessProfileUuid}) async {
  _loading = true;
  _error = null;
  _posts = [];
  notifyListeners();
  final fetchedPosts = businessProfileUuid == null
      ? await FeedService.fetchPosts(token, page: 0)
      : await FeedService.fetchBusinessFeed(token, businessProfileUuid, page: 0);
  _posts = fetchedPosts;
  _currentPage = 0;
  _hasMore = fetchedPosts.length == _pageSize;
  _loading = false;
  notifyListeners();
}
```

- [ ] **Step 4: Keep backward compatibility wrappers**

```dart
Future<void> fetchPosts(String token) => fetchPostsForProfile(token);
Future<void> loadMorePosts(String token) => loadMorePostsForProfile(token);
```

- [ ] **Step 5: Run provider tests**

Run: `flutter test test/feed_provider_business_mode_test.dart`  
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add lib/providers/feed_provider.dart test/feed_provider_business_mode_test.dart
git commit -m "feat(feed): make provider profile-aware for business mode"
```

### Task 3: Wire FeedPage and GalleryPage to profile-aware feed methods

**Files:**
- Modify: `lib/pages/home/home_page.dart`
- Modify: `lib/pages/gallery/gallery_page.dart`
- Create: `test/feed_page_profile_mode_test.dart`
- Create: `test/gallery_page_profile_mode_test.dart`

**Interfaces:**
- Consumes:
  - `PersonProvider.isProfessional`
  - `PersonProvider.activeBusinessProfile?.uuid`
  - `FeedProvider.fetchPostsForProfile(...)`
  - `FeedProvider.loadMorePostsForProfile(...)`
- Produces:
  - Mode-aware fetch and refresh behavior on Feed/Gallery pages

- [ ] **Step 1: Write failing widget tests for profile-aware loading**

```dart
testWidgets('FeedPage loads business feed when active business profile exists', (tester) async {});
testWidgets('GalleryPage refresh uses business feed when in business mode', (tester) async {});
```

- [ ] **Step 2: Run tests to verify failure**

Run: `flutter test test/feed_page_profile_mode_test.dart test/gallery_page_profile_mode_test.dart`  
Expected: FAIL because pages still call `fetchPosts()`/`loadMorePosts()`

- [ ] **Step 3: Update FeedPage initial load, refresh, retry, load-more**

```dart
String? _activeBusinessProfileUuid() =>
    context.read<PersonProvider>().activeBusinessProfile?.uuid;

Future<void> _fetchForCurrentProfile() {
  final token = _token;
  final businessUuid = _activeBusinessProfileUuid();
  return context.read<FeedProvider>().fetchPostsForProfile(
    token,
    businessProfileUuid: businessUuid,
  );
}
```

- [ ] **Step 4: Update GalleryPage load/refresh/load-more similarly**

```dart
final businessUuid = context.read<PersonProvider>().activeBusinessProfile?.uuid;
await context.read<FeedProvider>().fetchPostsForProfile(
  token,
  businessProfileUuid: businessUuid,
);
```

- [ ] **Step 5: Run widget tests**

Run: `flutter test test/feed_page_profile_mode_test.dart test/gallery_page_profile_mode_test.dart`  
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add lib/pages/home/home_page.dart lib/pages/gallery/gallery_page.dart test/feed_page_profile_mode_test.dart test/gallery_page_profile_mode_test.dart
git commit -m "feat(feed): wire feed and gallery pages to active profile mode"
```

### Task 4: Update sidebar + add Team and Followers routes/pages

**Files:**
- Modify: `lib/widgets/sidebar_menu.dart`
- Modify: `lib/main.dart`
- Create: `lib/pages/team/team_page.dart`
- Create: `lib/pages/followers/followers_page.dart`
- Create: `test/sidebar_menu_business_mode_test.dart`

**Interfaces:**
- Consumes:
  - `PersonProvider.isProfessional`
  - `MainLayout(navSection: NavSection.home, currentRoute: ...)`
- Produces:
  - Sidebar menu variants by profile mode
  - New routes: `/team`, `/followers`
  - New page widgets: `TeamPage`, `FollowersPage`

- [ ] **Step 1: Write failing sidebar widget test for business menu items**

```dart
testWidgets('sidebar shows Team and Followers in business mode', (tester) async {
  // pump SidebarContent with mocked PersonProvider.isProfessional = true
  // expect menuTeam/menuFollowers visible, menuFriends/menuMessages hidden
});
```

- [ ] **Step 2: Run test to verify failure**

Run: `flutter test test/sidebar_menu_business_mode_test.dart`  
Expected: FAIL due to current personal-only home list

- [ ] **Step 3: Implement business-mode branch in `_homeItems`**

```dart
if (context.read<PersonProvider>().isProfessional) {
  return [
    _SidebarItem(label: l10n.menuFeed, ...),
    _SidebarItem(label: l10n.menuGallery, ...),
    _SidebarItem(label: l10n.menuProfile, onTap: () => _go(context, '/business-profile'), ...),
    _SidebarItem(label: l10n.menuTeam, onTap: () => _go(context, '/team'), ...),
    _SidebarItem(label: l10n.menuFollowers, onTap: () => _go(context, '/followers'), ...),
    _SidebarItem(label: l10n.menuNotifications, ...),
  ];
}
```

- [ ] **Step 4: Add Team/Followers pages and routes**

```dart
// main.dart routes
'/team': (context) => const TeamPage(),
'/followers': (context) => const FollowersPage(),
```

- [ ] **Step 5: Run sidebar test**

Run: `flutter test test/sidebar_menu_business_mode_test.dart`  
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add lib/widgets/sidebar_menu.dart lib/main.dart lib/pages/team/team_page.dart lib/pages/followers/followers_page.dart test/sidebar_menu_business_mode_test.dart
git commit -m "feat(nav): add business sidebar entries and team/followers routes"
```

### Task 5: Apply business profile imagery in avatar surfaces

**Files:**
- Modify: `lib/widgets/sidebar_menu.dart`
- Modify: `lib/widgets/profile_menu.dart`
- Modify: `lib/widgets/person_avatar_widget.dart`
- Modify: `lib/widgets/post_composer_sheet.dart`
- Create: `test/person_avatar_business_mode_test.dart`

**Interfaces:**
- Consumes:
  - `PersonProvider.activeBusinessProfile?.logo`
  - `PersonProvider.activeBusinessProfile?.businessName`
  - existing avatar fallback assets
- Produces:
  - Unified avatar behavior: business logo in business mode, personal avatar otherwise

- [ ] **Step 1: Write failing widget tests for avatar source switching**

```dart
testWidgets('PersonAvatar uses active business logo in business mode', (tester) async {});
testWidgets('PersonAvatar falls back to personal avatar in personal mode', (tester) async {});
```

- [ ] **Step 2: Run tests to verify failure**

Run: `flutter test test/person_avatar_business_mode_test.dart`  
Expected: FAIL because `PersonAvatar` currently only reads `Person.avatar`

- [ ] **Step 3: Refactor `PersonAvatar` to read `PersonProvider` mode**

```dart
final personProvider = context.watch<PersonProvider>();
final businessLogo = personProvider.activeBusinessProfile?.logo;
final displayUrl = personProvider.isProfessional && (businessLogo?.isNotEmpty ?? false)
    ? businessLogo!
    : person?.avatar;
```

- [ ] **Step 4: Mirror mode-aware avatar logic in `ProfileMenu` and `PostComposerSheet` author row**

```dart
final activeBusiness = context.read<PersonProvider>().activeBusinessProfile;
final displayName = context.read<PersonProvider>().isProfessional
    ? activeBusiness?.businessName ?? _personDisplayName(person)
    : _personDisplayName(person);
```

- [ ] **Step 5: Run avatar tests**

Run: `flutter test test/person_avatar_business_mode_test.dart`  
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add lib/widgets/sidebar_menu.dart lib/widgets/profile_menu.dart lib/widgets/person_avatar_widget.dart lib/widgets/post_composer_sheet.dart test/person_avatar_business_mode_test.dart
git commit -m "feat(ui): show business profile imagery when profile mode is business"
```

### Task 6: Include business profile context when creating posts

**Files:**
- Modify: `lib/widgets/post_composer_sheet.dart`
- Create: `test/post_composer_business_payload_test.dart`

**Interfaces:**
- Consumes:
  - `PersonProvider.isProfessional`
  - `PersonProvider.activeBusinessProfile?.uuid`
  - `FeedProvider.createPost(Map<String, dynamic> data, String token)`
- Produces:
  - Post payload field: `businessProfileUuid` when in business mode
  - Author display name/logo reflecting active business profile

- [ ] **Step 1: Write failing test for post payload in business mode**

```dart
testWidgets('composer includes businessProfileUuid in payload when business mode is active', (tester) async {
  // intercept FeedProvider.createPost call
  // expect payload['businessProfileUuid'] == activeBusinessProfile.uuid
});
```

- [ ] **Step 2: Run test to verify failure**

Run: `flutter test test/post_composer_business_payload_test.dart`  
Expected: FAIL because payload currently has only personal author fields

- [ ] **Step 3: Implement payload enrichment in `_submit`**

```dart
final personProvider = context.read<PersonProvider>();
final activeBusiness = personProvider.activeBusinessProfile;
final isBusiness = personProvider.isProfessional && activeBusiness?.uuid != null;

final payload = {
  'content': content,
  'authorId': _person?.uuid ?? '',
  'authorName': isBusiness ? (activeBusiness!.businessName) : (_person?.fullName ?? ''),
  'authorObjectKey': isBusiness ? (activeBusiness!.logo ?? '') : (_person?.objectKey ?? ''),
  if (isBusiness) 'businessProfileUuid': activeBusiness!.uuid!,
  if (mediaPayload.isNotEmpty) 'media': mediaPayload,
  'mentions': mentionsPayload,
};
```

- [ ] **Step 4: Run test to verify pass**

Run: `flutter test test/post_composer_business_payload_test.dart`  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add lib/widgets/post_composer_sheet.dart test/post_composer_business_payload_test.dart
git commit -m "feat(feed): include active business profile context in post creation payload"
```

### Task 7: Localization keys + generated localization sync + full verification

**Files:**
- Modify: `lib/l10n/app_en.arb`
- Modify: `lib/l10n/app_pt.arb`
- Modify: `lib/l10n/app_pt_BR.arb`
- Modify: `lib/l10n/app_es.arb`
- Modify: `lib/l10n/app_fr.arb`
- Modify: `lib/l10n/app_nl.arb`
- Regenerate: `lib/l10n/app_localizations.dart` and generated locale classes

**Interfaces:**
- Consumes: existing localization generation configured with `flutter: generate: true`
- Produces:
  - `menuTeam`, `menuFollowers`, `businessModeLabel` (if used)
  - Generated getters in `AppLocalizations`

- [ ] **Step 1: Write failing test/assertion for new localization keys**

```dart
testWidgets('business menu labels are available in localizations', (tester) async {
  // Build MaterialApp with delegates
  // expect AppLocalizations.of(context)!.menuTeam is not empty
  // expect AppLocalizations.of(context)!.menuFollowers is not empty
});
```

- [ ] **Step 2: Run test to verify failure**

Run: `flutter test test/sidebar_menu_business_mode_test.dart`  
Expected: FAIL due to missing l10n getters

- [ ] **Step 3: Add translation keys in all `.arb` files and regenerate**

```json
"menuTeam": "Team",
"menuFollowers": "Followers"
```

Run: `flutter gen-l10n`

- [ ] **Step 4: Run focused tests**

Run: `flutter test test/sidebar_menu_business_mode_test.dart test/person_avatar_business_mode_test.dart test/post_composer_business_payload_test.dart`  
Expected: PASS

- [ ] **Step 5: Run project-level quality checks**

Run: `flutter analyze`  
Expected: No new errors

Run: `flutter test`  
Expected: All tests PASS

- [ ] **Step 6: Commit**

```bash
git add lib/l10n/*.arb lib/l10n/app_localizations*.dart
git commit -m "feat(i18n): add business navigation localization keys"
```

## Self-Review Notes

- **Spec coverage:** All approved items map to tasks (business feed endpoint, sidebar business menu, team/followers routes, business imagery, persistence via existing provider state).
- **Placeholder scan:** No TBD/TODO placeholders remain in execution steps; each task has concrete files, commands, and code skeletons.
- **Type consistency:** All new interfaces consistently use `String? businessProfileUuid` on provider-facing methods and `String businessProfileUuid` for service-level required input.
