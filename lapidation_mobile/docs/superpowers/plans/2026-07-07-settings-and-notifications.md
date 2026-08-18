# Mobile Settings & Notifications Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Apply the per-user `settings` object returned by `/workout/api/resource` in the Flutter app (language override, configurable home page, configurable context-menu position, notifications toggle), and add a Settings page plus a Notifications page that navigates to the post a notification came from.

**Architecture:** New `Settings` / `AppNotification` models + static service classes (`SettingsService`, `NotificationService`) following the existing Dio pattern, two new `ChangeNotifier` providers (`SettingsProvider`, `NotificationProvider`) registered in `main.dart`, a data-driven context-menu refactor so the sidebar and a new icons-only bar share one item list, and two new pages (`SettingsPage`, `NotificationsPage`) plus a `PostDetailPage` for notification navigation. Backend (`workout`, `timeline`) is already done — read-only reference.

**Tech Stack:** Flutter, provider, dio, shared_preferences, flutter gen-l10n, flutter_test.

## Global Constraints

- **Only `lapidation_mobile/` may be modified.** `workout/` and `timeline/` are reference-only.
- **DO NOT COMMIT.** The user reviews all changes on the already-created feature branch. Skip any commit step; leave the working tree dirty.
- Theme setting is **intentionally ignored** in the UI for now, but its value must be preserved round-trip when saving (send back the existing `theme` string, default `"Light"` when creating).
- Follow existing patterns: static service classes using `DioClient().dio` + `BaseService.handleDioError`, `ChangeNotifier` providers with `SharedPreferences` persistence, named routes in `main.dart`, `AppColors` styling, l10n via `.arb` files in `lib/l10n/` regenerated with `flutter gen-l10n`.
- All commands run from `/home/erocha/workspace/lapidation_project/lapidation_mobile/`.
- After every task: `flutter analyze` reports no new issues and `flutter test` passes.
- l10n keys must be added to **all six** arb files: `app_en.arb`, `app_pt.arb`, `app_pt_BR.arb`, `app_es.arb`, `app_fr.arb`, `app_nl.arb`, then run `flutter gen-l10n`.

## Backend Contract (reference — already implemented)

`GET /workout/api/resource` (bearer auth) returns:

```json
{
  "countries": [ ... ],
  "settings": {
    "id": 1,
    "uuid": "0d9c…",
    "personId": 42,
    "personUuid": "ab12…",
    "language": "pt_BR",
    "theme": "Light",
    "notificationsEnabled": true,
    "contextMenuPosition": "Left",
    "homePage": "Feed",
    "createdAt": "2026-07-07T10:00:00",
    "updatedAt": "2026-07-07T10:00:00"
  }
}
```

- `settings` is **nullable** (user may not have a row yet).
- `contextMenuPosition` ∈ `"Left" | "Top" | "Right" | "Bottom"` (workout `Position` enum; unknown → treat as Left).
- `GET /workout/api/settings/me` → `SettingsJson`; 404 when none.
- `PUT /workout/api/settings/me` with the full `SettingsJson` body **creates or updates** (persist upsert; `id` absent → insert).
- `GET /timeline/api/notifications?unread_only=<bool>&limit=<n>` → array of:

```json
{
  "uuid": "postUuid:mentionedUuid",
  "notificationType": "Mention",
  "recipientPersonUuid": "…",
  "actorPersonUuid": "…",
  "actorName": "Jane Doe",
  "postUuid": "…",
  "commentUuid": null,
  "entityType": "post",
  "entityUuid": "…",
  "snippet": "…",
  "read": false,
  "createdAt": "2026-07-07T10:00:00",
  "updatedAt": "2026-07-07T10:00:00"
}
```

- `entityType` is lowercase `"post"` or `"comment"`; `postUuid` is set for both types.
- `PUT /timeline/api/notifications/{uuid}/read` body `{"read": true}` marks read (`uuid` is the notification's `uuid` field, which doubles as the idempotency key).
- There is **no** single-post GET endpoint — a post must be resolved by walking `/timeline/api/feed?page=N` (page size 20).

---

### Task 1: Settings model + AppResources.settings

**Files:**
- Create: `lib/models/settings.dart`
- Modify: `lib/models/resource.dart` (add `settings` to `AppResources`)
- Test: `test/settings_model_test.dart`

**Interfaces:**
- Produces: `Settings` (fields `id`, `uuid`, `personId`, `personUuid`, `language`, `theme`, `notificationsEnabled`, `contextMenuPosition`, `homePage`, `fromJson`, `toJson`, `copyWith`), enums `ContextMenuPosition {left, top, right, bottom}` (`apiValue`, `fromApi`) and `HomePageOption {feed, gallery}` (`apiValue`, `route`, `fromApi`), and `AppResources.settings` (`Settings?`).

- [ ] **Step 1: Write the failing test**

```dart
// test/settings_model_test.dart
import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/models/resource.dart';
import 'package:lapidation_mobile/models/settings.dart';

void main() {
  final fullJson = {
    'id': 1,
    'uuid': '0d9c',
    'personId': 42,
    'personUuid': 'ab12',
    'language': 'pt_BR',
    'theme': 'Light',
    'notificationsEnabled': true,
    'contextMenuPosition': 'Top',
    'homePage': 'Gallery',
    'createdAt': '2026-07-07T10:00:00',
    'updatedAt': '2026-07-07T10:00:00',
  };

  group('Settings.fromJson', () {
    test('parses all fields', () {
      final s = Settings.fromJson(fullJson);
      expect(s.id, 1);
      expect(s.uuid, '0d9c');
      expect(s.personId, 42);
      expect(s.personUuid, 'ab12');
      expect(s.language, 'pt_BR');
      expect(s.theme, 'Light');
      expect(s.notificationsEnabled, true);
      expect(s.contextMenuPosition, ContextMenuPosition.top);
      expect(s.homePage, HomePageOption.gallery);
    });

    test('falls back to defaults on unknown enum values', () {
      final s = Settings.fromJson({
        ...fullJson,
        'contextMenuPosition': 'Diagonal',
        'homePage': 'Dashboard',
      });
      expect(s.contextMenuPosition, ContextMenuPosition.left);
      expect(s.homePage, HomePageOption.feed);
    });
  });

  test('toJson round-trips and uses API enum values', () {
    final s = Settings.fromJson(fullJson);
    final json = s.toJson();
    expect(json['contextMenuPosition'], 'Top');
    expect(json['homePage'], 'Gallery');
    expect(json['personId'], 42);
    expect(json['personUuid'], 'ab12');
    expect(json['notificationsEnabled'], true);
    expect(Settings.fromJson(json).contextMenuPosition, ContextMenuPosition.top);
  });

  test('toJson omits id and uuid when null (create flow)', () {
    final s = Settings(
      personId: 42,
      personUuid: 'ab12',
      language: 'en',
      theme: 'Light',
      notificationsEnabled: true,
      contextMenuPosition: ContextMenuPosition.left,
      homePage: HomePageOption.feed,
    );
    final json = s.toJson();
    expect(json.containsKey('id'), false);
    expect(json.containsKey('uuid'), false);
  });

  group('enums', () {
    test('ContextMenuPosition.fromApi is case-insensitive', () {
      expect(ContextMenuPosition.fromApi('right'), ContextMenuPosition.right);
      expect(ContextMenuPosition.fromApi('BOTTOM'), ContextMenuPosition.bottom);
      expect(ContextMenuPosition.fromApi(null), ContextMenuPosition.left);
    });

    test('HomePageOption routes', () {
      expect(HomePageOption.feed.route, '/home');
      expect(HomePageOption.gallery.route, '/gallery');
      expect(HomePageOption.fromApi('feed'), HomePageOption.feed);
      expect(HomePageOption.fromApi(null), HomePageOption.feed);
    });
  });

  test('AppResources parses optional settings', () {
    final withSettings =
        AppResources.fromJson({'countries': [], 'settings': fullJson});
    expect(withSettings.settings, isNotNull);
    expect(withSettings.settings!.homePage, HomePageOption.gallery);

    final without = AppResources.fromJson({'countries': []});
    expect(without.settings, isNull);

    expect(withSettings.toJson()['settings'], isNotNull);
    expect(without.toJson()['settings'], isNull);
  });
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `flutter test test/settings_model_test.dart`
Expected: FAIL — `Error: Couldn't resolve the package 'lapidation_mobile/models/settings.dart'` (file doesn't exist).

- [ ] **Step 3: Implement the model**

```dart
// lib/models/settings.dart

/// Where the context menu (sidebar / submenu) is rendered.
/// Mirrors the workout service's `Position` enum.
enum ContextMenuPosition {
  left('Left'),
  top('Top'),
  right('Right'),
  bottom('Bottom');

  final String apiValue;
  const ContextMenuPosition(this.apiValue);

  static ContextMenuPosition fromApi(String? value) {
    return ContextMenuPosition.values.firstWhere(
      (p) => p.apiValue.toLowerCase() == value?.toLowerCase(),
      orElse: () => ContextMenuPosition.left,
    );
  }
}

/// Which page is shown right after login.
enum HomePageOption {
  feed('Feed', '/home'),
  gallery('Gallery', '/gallery');

  final String apiValue;
  final String route;
  const HomePageOption(this.apiValue, this.route);

  static HomePageOption fromApi(String? value) {
    return HomePageOption.values.firstWhere(
      (o) => o.apiValue.toLowerCase() == value?.toLowerCase(),
      orElse: () => HomePageOption.feed,
    );
  }
}

class Settings {
  final int? id;
  final String? uuid;
  final int personId;
  final String personUuid;
  final String language;
  final String theme;
  final bool notificationsEnabled;
  final ContextMenuPosition contextMenuPosition;
  final HomePageOption homePage;

  Settings({
    this.id,
    this.uuid,
    required this.personId,
    required this.personUuid,
    required this.language,
    required this.theme,
    required this.notificationsEnabled,
    required this.contextMenuPosition,
    required this.homePage,
  });

  factory Settings.fromJson(Map<String, dynamic> json) {
    return Settings(
      id: json['id'] as int?,
      uuid: json['uuid'] as String?,
      personId: json['personId'] as int,
      personUuid: json['personUuid'] as String,
      language: json['language'] as String,
      theme: json['theme'] as String,
      notificationsEnabled: json['notificationsEnabled'] as bool,
      contextMenuPosition:
          ContextMenuPosition.fromApi(json['contextMenuPosition'] as String?),
      homePage: HomePageOption.fromApi(json['homePage'] as String?),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      if (id != null) 'id': id,
      if (uuid != null) 'uuid': uuid,
      'personId': personId,
      'personUuid': personUuid,
      'language': language,
      'theme': theme,
      'notificationsEnabled': notificationsEnabled,
      'contextMenuPosition': contextMenuPosition.apiValue,
      'homePage': homePage.apiValue,
    };
  }

  Settings copyWith({
    String? language,
    String? theme,
    bool? notificationsEnabled,
    ContextMenuPosition? contextMenuPosition,
    HomePageOption? homePage,
  }) {
    return Settings(
      id: id,
      uuid: uuid,
      personId: personId,
      personUuid: personUuid,
      language: language ?? this.language,
      theme: theme ?? this.theme,
      notificationsEnabled: notificationsEnabled ?? this.notificationsEnabled,
      contextMenuPosition: contextMenuPosition ?? this.contextMenuPosition,
      homePage: homePage ?? this.homePage,
    );
  }
}
```

In `lib/models/resource.dart`, add the import at the top and extend `AppResources`:

```dart
import 'settings.dart';
```

```dart
class AppResources {
  final List<Country> countries;
  final Settings? settings;

  AppResources({
    required this.countries,
    this.settings,
  });

  factory AppResources.fromJson(Map<String, dynamic> json) {
    return AppResources(
      countries: (json['countries'] as List<dynamic>?)
              ?.map((e) => Country.fromJson(e as Map<String, dynamic>))
              .toList() ??
          [],
      settings: json['settings'] != null
          ? Settings.fromJson(json['settings'] as Map<String, dynamic>)
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'countries': countries.map((e) => e.toJson()).toList(),
      'settings': settings?.toJson(),
    };
  }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `flutter test test/settings_model_test.dart` → PASS.
Run: `flutter analyze` → no new issues.

---

### Task 2: Settings-language → Locale mapper

**Files:**
- Create: `lib/utils/settings_locale_mapper.dart`
- Test: `test/settings_locale_mapper_test.dart`

**Interfaces:**
- Consumes: `LocaleProvider.supportedLocales` (`lib/providers/locale_provider.dart`).
- Produces: `Locale? localeFromSettingsLanguage(String? language)` — top-level function.

- [ ] **Step 1: Write the failing test**

```dart
// test/settings_locale_mapper_test.dart
import 'dart:ui';

import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/utils/settings_locale_mapper.dart';

void main() {
  test('maps exact supported tags', () {
    expect(localeFromSettingsLanguage('en'), const Locale('en'));
    expect(localeFromSettingsLanguage('pt_BR'), const Locale('pt', 'BR'));
    expect(localeFromSettingsLanguage('es'), const Locale('es'));
  });

  test('is tolerant to separators and casing', () {
    expect(localeFromSettingsLanguage('pt-BR'), const Locale('pt', 'BR'));
    expect(localeFromSettingsLanguage('PT_br'), const Locale('pt', 'BR'));
    expect(localeFromSettingsLanguage('EN'), const Locale('en'));
  });

  test('falls back to language-only match', () {
    expect(localeFromSettingsLanguage('pt'), const Locale('pt', 'BR'));
    expect(localeFromSettingsLanguage('pt_PT'), const Locale('pt', 'BR'));
  });

  test('returns null for unsupported or empty values', () {
    expect(localeFromSettingsLanguage('de'), isNull);
    expect(localeFromSettingsLanguage(''), isNull);
    expect(localeFromSettingsLanguage('  '), isNull);
    expect(localeFromSettingsLanguage(null), isNull);
  });
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `flutter test test/settings_locale_mapper_test.dart`
Expected: FAIL — package URI can't be resolved.

- [ ] **Step 3: Implement the mapper**

```dart
// lib/utils/settings_locale_mapper.dart
import 'dart:ui';

import '../providers/locale_provider.dart';

/// Resolves a language tag coming from the settings API ("en", "pt_BR",
/// "pt-BR", "pt") to one of the app's supported locales.
/// Returns null when the tag is empty or not supported.
Locale? localeFromSettingsLanguage(String? language) {
  if (language == null || language.trim().isEmpty) return null;

  final normalized = language.trim().replaceAll('-', '_');
  final parts = normalized.split('_');
  final lang = parts[0].toLowerCase();
  final country = parts.length > 1 ? parts[1].toUpperCase() : null;

  for (final locale in LocaleProvider.supportedLocales) {
    if (locale.languageCode == lang && locale.countryCode == country) {
      return locale;
    }
  }
  // Language-only fallback (e.g. "pt" or "pt_PT" -> pt_BR).
  for (final locale in LocaleProvider.supportedLocales) {
    if (locale.languageCode == lang) return locale;
  }
  return null;
}
```

Note: `Locale('en').countryCode` is `null`, so the exact loop already matches country-less tags.

- [ ] **Step 4: Run tests to verify they pass**

Run: `flutter test test/settings_locale_mapper_test.dart` → PASS.
Run: `flutter analyze` → no new issues.

---

### Task 3: SettingsService + ApiConfig endpoint

**Files:**
- Modify: `lib/config/api_config.dart`
- Create: `lib/services/settings_service.dart`

**Interfaces:**
- Consumes: `Settings` from Task 1; `DioClient`, `BaseService.handleDioError`, `AppException` (existing).
- Produces: `SettingsService.fetchMySettings(String token) → Future<Settings>` and `SettingsService.updateMySettings(Settings settings, String token) → Future<Settings>`; `ApiConfig.settingsMeEndpoint`.

No unit test — pure network wrapper, matching the untested pattern of `ResourceService`/`FeedService`.

- [ ] **Step 1: Add the endpoint constant**

In `lib/config/api_config.dart`, below `resourceEndpoint`:

```dart
   static const String settingsMeEndpoint = '/workout/api/settings/me';
```

- [ ] **Step 2: Implement the service**

```dart
// lib/services/settings_service.dart
import 'package:dio/dio.dart';
import '../config/api_config.dart';
import '../models/settings.dart';
import '../utils/dio_client.dart';
import 'base_service.dart';

class SettingsService {
  static final _dio = DioClient().dio;

  static Future<Settings> fetchMySettings(String token) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(ApiConfig.settingsMeEndpoint);
      if (response.statusCode == 200) {
        return Settings.fromJson(response.data as Map<String, dynamic>);
      }
      throw AppException(
        statusCode: response.statusCode ?? 500,
        message: response.data?['message'] ?? 'Failed to fetch settings',
      );
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to fetch settings');
    }
  }

  static Future<Settings> updateMySettings(
      Settings settings, String token) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.put(
        ApiConfig.settingsMeEndpoint,
        data: settings.toJson(),
      );
      if (response.statusCode == 200) {
        return Settings.fromJson(response.data as Map<String, dynamic>);
      }
      throw AppException(
        statusCode: response.statusCode ?? 500,
        message: response.data?['message'] ?? 'Failed to update settings',
      );
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to update settings');
    }
  }
}
```

- [ ] **Step 3: Verify**

Run: `flutter analyze` → no new issues. Run: `flutter test` → all pass.

---

### Task 4: SettingsProvider + registration + logout clearing

**Files:**
- Create: `lib/providers/settings_provider.dart`
- Modify: `lib/main.dart` (register provider)
- Modify: `lib/widgets/sidebar_menu.dart` (clear on logout)
- Modify: `lib/widgets/app_header.dart` (clear on logout)

**Interfaces:**
- Consumes: `Settings`, `ContextMenuPosition`, `HomePageOption` (Task 1); `SettingsService` (Task 3).
- Produces: `SettingsProvider` with `settings` (`Settings?`), `saving` (`bool`), `error` (`String?`), `contextMenuPosition` (defaults `left`), `homePage` (defaults `feed`), `notificationsEnabled` (defaults `true`), `applyFromResources(Settings?)`, `fetchMySettings(String token)`, `save(Settings, String token) → Future<bool>`, `clear()`.

- [ ] **Step 1: Implement the provider**

```dart
// lib/providers/settings_provider.dart
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../models/settings.dart';
import '../services/base_service.dart';
import '../services/settings_service.dart';

class SettingsProvider extends ChangeNotifier {
  static const String _storageKey = 'user_settings';

  Settings? _settings;
  bool _saving = false;
  String? _error;

  Settings? get settings => _settings;
  bool get saving => _saving;
  String? get error => _error;

  ContextMenuPosition get contextMenuPosition =>
      _settings?.contextMenuPosition ?? ContextMenuPosition.left;
  HomePageOption get homePage => _settings?.homePage ?? HomePageOption.feed;
  bool get notificationsEnabled => _settings?.notificationsEnabled ?? true;

  SettingsProvider() {
    _loadFromStorage();
  }

  Future<void> _loadFromStorage() async {
    final prefs = await SharedPreferences.getInstance();
    final settingsJson = prefs.getString(_storageKey);
    if (settingsJson != null) {
      try {
        final data = jsonDecode(settingsJson) as Map<String, dynamic>;
        _settings = Settings.fromJson(data);
        notifyListeners();
      } catch (_) {
        await prefs.remove(_storageKey);
      }
    }
  }

  Future<void> _saveToStorage(Settings settings) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_storageKey, jsonEncode(settings.toJson()));
  }

  /// Adopts the settings object embedded in the /resource response.
  Future<void> applyFromResources(Settings? settings) async {
    if (settings == null) return;
    _settings = settings;
    await _saveToStorage(settings);
    notifyListeners();
  }

  /// Refreshes from GET /settings/me. Keeps current state on failure
  /// (e.g. 404 when the user has no settings row yet).
  Future<void> fetchMySettings(String token) async {
    try {
      _settings = await SettingsService.fetchMySettings(token);
      await _saveToStorage(_settings!);
      _error = null;
      notifyListeners();
    } catch (_) {
      // Keep whatever we already have.
    }
  }

  /// Persists via PUT /settings/me (creates when no row exists yet).
  Future<bool> save(Settings settings, String token) async {
    _saving = true;
    _error = null;
    notifyListeners();

    try {
      _settings = await SettingsService.updateMySettings(settings, token);
      await _saveToStorage(_settings!);
      _saving = false;
      notifyListeners();
      return true;
    } on AppException catch (e) {
      _error = e.message;
      _saving = false;
      notifyListeners();
      return false;
    } catch (_) {
      _error = 'Failed to save settings. Please try again.';
      _saving = false;
      notifyListeners();
      return false;
    }
  }

  /// Clear settings (e.g., on logout).
  Future<void> clear() async {
    _settings = null;
    _error = null;
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove(_storageKey);
    notifyListeners();
  }
}
```

- [ ] **Step 2: Register in main.dart**

Add import `'providers/settings_provider.dart'` and, inside the `MultiProvider` providers list after `FeedProvider`:

```dart
        ChangeNotifierProvider(create: (_) => SettingsProvider()),
```

- [ ] **Step 3: Clear settings on logout**

In `lib/widgets/sidebar_menu.dart` logout `onTap` (the pinned-bottom `_SidebarItem` with `Icons.logout`), after `context.read<PersonProvider>().clear();` add:

```dart
            context.read<SettingsProvider>().clear();
```

with import `'../providers/settings_provider.dart'`.

In `lib/widgets/app_header.dart` `_ProfileMenu.onSelected` `case 'logout':`, after `context.read<PersonProvider>().clear();` add the same line and import.

- [ ] **Step 4: Verify**

Run: `flutter analyze` → no new issues. Run: `flutter test` → all pass.

---

### Task 5: Apply settings at login (language override + configured home page)

**Files:**
- Modify: `lib/pages/sign_in/sign_in_page.dart`
- Modify: `lib/pages/home/home_page.dart` (fallback path)

**Interfaces:**
- Consumes: `ResourceProvider.fetchResources`, `AppResources.settings` (Task 1), `SettingsProvider.applyFromResources`/`homePage` (Task 4), `localeFromSettingsLanguage` (Task 2), `LocaleProvider.setLocale`.

- [ ] **Step 1: Fetch + apply settings before navigating after login**

In `lib/pages/sign_in/sign_in_page.dart` add imports:

```dart
import '../../providers/locale_provider.dart';
import '../../providers/resource_provider.dart';
import '../../providers/settings_provider.dart';
import '../../utils/settings_locale_mapper.dart';
```

Add this private method to `_SignInPageState`:

```dart
  /// Fetches resources, applies the user's server-side settings (language
  /// override, configured home page) and returns the initial route.
  Future<String> _applyUserSettings(String token) async {
    final resourceProvider = context.read<ResourceProvider>();
    final settingsProvider = context.read<SettingsProvider>();
    final localeProvider = context.read<LocaleProvider>();

    await resourceProvider.fetchResources(token);
    final settings = resourceProvider.resources?.settings;
    await settingsProvider.applyFromResources(settings);

    final locale = localeFromSettingsLanguage(settings?.language);
    if (locale != null && locale != localeProvider.locale) {
      await localeProvider.setLocale(locale);
    }

    return settingsProvider.homePage.route;
  }
```

In `_handleAuthCheck`, replace:

```dart
      if (mounted) {
        Navigator.of(context).pushReplacementNamed('/home');
        return;
      }
```

with:

```dart
      if (!mounted) return;
      final route = await _applyUserSettings(authProvider.auth!.accessToken);
      if (mounted) {
        Navigator.of(context).pushReplacementNamed(route);
        return;
      }
```

In `_handleSignIn`, replace:

```dart
      if (mounted) {
        Navigator.of(context).pushReplacementNamed('/home');
      }
```

with:

```dart
      if (!mounted) return;
      final route = await _applyUserSettings(authProvider.auth!.accessToken);
      if (mounted) {
        Navigator.of(context).pushReplacementNamed(route);
      }
```

- [ ] **Step 2: Push settings into the provider on the home-page fallback fetch**

In `lib/pages/home/home_page.dart` `_fetchInitialData`, replace:

```dart
    if (resourceProvider.resources == null) {
      resourceProvider.fetchResources(token);
    }
```

with:

```dart
    if (resourceProvider.resources == null) {
      resourceProvider.fetchResources(token).then((ok) {
        if (ok && mounted) {
          context
              .read<SettingsProvider>()
              .applyFromResources(resourceProvider.resources?.settings);
        }
      });
    }
```

and add import `'../../providers/settings_provider.dart'`.

- [ ] **Step 3: Verify**

Run: `flutter analyze` → no new issues. Run: `flutter test` → all pass.
Behavior note: if `/resource` fails or `settings` is null, `_applyUserSettings` returns `/home` (default `HomePageOption.feed`) and the language is untouched — same as today.

---

### Task 6: Settings page + route + header entry point + l10n

**Files:**
- Create: `lib/pages/settings/settings_page.dart`
- Modify: `lib/main.dart` (route `/settings`)
- Modify: `lib/widgets/app_header.dart` (profile menu → navigate)
- Modify: all six `lib/l10n/app_*.arb` files

**Interfaces:**
- Consumes: `SettingsProvider` (Task 4), `Settings`/enums (Task 1), `localeFromSettingsLanguage` (Task 2), `LocaleProvider`, `PersonProvider.person` (`id`, `uuid`), `AuthProvider.auth.accessToken`, `MainLayout`.
- Produces: route `'/settings'` → `SettingsPage`.

- [ ] **Step 1: Add l10n keys**

`lib/l10n/app_en.arb` (before the closing `}`):

```json
  "settingsTitle": "Settings",
  "settingsHomePage": "Home page",
  "settingsHomePageFeed": "Feed",
  "settingsHomePageGallery": "Gallery",
  "settingsContextMenuPosition": "Menu position",
  "settingsPositionLeft": "Left",
  "settingsPositionTop": "Top",
  "settingsPositionRight": "Right",
  "settingsPositionBottom": "Bottom",
  "settingsNotificationsEnabled": "Enable notifications",
  "settingsSave": "Save",
  "settingsSaved": "Settings saved",
  "settingsSaveFailed": "Failed to save settings"
```

`app_pt.arb` **and** `app_pt_BR.arb`:

```json
  "settingsTitle": "Configurações",
  "settingsHomePage": "Página inicial",
  "settingsHomePageFeed": "Feed",
  "settingsHomePageGallery": "Galeria",
  "settingsContextMenuPosition": "Posição do menu",
  "settingsPositionLeft": "Esquerda",
  "settingsPositionTop": "Topo",
  "settingsPositionRight": "Direita",
  "settingsPositionBottom": "Inferior",
  "settingsNotificationsEnabled": "Ativar notificações",
  "settingsSave": "Salvar",
  "settingsSaved": "Configurações salvas",
  "settingsSaveFailed": "Falha ao salvar as configurações"
```

`app_es.arb`:

```json
  "settingsTitle": "Configuración",
  "settingsHomePage": "Página de inicio",
  "settingsHomePageFeed": "Feed",
  "settingsHomePageGallery": "Galería",
  "settingsContextMenuPosition": "Posición del menú",
  "settingsPositionLeft": "Izquierda",
  "settingsPositionTop": "Arriba",
  "settingsPositionRight": "Derecha",
  "settingsPositionBottom": "Abajo",
  "settingsNotificationsEnabled": "Activar notificaciones",
  "settingsSave": "Guardar",
  "settingsSaved": "Configuración guardada",
  "settingsSaveFailed": "Error al guardar la configuración"
```

`app_fr.arb`:

```json
  "settingsTitle": "Paramètres",
  "settingsHomePage": "Page d'accueil",
  "settingsHomePageFeed": "Fil",
  "settingsHomePageGallery": "Galerie",
  "settingsContextMenuPosition": "Position du menu",
  "settingsPositionLeft": "Gauche",
  "settingsPositionTop": "Haut",
  "settingsPositionRight": "Droite",
  "settingsPositionBottom": "Bas",
  "settingsNotificationsEnabled": "Activer les notifications",
  "settingsSave": "Enregistrer",
  "settingsSaved": "Paramètres enregistrés",
  "settingsSaveFailed": "Échec de l'enregistrement des paramètres"
```

`app_nl.arb`:

```json
  "settingsTitle": "Instellingen",
  "settingsHomePage": "Startpagina",
  "settingsHomePageFeed": "Feed",
  "settingsHomePageGallery": "Galerij",
  "settingsContextMenuPosition": "Menupositie",
  "settingsPositionLeft": "Links",
  "settingsPositionTop": "Boven",
  "settingsPositionRight": "Rechts",
  "settingsPositionBottom": "Onder",
  "settingsNotificationsEnabled": "Meldingen inschakelen",
  "settingsSave": "Opslaan",
  "settingsSaved": "Instellingen opgeslagen",
  "settingsSaveFailed": "Instellingen opslaan mislukt"
```

Run: `flutter gen-l10n` → regenerates `lib/l10n/app_localizations*.dart` without errors.

- [ ] **Step 2: Implement the page**

```dart
// lib/pages/settings/settings_page.dart
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../models/settings.dart';
import '../../providers/auth_provider.dart';
import '../../providers/locale_provider.dart';
import '../../providers/person_provider.dart';
import '../../providers/settings_provider.dart';
import '../../utils/settings_locale_mapper.dart';
import '../../widgets/main_layout.dart';

class SettingsPage extends StatefulWidget {
  const SettingsPage({super.key});

  @override
  State<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  String? _language;
  HomePageOption _homePage = HomePageOption.feed;
  ContextMenuPosition _position = ContextMenuPosition.left;
  bool _notificationsEnabled = true;
  bool _initialized = false;

  String get _token => context.read<AuthProvider>().auth?.accessToken ?? '';

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _load());
  }

  Future<void> _load() async {
    final settingsProvider = context.read<SettingsProvider>();
    final token = _token;
    if (token.isNotEmpty) {
      await settingsProvider.fetchMySettings(token);
    }
    if (!mounted) return;
    _seedFields(settingsProvider.settings);
  }

  void _seedFields(Settings? settings) {
    final localeProvider = context.read<LocaleProvider>();
    setState(() {
      _language = settings?.language ?? localeProvider.currentLocaleKey;
      _homePage = settings?.homePage ?? HomePageOption.feed;
      _position = settings?.contextMenuPosition ?? ContextMenuPosition.left;
      _notificationsEnabled = settings?.notificationsEnabled ?? true;
      _initialized = true;
    });
  }

  Future<void> _save() async {
    final l10n = AppLocalizations.of(context)!;
    final settingsProvider = context.read<SettingsProvider>();
    final localeProvider = context.read<LocaleProvider>();
    final person = context.read<PersonProvider>().person;
    final base = settingsProvider.settings;

    if (person == null && base == null) return;

    final settings = Settings(
      id: base?.id,
      uuid: base?.uuid,
      personId: base?.personId ?? person!.id,
      personUuid: base?.personUuid ?? person!.uuid,
      language: _language ?? 'en',
      theme: base?.theme ?? 'Light',
      notificationsEnabled: _notificationsEnabled,
      contextMenuPosition: _position,
      homePage: _homePage,
    );

    final ok = await settingsProvider.save(settings, _token);
    if (!mounted) return;

    if (ok) {
      final locale = localeFromSettingsLanguage(_language);
      if (locale != null && locale != localeProvider.locale) {
        await localeProvider.setLocale(locale);
      }
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(l10n.settingsSaved)),
      );
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(settingsProvider.error ?? l10n.settingsSaveFailed),
          backgroundColor: AppColors.danger,
        ),
      );
    }
  }

  String _positionLabel(ContextMenuPosition p, AppLocalizations l10n) {
    switch (p) {
      case ContextMenuPosition.left:
        return l10n.settingsPositionLeft;
      case ContextMenuPosition.top:
        return l10n.settingsPositionTop;
      case ContextMenuPosition.right:
        return l10n.settingsPositionRight;
      case ContextMenuPosition.bottom:
        return l10n.settingsPositionBottom;
    }
  }

  String _homePageLabel(HomePageOption o, AppLocalizations l10n) {
    switch (o) {
      case HomePageOption.feed:
        return l10n.settingsHomePageFeed;
      case HomePageOption.gallery:
        return l10n.settingsHomePageGallery;
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final settingsProvider = context.watch<SettingsProvider>();

    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/settings',
      body: !_initialized
          ? const Center(
              child: CircularProgressIndicator(color: AppColors.primary))
          : SingleChildScrollView(
              padding: const EdgeInsets.all(16),
              child: Center(
                child: ConstrainedBox(
                  constraints: const BoxConstraints(maxWidth: 600),
                  child: Card(
                    color: Colors.white,
                    child: Padding(
                      padding: const EdgeInsets.all(16),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          Text(
                            l10n.settingsTitle,
                            style: const TextStyle(
                              fontSize: 20,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                          const SizedBox(height: 24),

                          // Language
                          DropdownButtonFormField<String>(
                            initialValue: _language,
                            decoration: InputDecoration(
                              labelText: l10n.menuLanguage,
                              border: const OutlineInputBorder(),
                            ),
                            items: LocaleProvider.localeNames.entries
                                .map((e) => DropdownMenuItem(
                                      value: e.key,
                                      child: Text(e.value),
                                    ))
                                .toList(),
                            onChanged: (v) => setState(() => _language = v),
                          ),
                          const SizedBox(height: 16),

                          // Home page
                          DropdownButtonFormField<HomePageOption>(
                            initialValue: _homePage,
                            decoration: InputDecoration(
                              labelText: l10n.settingsHomePage,
                              border: const OutlineInputBorder(),
                            ),
                            items: HomePageOption.values
                                .map((o) => DropdownMenuItem(
                                      value: o,
                                      child: Text(_homePageLabel(o, l10n)),
                                    ))
                                .toList(),
                            onChanged: (v) => setState(
                                () => _homePage = v ?? HomePageOption.feed),
                          ),
                          const SizedBox(height: 16),

                          // Context menu position
                          DropdownButtonFormField<ContextMenuPosition>(
                            initialValue: _position,
                            decoration: InputDecoration(
                              labelText: l10n.settingsContextMenuPosition,
                              border: const OutlineInputBorder(),
                            ),
                            items: ContextMenuPosition.values
                                .map((p) => DropdownMenuItem(
                                      value: p,
                                      child: Text(_positionLabel(p, l10n)),
                                    ))
                                .toList(),
                            onChanged: (v) => setState(() =>
                                _position = v ?? ContextMenuPosition.left),
                          ),
                          const SizedBox(height: 8),

                          // Notifications
                          SwitchListTile(
                            title: Text(l10n.settingsNotificationsEnabled),
                            value: _notificationsEnabled,
                            activeThumbColor: AppColors.primary,
                            contentPadding: EdgeInsets.zero,
                            onChanged: (v) =>
                                setState(() => _notificationsEnabled = v),
                          ),
                          const SizedBox(height: 16),

                          // Save
                          SizedBox(
                            height: 48,
                            child: ElevatedButton(
                              onPressed:
                                  settingsProvider.saving ? null : _save,
                              style: ElevatedButton.styleFrom(
                                backgroundColor: AppColors.primary,
                                disabledBackgroundColor:
                                    AppColors.primaryDisabled,
                                foregroundColor: Colors.white,
                                shape: RoundedRectangleBorder(
                                  borderRadius: BorderRadius.circular(4),
                                ),
                              ),
                              child: settingsProvider.saving
                                  ? const SizedBox(
                                      width: 24,
                                      height: 24,
                                      child: CircularProgressIndicator(
                                        color: Colors.white,
                                        strokeWidth: 2,
                                      ),
                                    )
                                  : Text(
                                      l10n.settingsSave,
                                      style: const TextStyle(
                                        fontSize: 16,
                                        fontWeight: FontWeight.w600,
                                      ),
                                    ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ),
            ),
    );
  }
}
```

- [ ] **Step 3: Register the route and header entry point**

`lib/main.dart`: import `'pages/settings/settings_page.dart'` and add to `routes`:

```dart
              '/settings': (context) => const SettingsPage(),
```

`lib/widgets/app_header.dart` `_ProfileMenu.onSelected` — replace:

```dart
          case 'settings':
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text(l10n.comingSoon)),
            );
            break;
```

with:

```dart
          case 'settings':
            Navigator.of(context).pushNamed('/settings');
            break;
```

- [ ] **Step 4: Verify**

Run: `flutter analyze` → no new issues. Run: `flutter test` → all pass.

---

### Task 7: Extract context-menu items (data-driven sidebar)

**Files:**
- Create: `lib/config/context_menu_items.dart`
- Modify: `lib/widgets/sidebar_menu.dart`

**Interfaces:**
- Consumes: `NavSection`, `AppLocalizations`.
- Produces: `ContextMenuItem` (`icon`, `activeIcon`, `label`, `route` — `route == null` means "coming soon"), `sectionMenuItems(NavSection, AppLocalizations) → List<ContextMenuItem>`, `alwaysMenuItems(AppLocalizations) → List<ContextMenuItem>`. Task 8's icon bar consumes exactly these.

Note: the notifications item keeps `route: null` (coming soon) in this task; Task 11 flips it to `'/notifications'`.

- [ ] **Step 1: Create the shared item definitions**

```dart
// lib/config/context_menu_items.dart
import 'package:flutter/material.dart';

import '../l10n/app_localizations.dart';
import 'nav_section.dart';

/// One entry of the context menu (sidebar drawer / panel / icon bar).
class ContextMenuItem {
  final IconData icon;
  final IconData activeIcon;
  final String label;

  /// Named route to navigate to, or null when the feature is not
  /// implemented yet (shows the "coming soon" snackbar).
  final String? route;

  const ContextMenuItem({
    required this.icon,
    required this.activeIcon,
    required this.label,
    this.route,
  });
}

/// Section-specific items — mirrors the previous hardcoded sidebar lists.
List<ContextMenuItem> sectionMenuItems(
    NavSection section, AppLocalizations l10n) {
  switch (section) {
    case NavSection.home:
      return [
        ContextMenuItem(
            icon: Icons.home_outlined,
            activeIcon: Icons.home,
            label: l10n.menuHome,
            route: '/home'),
        ContextMenuItem(
            icon: Icons.person_outline,
            activeIcon: Icons.person,
            label: l10n.menuProfile,
            route: '/profile'),
        ContextMenuItem(
            icon: Icons.people_outline,
            activeIcon: Icons.people,
            label: l10n.menuFriends,
            route: '/friends'),
        ContextMenuItem(
            icon: Icons.chat_bubble_outline,
            activeIcon: Icons.chat_bubble,
            label: l10n.menuMessages),
        ContextMenuItem(
            icon: Icons.notifications_outlined,
            activeIcon: Icons.notifications,
            label: l10n.menuNotifications),
      ];
    case NavSection.gallery:
      return [
        ContextMenuItem(
            icon: Icons.image_outlined,
            activeIcon: Icons.image,
            label: 'Gallery',
            route: '/gallery'),
        ContextMenuItem(
            icon: Icons.person_outline,
            activeIcon: Icons.person,
            label: l10n.menuProfile,
            route: '/profile'),
        ContextMenuItem(
            icon: Icons.people_outline,
            activeIcon: Icons.people,
            label: l10n.menuFriends,
            route: '/friends'),
      ];
    case NavSection.workout:
      return [
        ContextMenuItem(
            icon: Icons.fitness_center_outlined,
            activeIcon: Icons.fitness_center,
            label: l10n.menuWorkouts,
            route: '/workouts'),
        ContextMenuItem(
            icon: Icons.list_outlined,
            activeIcon: Icons.list,
            label: l10n.menuExercises,
            route: '/exercises'),
        ContextMenuItem(
            icon: Icons.bar_chart_outlined,
            activeIcon: Icons.bar_chart,
            label: l10n.menuWorkoutSessions,
            route: '/workout-sessions'),
        ContextMenuItem(
            icon: Icons.trending_up_outlined,
            activeIcon: Icons.trending_up,
            label: l10n.menuEvolution,
            route: '/evolution'),
      ];
  }
}

/// Items shown for every section (after the divider).
List<ContextMenuItem> alwaysMenuItems(AppLocalizations l10n) {
  return [
    ContextMenuItem(
        icon: Icons.settings_outlined,
        activeIcon: Icons.settings,
        label: l10n.menuSettings,
        route: '/settings'),
  ];
}
```

- [ ] **Step 2: Refactor SidebarContent to consume the shared items**

In `lib/widgets/sidebar_menu.dart` add import `'../config/context_menu_items.dart'`. Replace the four methods `_buildSectionItems`, `_homeItems`, `_galleryItems`, `_workoutItems` with:

```dart
  List<Widget> _buildSectionItems(
      BuildContext context, AppLocalizations l10n) {
    return sectionMenuItems(navSection, l10n)
        .map((item) => _buildItem(context, l10n, item))
        .toList();
  }

  Widget _buildItem(
      BuildContext context, AppLocalizations l10n, ContextMenuItem item) {
    return _SidebarItem(
      icon: item.icon,
      activeIcon: item.activeIcon,
      label: item.label,
      isActive: item.route != null && currentRoute == item.route,
      isCollapsed: isCollapsed,
      onTap: item.route == null
          ? () => _showComingSoon(context, l10n)
          : () => _go(context, item.route!),
    );
  }
```

In `_buildAlwaysItems`, replace the hardcoded settings `_SidebarItem` (the one with `Icons.settings_outlined`) with:

```dart
      ...alwaysMenuItems(l10n).map((item) => _buildItem(context, l10n, item)),
```

keeping the language-selector rows and everything else unchanged. This also wires the sidebar Settings item to `'/settings'`.

- [ ] **Step 3: Verify**

Run: `flutter analyze` → no new issues. Run: `flutter test` → all pass.
Manual sanity: sidebar renders identical items per section; Settings navigates; Messages/Notifications still show "coming soon".

---

### Task 8: contextMenuPosition-aware MainLayout + icons-only bar

**Files:**
- Create: `lib/widgets/context_icon_menu.dart`
- Modify: `lib/widgets/main_layout.dart`
- Modify: `lib/widgets/app_header.dart` (hide hamburger when no drawer)
- Test: `test/main_layout_position_test.dart`

**Interfaces:**
- Consumes: `SettingsProvider.contextMenuPosition` (Task 4), `ContextMenuItem`/`sectionMenuItems`/`alwaysMenuItems` (Task 7), `AuthProvider`, `PersonProvider`.
- Produces: `ContextIconMenu({required NavSection navSection, required String currentRoute})` widget; `MainLayout` renders per position: left = current behavior, right = mirrored (endDrawer / right panel), top/bottom = icons-only horizontal bar, no drawer, no user info.

- [ ] **Step 1: Write the failing widget test**

```dart
// test/main_layout_position_test.dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:lapidation_mobile/l10n/app_localizations.dart';
import 'package:lapidation_mobile/models/settings.dart';
import 'package:lapidation_mobile/providers/auth_provider.dart';
import 'package:lapidation_mobile/providers/locale_provider.dart';
import 'package:lapidation_mobile/providers/person_provider.dart';
import 'package:lapidation_mobile/providers/settings_provider.dart';
import 'package:lapidation_mobile/widgets/context_icon_menu.dart';
import 'package:lapidation_mobile/widgets/main_layout.dart';

Settings _settings(ContextMenuPosition position) => Settings(
      personId: 1,
      personUuid: 'p-uuid',
      language: 'en',
      theme: 'Light',
      notificationsEnabled: true,
      contextMenuPosition: position,
      homePage: HomePageOption.feed,
    );

Future<void> _pump(WidgetTester tester, ContextMenuPosition position) async {
  SharedPreferences.setMockInitialValues({});
  final settingsProvider = SettingsProvider();
  await settingsProvider.applyFromResources(_settings(position));

  await tester.pumpWidget(
    MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (_) => AuthProvider()),
        ChangeNotifierProvider(create: (_) => PersonProvider()),
        ChangeNotifierProvider(create: (_) => LocaleProvider()),
        ChangeNotifierProvider.value(value: settingsProvider),
      ],
      child: const MaterialApp(
        localizationsDelegates: AppLocalizations.localizationsDelegates,
        supportedLocales: AppLocalizations.supportedLocales,
        home: MainLayout(body: Text('page-body')),
      ),
    ),
  );
  await tester.pumpAndSettle();
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('left position uses a drawer on mobile, no icon bar',
      (tester) async {
    tester.view.physicalSize = const Size(400, 800);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.reset);

    await _pump(tester, ContextMenuPosition.left);

    final scaffold = tester.widget<Scaffold>(find.byType(Scaffold).first);
    expect(scaffold.drawer, isNotNull);
    expect(scaffold.endDrawer, isNull);
    expect(find.byType(ContextIconMenu), findsNothing);
  });

  testWidgets('right position uses an endDrawer on mobile', (tester) async {
    tester.view.physicalSize = const Size(400, 800);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.reset);

    await _pump(tester, ContextMenuPosition.right);

    final scaffold = tester.widget<Scaffold>(find.byType(Scaffold).first);
    expect(scaffold.drawer, isNull);
    expect(scaffold.endDrawer, isNotNull);
    expect(find.byType(ContextIconMenu), findsNothing);
  });

  testWidgets('top position shows the icon bar above the body, no drawers',
      (tester) async {
    tester.view.physicalSize = const Size(400, 800);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.reset);

    await _pump(tester, ContextMenuPosition.top);

    final scaffold = tester.widget<Scaffold>(find.byType(Scaffold).first);
    expect(scaffold.drawer, isNull);
    expect(scaffold.endDrawer, isNull);
    expect(find.byType(ContextIconMenu), findsOneWidget);

    final barY = tester.getTopLeft(find.byType(ContextIconMenu)).dy;
    final bodyY = tester.getTopLeft(find.text('page-body')).dy;
    expect(barY, lessThan(bodyY));
    // Icons only — no nav labels rendered as text.
    expect(find.text('Home'), findsNothing);
  });

  testWidgets('bottom position shows the icon bar below the body',
      (tester) async {
    tester.view.physicalSize = const Size(400, 800);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.reset);

    await _pump(tester, ContextMenuPosition.bottom);

    expect(find.byType(ContextIconMenu), findsOneWidget);
    final barY = tester.getTopLeft(find.byType(ContextIconMenu)).dy;
    final bodyY = tester.getTopLeft(find.text('page-body')).dy;
    expect(barY, greaterThan(bodyY));
  });
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `flutter test test/main_layout_position_test.dart`
Expected: FAIL — `context_icon_menu.dart` doesn't exist.

- [ ] **Step 3: Implement ContextIconMenu**

```dart
// lib/widgets/context_icon_menu.dart
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../config/app_colors.dart';
import '../config/context_menu_items.dart';
import '../config/nav_section.dart';
import '../l10n/app_localizations.dart';
import '../providers/auth_provider.dart';
import '../providers/person_provider.dart';
import '../providers/settings_provider.dart';

/// Icons-only context menu rendered as a horizontal bar when the user's
/// contextMenuPosition setting is Top or Bottom.
/// No labels and no user information — icons with tooltips only.
class ContextIconMenu extends StatelessWidget {
  final NavSection navSection;
  final String currentRoute;

  const ContextIconMenu({
    super.key,
    required this.navSection,
    required this.currentRoute,
  });

  void _showComingSoon(BuildContext context, AppLocalizations l10n) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(l10n.comingSoon),
        duration: const Duration(seconds: 2),
        behavior: SnackBarBehavior.floating,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final items = [
      ...sectionMenuItems(navSection, l10n),
      ...alwaysMenuItems(l10n),
    ];

    return Material(
      color: Colors.white,
      elevation: 1,
      child: SafeArea(
        top: false,
        child: SizedBox(
          height: 48,
          child: SingleChildScrollView(
            scrollDirection: Axis.horizontal,
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                for (final item in items)
                  IconButton(
                    tooltip: item.label,
                    icon: Icon(
                      currentRoute == item.route
                          ? item.activeIcon
                          : item.icon,
                      size: 24,
                      color: currentRoute == item.route
                          ? AppColors.primary
                          : const Color(0xFF555555),
                    ),
                    onPressed: item.route == null
                        ? () => _showComingSoon(context, l10n)
                        : () => Navigator.of(context)
                            .pushNamedAndRemoveUntil(
                                item.route!, (route) => false),
                  ),
                IconButton(
                  tooltip: l10n.logout,
                  icon: const Icon(Icons.logout,
                      size: 24, color: AppColors.danger),
                  onPressed: () {
                    context.read<AuthProvider>().signOut();
                    context.read<PersonProvider>().clear();
                    context.read<SettingsProvider>().clear();
                    Navigator.of(context).pushNamedAndRemoveUntil(
                        '/login', (route) => false);
                  },
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
```

- [ ] **Step 4: Make MainLayout position-aware**

Replace the `_MainLayoutState.build` method in `lib/widgets/main_layout.dart` (and add imports `package:provider/provider.dart`, `'../models/settings.dart'`, `'../providers/settings_provider.dart'`, `'context_icon_menu.dart'`):

```dart
  @override
  Widget build(BuildContext context) {
    final screenWidth = MediaQuery.of(context).size.width;
    final isDesktop = screenWidth >= 768;
    final position = context.watch<SettingsProvider>().contextMenuPosition;
    final isHorizontal = position == ContextMenuPosition.top ||
        position == ContextMenuPosition.bottom;

    final sidebarPanel = <Widget>[
      AnimatedContainer(
        duration: const Duration(milliseconds: 200),
        width: _isCollapsed ? 80 : 260,
        child: ClipRect(
          child: Container(
            color: Colors.white,
            child: SidebarContent(
              isCollapsed: _isCollapsed,
              onToggleCollapse: _toggleCollapsed,
              navSection: widget.navSection,
              currentRoute: widget.currentRoute,
            ),
          ),
        ),
      ),
      const VerticalDivider(width: 1, thickness: 1),
    ];

    Widget body;
    if (isHorizontal) {
      final bar = ContextIconMenu(
        navSection: widget.navSection,
        currentRoute: widget.currentRoute,
      );
      body = Column(
        children: [
          if (position == ContextMenuPosition.top) bar,
          Expanded(child: widget.body),
          if (position == ContextMenuPosition.bottom) bar,
        ],
      );
    } else if (isDesktop) {
      body = Row(
        children: position == ContextMenuPosition.left
            ? [...sidebarPanel, Expanded(child: widget.body)]
            : [Expanded(child: widget.body), ...sidebarPanel.reversed],
      );
    } else {
      body = widget.body;
    }

    final drawer = SidebarMenu(
      navSection: widget.navSection,
      currentRoute: widget.currentRoute,
    );
    final useDrawer = !isDesktop && !isHorizontal;

    return Scaffold(
      key: _scaffoldKey,
      appBar: AppHeader(
        notificationCount: widget.notificationCount,
        currentSection: widget.navSection,
        onMenuPressed: !useDrawer
            ? null
            : () {
                if (position == ContextMenuPosition.right) {
                  _scaffoldKey.currentState?.openEndDrawer();
                } else {
                  _scaffoldKey.currentState?.openDrawer();
                }
              },
        onSearchPressed: () {
          // TODO: Implement search
        },
        onNotificationsPressed: () {
          // TODO: Navigate to notifications
        },
        onProfilePressed: () {
          Navigator.of(context).pushNamedAndRemoveUntil(
            '/profile',
            (route) => false,
          );
        },
      ),
      drawer:
          useDrawer && position == ContextMenuPosition.left ? drawer : null,
      endDrawer:
          useDrawer && position == ContextMenuPosition.right ? drawer : null,
      body: body,
      floatingActionButton: widget.floatingActionButton,
    );
  }
```

- [ ] **Step 5: Hide the hamburger when there is no drawer**

In `lib/widgets/app_header.dart` `build`, change the hamburger condition from:

```dart
                if (!isDesktop)
```

to:

```dart
                if (!isDesktop && onMenuPressed != null)
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `flutter test test/main_layout_position_test.dart` → PASS.
Run: `flutter analyze` → no new issues. Run: `flutter test` → all pass.

---

### Task 9: AppNotification model + NotificationService

**Files:**
- Create: `lib/models/app_notification.dart`
- Create: `lib/services/notification_service.dart`
- Modify: `lib/config/api_config.dart`
- Test: `test/app_notification_model_test.dart`

**Interfaces:**
- Produces: `AppNotification` (`uuid`, `notificationType`, `recipientPersonUuid`, `actorPersonUuid`, `actorName`, `postUuid`, `commentUuid`, `entityType`, `entityUuid`, `snippet`, `read`, `createdAt`, `updatedAt`, `fromJson`, `copyWith({bool? read})`, `String? get targetPostUuid`); `NotificationService.fetchNotifications(String token, {bool unreadOnly = false, int limit = 50}) → Future<List<AppNotification>>`; `NotificationService.markAsRead(String uuid, String token) → Future<void>`; `ApiConfig.notificationsEndpoint`.

- [ ] **Step 1: Write the failing test**

```dart
// test/app_notification_model_test.dart
import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/models/app_notification.dart';

void main() {
  final json = {
    'uuid': 'post-1:person-9',
    'notificationType': 'Mention',
    'recipientPersonUuid': 'person-9',
    'actorPersonUuid': 'person-2',
    'actorName': 'Jane Doe',
    'postUuid': 'post-1',
    'entityType': 'post',
    'entityUuid': 'post-1',
    'snippet': 'hey @you check this',
    'read': false,
    'createdAt': '2026-07-07T10:00:00',
    'updatedAt': '2026-07-07T10:00:00',
  };

  test('parses json (commentUuid absent)', () {
    final n = AppNotification.fromJson(json);
    expect(n.uuid, 'post-1:person-9');
    expect(n.actorName, 'Jane Doe');
    expect(n.postUuid, 'post-1');
    expect(n.commentUuid, isNull);
    expect(n.read, false);
    expect(n.createdAt, DateTime.parse('2026-07-07T10:00:00'));
  });

  test('copyWith read flips only the read flag', () {
    final n = AppNotification.fromJson(json).copyWith(read: true);
    expect(n.read, true);
    expect(n.uuid, 'post-1:person-9');
  });

  test('targetPostUuid prefers postUuid then post-typed entityUuid', () {
    expect(AppNotification.fromJson(json).targetPostUuid, 'post-1');

    final noPostUuid = Map<String, dynamic>.from(json)..remove('postUuid');
    expect(AppNotification.fromJson(noPostUuid).targetPostUuid, 'post-1');

    final commentOnly = {
      ...noPostUuid,
      'entityType': 'comment',
      'entityUuid': 'comment-5',
    };
    expect(AppNotification.fromJson(commentOnly).targetPostUuid, isNull);
  });
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `flutter test test/app_notification_model_test.dart`
Expected: FAIL — package URI can't be resolved.

- [ ] **Step 3: Implement the model**

```dart
// lib/models/app_notification.dart

/// In-app notification from the timeline service
/// (GET /timeline/api/notifications).
class AppNotification {
  final String uuid;
  final String notificationType;
  final String recipientPersonUuid;
  final String actorPersonUuid;
  final String actorName;
  final String? postUuid;
  final String? commentUuid;
  final String entityType;
  final String entityUuid;
  final String snippet;
  final bool read;
  final DateTime createdAt;
  final DateTime updatedAt;

  AppNotification({
    required this.uuid,
    required this.notificationType,
    required this.recipientPersonUuid,
    required this.actorPersonUuid,
    required this.actorName,
    this.postUuid,
    this.commentUuid,
    required this.entityType,
    required this.entityUuid,
    required this.snippet,
    required this.read,
    required this.createdAt,
    required this.updatedAt,
  });

  factory AppNotification.fromJson(Map<String, dynamic> json) {
    return AppNotification(
      uuid: json['uuid'] as String? ?? '',
      notificationType: json['notificationType'] as String? ?? '',
      recipientPersonUuid: json['recipientPersonUuid'] as String? ?? '',
      actorPersonUuid: json['actorPersonUuid'] as String? ?? '',
      actorName: json['actorName'] as String? ?? '',
      postUuid: json['postUuid'] as String?,
      commentUuid: json['commentUuid'] as String?,
      entityType: json['entityType'] as String? ?? '',
      entityUuid: json['entityUuid'] as String? ?? '',
      snippet: json['snippet'] as String? ?? '',
      read: json['read'] as bool? ?? false,
      createdAt: DateTime.parse(json['createdAt'] as String),
      updatedAt: DateTime.parse(json['updatedAt'] as String),
    );
  }

  AppNotification copyWith({bool? read}) {
    return AppNotification(
      uuid: uuid,
      notificationType: notificationType,
      recipientPersonUuid: recipientPersonUuid,
      actorPersonUuid: actorPersonUuid,
      actorName: actorName,
      postUuid: postUuid,
      commentUuid: commentUuid,
      entityType: entityType,
      entityUuid: entityUuid,
      snippet: snippet,
      read: read ?? this.read,
      createdAt: createdAt,
      updatedAt: updatedAt,
    );
  }

  /// The post to open when the notification is tapped.
  String? get targetPostUuid {
    if (postUuid != null && postUuid!.isNotEmpty) return postUuid;
    if (entityType.toLowerCase() == 'post') return entityUuid;
    return null;
  }
}
```

- [ ] **Step 4: Add endpoint + service**

In `lib/config/api_config.dart`, below `settingsMeEndpoint`:

```dart
   static const String notificationsEndpoint = '/timeline/api/notifications';
```

```dart
// lib/services/notification_service.dart
import 'package:dio/dio.dart';
import '../config/api_config.dart';
import '../models/app_notification.dart';
import '../utils/dio_client.dart';
import 'base_service.dart';

class NotificationService {
  static final _dio = DioClient().dio;

  static Future<List<AppNotification>> fetchNotifications(
    String token, {
    bool unreadOnly = false,
    int limit = 50,
  }) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        ApiConfig.notificationsEndpoint,
        queryParameters: {'unread_only': unreadOnly, 'limit': limit},
      );
      if (response.statusCode == 200) {
        final List<dynamic> data = response.data;
        return data
            .map((e) => AppNotification.fromJson(e as Map<String, dynamic>))
            .toList();
      }
      throw AppException(
        statusCode: response.statusCode ?? 500,
        message: response.data?['message'] ?? 'Failed to load notifications',
      );
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to load notifications');
    }
  }

  static Future<void> markAsRead(String uuid, String token) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.put(
        '${ApiConfig.notificationsEndpoint}/$uuid/read',
        data: {'read': true},
      );
      if (response.statusCode != 200) {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message:
              response.data?['message'] ?? 'Failed to mark notification read',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to mark notification read');
    }
  }
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `flutter test test/app_notification_model_test.dart` → PASS.
Run: `flutter analyze` → no new issues.

---

### Task 10: NotificationProvider + registration

**Files:**
- Create: `lib/providers/notification_provider.dart`
- Modify: `lib/main.dart` (register)
- Modify: `lib/widgets/sidebar_menu.dart` + `lib/widgets/app_header.dart` (clear on logout, same spots as Task 4)
- Modify: `lib/widgets/context_icon_menu.dart` (clear on logout)

**Interfaces:**
- Consumes: `AppNotification`, `NotificationService` (Task 9).
- Produces: `NotificationProvider` with `notifications`, `loading`, `error`, `unreadCount`, `fetchNotifications(String token)`, `fetchIfStale(String token)`, `markAsRead(String uuid, String token) → Future<bool>`, `clear()`.

- [ ] **Step 1: Implement the provider**

```dart
// lib/providers/notification_provider.dart
import 'package:flutter/foundation.dart';

import '../models/app_notification.dart';
import '../services/base_service.dart';
import '../services/notification_service.dart';

class NotificationProvider extends ChangeNotifier {
  static const Duration _staleAfter = Duration(seconds: 60);

  List<AppNotification> _notifications = [];
  bool _loading = false;
  String? _error;
  DateTime? _lastFetched;

  List<AppNotification> get notifications =>
      List.unmodifiable(_notifications);
  bool get loading => _loading;
  String? get error => _error;
  int get unreadCount => _notifications.where((n) => !n.read).length;

  Future<void> fetchNotifications(String token) async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      _notifications = await NotificationService.fetchNotifications(token);
      _lastFetched = DateTime.now();
      _loading = false;
      notifyListeners();
    } on AppException catch (e) {
      _error = e.message;
      _loading = false;
      notifyListeners();
    } catch (_) {
      _error = 'Failed to load notifications. Please try again.';
      _loading = false;
      notifyListeners();
    }
  }

  /// Fetches only when never loaded or older than [_staleAfter].
  /// Used by the layout to keep the header badge fresh without
  /// refetching on every navigation.
  Future<void> fetchIfStale(String token) async {
    if (_loading) return;
    if (_lastFetched != null &&
        DateTime.now().difference(_lastFetched!) < _staleAfter) {
      return;
    }
    await fetchNotifications(token);
  }

  /// Optimistically marks as read; reverts on failure.
  Future<bool> markAsRead(String uuid, String token) async {
    final index = _notifications.indexWhere((n) => n.uuid == uuid);
    if (index < 0 || _notifications[index].read) return true;

    _notifications = [..._notifications];
    _notifications[index] = _notifications[index].copyWith(read: true);
    notifyListeners();

    try {
      await NotificationService.markAsRead(uuid, token);
      return true;
    } catch (_) {
      _notifications = [..._notifications];
      _notifications[index] = _notifications[index].copyWith(read: false);
      notifyListeners();
      return false;
    }
  }

  /// Clear notifications (e.g., on logout).
  void clear() {
    _notifications = [];
    _error = null;
    _lastFetched = null;
    notifyListeners();
  }
}
```

- [ ] **Step 2: Register and clear on logout**

`lib/main.dart` — import `'providers/notification_provider.dart'` and add after `SettingsProvider`:

```dart
        ChangeNotifierProvider(create: (_) => NotificationProvider()),
```

In the three logout handlers (sidebar item, header profile menu, `ContextIconMenu`), next to `context.read<SettingsProvider>().clear();` add:

```dart
            context.read<NotificationProvider>().clear();
```

with the matching import in each file.

- [ ] **Step 3: Update the MainLayout widget test harness**

`test/main_layout_position_test.dart` will need the new provider once Task 12 wires it into `MainLayout`; add it now so the harness stays valid:

```dart
        ChangeNotifierProvider(create: (_) => NotificationProvider()),
```

(with import `package:lapidation_mobile/providers/notification_provider.dart`).

- [ ] **Step 4: Verify**

Run: `flutter analyze` → no new issues. Run: `flutter test` → all pass.

---

### Task 11: PostDetailPage + FeedProvider.findPostByUuid + l10n

**Files:**
- Modify: `lib/providers/feed_provider.dart`
- Create: `lib/pages/home/post_detail_page.dart`
- Modify: all six `lib/l10n/app_*.arb` files

**Interfaces:**
- Consumes: `FeedService.fetchPosts(token, page:)`, `FeedPost.uuid`, `FeedPostWidget(post:)`, `MainLayout`.
- Produces: `FeedProvider.findPostByUuid(String token, String uuid, {int maxPages = 10}) → Future<FeedPost?>`; `PostDetailPage({required String postUuid})` (pushed via `MaterialPageRoute`, not a named route). Task 12 consumes `PostDetailPage`.

- [ ] **Step 1: Add l10n keys**

`app_en.arb`: `"postNotFound": "Post not found"`
`app_pt.arb` + `app_pt_BR.arb`: `"postNotFound": "Publicação não encontrada"`
`app_es.arb`: `"postNotFound": "Publicación no encontrada"`
`app_fr.arb`: `"postNotFound": "Publication introuvable"`
`app_nl.arb`: `"postNotFound": "Bericht niet gevonden"`

Run: `flutter gen-l10n`.

- [ ] **Step 2: Add finder to FeedProvider**

In `lib/providers/feed_provider.dart`, under the `// ── Helpers ──` section:

```dart
  /// Finds a post by uuid — first in the already-loaded feed, then by
  /// walking the paginated feed (bounded). Used when arriving from a
  /// notification; the timeline service has no single-post endpoint.
  /// Returns null when the post cannot be found.
  Future<FeedPost?> findPostByUuid(
    String token,
    String uuid, {
    int maxPages = 10,
  }) async {
    for (final post in _posts) {
      if (post.uuid == uuid) return post;
    }
    for (var page = 0; page < maxPages; page++) {
      final fetched = await FeedService.fetchPosts(token, page: page);
      for (final post in fetched) {
        if (post.uuid == uuid) return post;
      }
      if (fetched.length < _pageSize) break;
    }
    return null;
  }
```

- [ ] **Step 3: Implement PostDetailPage**

```dart
// lib/pages/home/post_detail_page.dart
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../models/feed_post.dart';
import '../../providers/auth_provider.dart';
import '../../providers/feed_provider.dart';
import '../../widgets/feed_post_widget.dart';
import '../../widgets/main_layout.dart';

/// Shows a single feed post — the target of a notification tap.
class PostDetailPage extends StatefulWidget {
  final String postUuid;

  const PostDetailPage({super.key, required this.postUuid});

  @override
  State<PostDetailPage> createState() => _PostDetailPageState();
}

class _PostDetailPageState extends State<PostDetailPage> {
  FeedPost? _post;
  bool _loading = true;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _load());
  }

  Future<void> _load() async {
    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
    FeedPost? post;
    if (token.isNotEmpty) {
      try {
        post = await context
            .read<FeedProvider>()
            .findPostByUuid(token, widget.postUuid);
      } catch (_) {
        post = null;
      }
    }
    if (mounted) {
      setState(() {
        _post = post;
        _loading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/home',
      body: _loading
          ? const Center(
              child: CircularProgressIndicator(color: AppColors.primary))
          : _post == null
              ? Center(
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(Icons.search_off,
                          size: 64, color: Colors.grey[300]),
                      const SizedBox(height: 12),
                      Text(
                        l10n.postNotFound,
                        style: TextStyle(
                            fontSize: 16, color: Colors.grey[600]),
                      ),
                    ],
                  ),
                )
              : SingleChildScrollView(
                  child: FeedPostWidget(post: _post!),
                ),
    );
  }
}
```

- [ ] **Step 4: Verify**

Run: `flutter analyze` → no new issues. Run: `flutter test` → all pass.

---

### Task 12: Notifications page + badge/menu wiring + l10n

**Files:**
- Create: `lib/pages/notifications/notifications_page.dart`
- Modify: `lib/main.dart` (route `/notifications`)
- Modify: `lib/widgets/main_layout.dart` (badge count from provider, bell navigation, fetch-if-stale)
- Modify: `lib/config/context_menu_items.dart` (notifications item gets its route)
- Modify: all six `lib/l10n/app_*.arb` files

**Interfaces:**
- Consumes: `NotificationProvider` (Task 10), `AppNotification.targetPostUuid` (Task 9), `SettingsProvider.notificationsEnabled` (Task 4), `PostDetailPage` (Task 11).
- Produces: route `'/notifications'` → `NotificationsPage`; live unread badge in the header.

- [ ] **Step 1: Add l10n keys**

`app_en.arb`:

```json
  "notificationsEmpty": "No notifications yet",
  "notificationsDisabled": "Notifications are disabled",
  "notificationsEnableInSettings": "Enable them in Settings",
  "notificationMentionedYou": "mentioned you"
```

`app_pt.arb` + `app_pt_BR.arb`:

```json
  "notificationsEmpty": "Nenhuma notificação",
  "notificationsDisabled": "As notificações estão desativadas",
  "notificationsEnableInSettings": "Ative-as nas Configurações",
  "notificationMentionedYou": "mencionou você"
```

`app_es.arb`:

```json
  "notificationsEmpty": "Sin notificaciones",
  "notificationsDisabled": "Las notificaciones están desactivadas",
  "notificationsEnableInSettings": "Actívalas en Configuración",
  "notificationMentionedYou": "te mencionó"
```

`app_fr.arb`:

```json
  "notificationsEmpty": "Aucune notification",
  "notificationsDisabled": "Les notifications sont désactivées",
  "notificationsEnableInSettings": "Activez-les dans les Paramètres",
  "notificationMentionedYou": "vous a mentionné"
```

`app_nl.arb`:

```json
  "notificationsEmpty": "Geen meldingen",
  "notificationsDisabled": "Meldingen zijn uitgeschakeld",
  "notificationsEnableInSettings": "Schakel ze in bij Instellingen",
  "notificationMentionedYou": "heeft je vermeld"
```

Run: `flutter gen-l10n`.

- [ ] **Step 2: Implement the page**

```dart
// lib/pages/notifications/notifications_page.dart
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../models/app_notification.dart';
import '../../providers/auth_provider.dart';
import '../../providers/notification_provider.dart';
import '../../providers/settings_provider.dart';
import '../../widgets/main_layout.dart';
import '../home/post_detail_page.dart';

class NotificationsPage extends StatefulWidget {
  const NotificationsPage({super.key});

  @override
  State<NotificationsPage> createState() => _NotificationsPageState();
}

class _NotificationsPageState extends State<NotificationsPage> {
  String get _token => context.read<AuthProvider>().auth?.accessToken ?? '';

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      final enabled = context.read<SettingsProvider>().notificationsEnabled;
      final token = _token;
      if (enabled && token.isNotEmpty) {
        context.read<NotificationProvider>().fetchNotifications(token);
      }
    });
  }

  Future<void> _onTap(AppNotification notification) async {
    final token = _token;
    if (token.isNotEmpty && notification.uuid.isNotEmpty) {
      // Fire and forget — the provider reverts on failure.
      context.read<NotificationProvider>().markAsRead(notification.uuid, token);
    }
    final target = notification.targetPostUuid;
    if (target != null && mounted) {
      Navigator.of(context).push(
        MaterialPageRoute(builder: (_) => PostDetailPage(postUuid: target)),
      );
    }
  }

  String _relativeTime(DateTime dateTime) {
    final diff = DateTime.now().difference(dateTime);
    if (diff.inDays > 0) return '${diff.inDays}d';
    if (diff.inHours > 0) return '${diff.inHours}h';
    if (diff.inMinutes > 0) return '${diff.inMinutes}m';
    return 'now';
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final enabled = context.watch<SettingsProvider>().notificationsEnabled;

    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/notifications',
      body: !enabled
          ? _DisabledState(l10n: l10n)
          : Consumer<NotificationProvider>(
              builder: (context, provider, _) {
                if (provider.loading && provider.notifications.isEmpty) {
                  return const Center(
                    child:
                        CircularProgressIndicator(color: AppColors.primary),
                  );
                }
                if (provider.error != null &&
                    provider.notifications.isEmpty) {
                  return Center(
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Text(provider.error!,
                            style:
                                const TextStyle(color: AppColors.danger)),
                        TextButton(
                          onPressed: () =>
                              provider.fetchNotifications(_token),
                          child: const Text('Retry'),
                        ),
                      ],
                    ),
                  );
                }
                if (provider.notifications.isEmpty) {
                  return Center(
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Icon(Icons.notifications_none,
                            size: 80, color: Colors.grey[300]),
                        const SizedBox(height: 16),
                        Text(
                          l10n.notificationsEmpty,
                          style: TextStyle(
                              fontSize: 16, color: Colors.grey[600]),
                        ),
                      ],
                    ),
                  );
                }
                return RefreshIndicator(
                  color: AppColors.primary,
                  onRefresh: () => provider.fetchNotifications(_token),
                  child: ListView.separated(
                    physics: const AlwaysScrollableScrollPhysics(),
                    itemCount: provider.notifications.length,
                    separatorBuilder: (_, _) =>
                        const Divider(height: 1, indent: 72),
                    itemBuilder: (context, index) {
                      final notification = provider.notifications[index];
                      return _NotificationTile(
                        notification: notification,
                        l10n: l10n,
                        relativeTime:
                            _relativeTime(notification.createdAt),
                        onTap: () => _onTap(notification),
                      );
                    },
                  ),
                );
              },
            ),
    );
  }
}

class _DisabledState extends StatelessWidget {
  final AppLocalizations l10n;

  const _DisabledState({required this.l10n});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(Icons.notifications_off_outlined,
              size: 80, color: Colors.grey[300]),
          const SizedBox(height: 16),
          Text(
            l10n.notificationsDisabled,
            style: TextStyle(fontSize: 16, color: Colors.grey[600]),
          ),
          const SizedBox(height: 8),
          TextButton(
            onPressed: () => Navigator.of(context).pushNamed('/settings'),
            child: Text(l10n.notificationsEnableInSettings),
          ),
        ],
      ),
    );
  }
}

class _NotificationTile extends StatelessWidget {
  final AppNotification notification;
  final AppLocalizations l10n;
  final String relativeTime;
  final VoidCallback onTap;

  const _NotificationTile({
    required this.notification,
    required this.l10n,
    required this.relativeTime,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return ListTile(
      onTap: onTap,
      tileColor:
          notification.read ? null : AppColors.primary.withAlpha(10),
      leading: CircleAvatar(
        backgroundColor: AppColors.primary.withAlpha(30),
        child: const Icon(Icons.alternate_email,
            color: AppColors.primary, size: 20),
      ),
      title: RichText(
        maxLines: 1,
        overflow: TextOverflow.ellipsis,
        text: TextSpan(
          style: const TextStyle(color: Color(0xFF333333), fontSize: 14),
          children: [
            TextSpan(
              text: notification.actorName,
              style: const TextStyle(fontWeight: FontWeight.w600),
            ),
            TextSpan(text: ' ${l10n.notificationMentionedYou}'),
          ],
        ),
      ),
      subtitle: Text(
        notification.snippet,
        maxLines: 2,
        overflow: TextOverflow.ellipsis,
        style: TextStyle(color: Colors.grey[600], fontSize: 13),
      ),
      trailing: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          Text(
            relativeTime,
            style: TextStyle(color: Colors.grey[500], fontSize: 12),
          ),
          if (!notification.read) ...[
            const SizedBox(height: 6),
            Container(
              width: 10,
              height: 10,
              decoration: const BoxDecoration(
                color: AppColors.primary,
                shape: BoxShape.circle,
              ),
            ),
          ],
        ],
      ),
    );
  }
}
```

- [ ] **Step 3: Register route and wire menus**

`lib/main.dart`: import `'pages/notifications/notifications_page.dart'` and add:

```dart
              '/notifications': (context) => const NotificationsPage(),
```

`lib/config/context_menu_items.dart`: give the notifications item its route:

```dart
        ContextMenuItem(
            icon: Icons.notifications_outlined,
            activeIcon: Icons.notifications,
            label: l10n.menuNotifications,
            route: '/notifications'),
```

- [ ] **Step 4: Live badge + bell navigation in MainLayout**

In `lib/widgets/main_layout.dart`:

1. Remove the `notificationCount` field and constructor parameter from `MainLayout` (verify no caller passes it: `rg "notificationCount" lib/` must only show `app_header.dart` after this change).
2. Add imports `'../providers/auth_provider.dart'` and `'../providers/notification_provider.dart'`.
3. In `_MainLayoutState`, add:

```dart
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      final token =
          context.read<AuthProvider>().auth?.accessToken ?? '';
      final enabled =
          context.read<SettingsProvider>().notificationsEnabled;
      if (token.isNotEmpty && enabled) {
        context.read<NotificationProvider>().fetchIfStale(token);
      }
    });
  }
```

4. In `build`, read the badge count and wire the bell:

```dart
    final notificationsEnabled =
        context.watch<SettingsProvider>().notificationsEnabled;
    final unreadCount = notificationsEnabled
        ? context.watch<NotificationProvider>().unreadCount
        : 0;
```

and in the `AppHeader(...)` call replace `notificationCount: widget.notificationCount,` with `notificationCount: unreadCount,` and the `onNotificationsPressed` TODO with:

```dart
        onNotificationsPressed: () {
          Navigator.of(context).pushNamed('/notifications');
        },
```

- [ ] **Step 5: Verify**

Run: `flutter analyze` → no new issues. Run: `flutter test` → all pass (including `test/main_layout_position_test.dart`, whose harness already provides `NotificationProvider` from Task 10).

---

### Task 13: Final verification

**Files:** none (verification only).

- [ ] **Step 1: Full static + test pass**

Run: `flutter gen-l10n` → no errors.
Run: `flutter analyze` → no issues introduced by this work.
Run: `flutter test` → all tests pass.

- [ ] **Step 2: Manual QA checklist (against the dev stack in `development/`)**

1. Login with a user whose settings has `language: "pt_BR"` while the device is in English → UI switches to Portuguese right after login.
2. Settings `homePage: "Gallery"` → Gallery page appears after login; `"Feed"` → feed page.
3. Settings page: open from header profile menu and from sidebar; change each field, Save → success snackbar; re-login → values persist and are applied.
4. `contextMenuPosition`: Left → drawer/left panel (unchanged); Right → drawer opens from the right / panel on the right; Top → icons-only bar fixed below the header, no user info; Bottom → same bar at the bottom.
5. Bell icon shows unread count; tapping it (or the sidebar item) opens Notifications; tapping a notification marks it read and opens the post it came from; already-read items render without the dot.
6. Toggle `notificationsEnabled` off → badge disappears, notifications page shows the disabled state with a link to Settings.
7. User with **no** settings row (`settings: null` in `/resource`) → app behaves as today (English/device language, feed home, left menu); opening Settings and saving creates the row.

- [ ] **Step 3: Leave the working tree uncommitted**

Per the global constraint, do **not** commit — summarize the changed files for the user's review.

---

## Self-Review Notes

- **Spec coverage:** language override → Tasks 2/5; homePage → Tasks 1/5/6; contextMenuPosition (Left/Top/Right/Bottom, icons-only top/bottom, no user info) → Tasks 7/8; settings page → Task 6; notificationsEnabled + notifications page + navigate-to-origin → Tasks 9–12; theme deliberately ignored (round-tripped only) per user instruction.
- **Ordering:** `/settings` route (Task 6) exists before `context_menu_items.dart` references it (Task 7); `/notifications` route referenced in menus only from Task 12, after the page exists; `NotificationProvider` is registered (Task 10) before `MainLayout` watches it (Task 12).
- **Type consistency:** `SettingsProvider.applyFromResources(Settings?)`, `HomePageOption.route`, `AppNotification.targetPostUuid`, `FeedProvider.findPostByUuid(token, uuid)` are used with these exact names across tasks.