# Mobile User Settings Edit Page Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a `/settings` page in `socialgym_mobile` where the user edits language, notification preference, context-menu position, and home page, persisted via the `workout` service's gRPC `SettingsService`.

**Architecture:** `SettingsPage` (form UI) → `SettingsProvider` (state, already app-wide via `MultiProvider`) → `GrpcSettingsService` (new static client wrapper, mirrors the existing `GrpcPersonService` pattern) → generated `SettingsServiceClient` stub → workout's already-implemented `GrpcSettingService`.

**Tech Stack:** Flutter 3 / Dart 3.12, `provider` (ChangeNotifier), `grpc` ^5.1.0 package, generated protobuf stubs via `protoc` + `protoc-gen-dart`, `flutter gen-l10n` for ARB-based i18n.

## Global Constraints

- Work happens only in `socialgym_mobile/` (its own git repo). The `workout` backend is complete and out of scope — do not edit anything under `workout/`.
- `proto/settings.proto` is already staged/committed on this branch (`feature/notification_and_settings`) and must not be changed — it's byte-identical to `workout/integration/proto/settings.proto`.
- Follow the existing static-service-class pattern (`GrpcPersonService`) — no new repository/interface abstractions.
- The backend's `Position::from_string` (Rust) only recognizes capitalized position strings (`"Left"/"Top"/"Right"/"Bottom"`) and silently defaults to `Left` otherwise. `ContextMenuPosition`'s Dart-side canonical value stays lowercase; capitalize only at the `GrpcSettingsService` wire boundary.
- `SettingsGateway::persist` (Rust) does a SeaORM `save()` that **inserts** a new row whenever `id` is unset — every `persistSettings` call from the app must carry the previously-loaded `id`/`uuid` forward, or it will create duplicate settings rows.
- No widget tests or gRPC-client-mocking tests exist anywhere in this repo today (confirmed: `NotificationsPage` has none; the one gRPC test, `test/grpc_person_service_test.dart`, only round-trips proto message encode/decode). Don't introduce a new mocking framework — follow that precedent.
- Six ARB locale files exist: `app_en.arb`, `app_es.arb`, `app_fr.arb`, `app_nl.arb`, `app_pt.arb`, `app_pt_BR.arb`. Add new keys to all six.
- `flutter` and `protoc`/`protoc-gen-dart` are installed and on `PATH` in this environment (verified: `protoc` 3.21.12, `protoc_plugin` 25.0.0, Dart SDK 3.12.2).

---

### Task 1: Generate gRPC stubs for `settings.proto`

**Files:**
- Run: `socialgym_mobile/tool/generate_proto.sh` (no edits — script already handles all `proto/*.proto` files)
- Produces: `lib/src/generated/grpc/settings.pb.dart`, `settings.pbenum.dart`, `settings.pbgrpc.dart`, `settings.pbjson.dart`

**Interfaces:**
- Produces (for later tasks): `$settings.Setting` (fields: `id` int, `uuid` String, `ownerId` int, `ownerUuid` String, `language` String, `theme` String, `notificationsEnabled` bool, `contextMenuPosition` String, `homePage` String, `createdAt` String, `updatedAt` String), `$settings.SettingIdRequest`, `$settings.SettingOwnerIdRequest` (fields `ownerId` int, `ownerUuid` String), `$settings.SettingsServiceClient` with methods `getById`, `persistSettings`, `getByUuid`, `getByOwnerIds` (each `Future<Setting> Function(Request, {CallOptions? options})`).

- [ ] **Step 1: Run the generator**

```bash
cd /home/erocha/workspace/socialgym_project/socialgym_mobile
./tool/generate_proto.sh
```

Expected output: `Generated Dart gRPC files in .../lib/src/generated/grpc` with no errors.

- [ ] **Step 2: Verify the settings stubs were created**

```bash
ls lib/src/generated/grpc/settings.*
```

Expected: `settings.pb.dart  settings.pbenum.dart  settings.pbgrpc.dart  settings.pbjson.dart`

- [ ] **Step 3: Verify the whole project still analyzes cleanly**

```bash
flutter pub get && dart analyze lib/src/generated/grpc/settings.pb.dart lib/src/generated/grpc/settings.pbgrpc.dart
```

Expected: `No issues found!`

- [ ] **Step 4: Commit**

```bash
git add lib/src/generated/grpc/settings.pb.dart lib/src/generated/grpc/settings.pbenum.dart lib/src/generated/grpc/settings.pbgrpc.dart lib/src/generated/grpc/settings.pbjson.dart
git commit -m "Generate gRPC Dart stubs for SettingsService"
```

---

### Task 2: Fix `Settings.copyWith` and thread `notificationsEnabled` through

**Files:**
- Modify: `lib/models/resource.dart:124-130`
- Test: `test/settings_copy_with_test.dart` (create)

**Interfaces:**
- Produces: `Settings.copyWith({int? id, String? uuid, int? personId, String? personUuid, String? language, String? theme, bool? notificationsEnabled, Pages? homePage, ContextMenuPosition? contextMenuPosition})` — every field now threads through by default instead of resetting to constructor defaults.

- [ ] **Step 1: Write the failing test**

Create `test/settings_copy_with_test.dart`:

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/models/resource.dart';

void main() {
  group('Settings.copyWith', () {
    test('preserves id, personId, personUuid, theme and notificationsEnabled when only language changes', () {
      final original = Settings(
        id: 42,
        uuid: 'setting-uuid',
        personId: 7,
        personUuid: 'person-uuid',
        language: 'en',
        theme: 'dark',
        notificationsEnabled: false,
        contextMenuPosition: ContextMenuPosition.right,
        homePage: Pages.gallery,
      );

      final updated = original.copyWith(language: 'es');

      expect(updated.id, 42);
      expect(updated.uuid, 'setting-uuid');
      expect(updated.personId, 7);
      expect(updated.personUuid, 'person-uuid');
      expect(updated.theme, 'dark');
      expect(updated.notificationsEnabled, false);
      expect(updated.contextMenuPosition, ContextMenuPosition.right);
      expect(updated.homePage, Pages.gallery);
      expect(updated.language, 'es');
    });

    test('updates notificationsEnabled when passed explicitly', () {
      final original = Settings(notificationsEnabled: true);
      final updated = original.copyWith(notificationsEnabled: false);
      expect(updated.notificationsEnabled, false);
    });
  });
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `flutter test test/settings_copy_with_test.dart`
Expected: FAIL — `updated.id` is `0`, not `42` (and similar for the other preserved fields), because the current `copyWith` drops them.

- [ ] **Step 3: Fix `copyWith`**

In `lib/models/resource.dart`, replace lines 124-130:

```dart
  Settings copyWith({String? language, Pages? homePage, ContextMenuPosition? contextMenuPosition}) {
    return Settings(
      language: language ?? this.language,
      homePage: homePage ?? this.homePage,
      contextMenuPosition: contextMenuPosition ?? this.contextMenuPosition,
    );
  }
```

with:

```dart
  Settings copyWith({
    int? id,
    String? uuid,
    int? personId,
    String? personUuid,
    String? language,
    String? theme,
    bool? notificationsEnabled,
    Pages? homePage,
    ContextMenuPosition? contextMenuPosition,
  }) {
    return Settings(
      id: id ?? this.id,
      uuid: uuid ?? this.uuid,
      personId: personId ?? this.personId,
      personUuid: personUuid ?? this.personUuid,
      language: language ?? this.language,
      theme: theme ?? this.theme,
      notificationsEnabled: notificationsEnabled ?? this.notificationsEnabled,
      homePage: homePage ?? this.homePage,
      contextMenuPosition: contextMenuPosition ?? this.contextMenuPosition,
      createdAt: createdAt,
      updatedAt: updatedAt,
    );
  }
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `flutter test test/settings_copy_with_test.dart`
Expected: PASS (both tests).

- [ ] **Step 5: Run the full test suite to check for regressions**

Run: `flutter test`
Expected: All existing tests still PASS (this change only fixes a bug in unused parameters — no existing caller passes `id`/`uuid`/`personId`/`personUuid`/`theme`/`notificationsEnabled` to `copyWith` today, so behavior for existing callers is unchanged except those fields now survive instead of resetting).

- [ ] **Step 6: Commit**

```bash
git add lib/models/resource.dart test/settings_copy_with_test.dart
git commit -m "Fix Settings.copyWith dropping id/personId/personUuid/theme/notificationsEnabled"
```

---

### Task 3: `GrpcSettingsService` client wrapper

**Files:**
- Create: `lib/services/grpc/grpc_settings_service.dart`
- Test: `test/grpc_settings_service_test.dart` (create)

**Interfaces:**
- Consumes: `Settings`, `ContextMenuPosition`, `Pages` from `lib/models/resource.dart` (Task 2); `$settings.Setting`, `$settings.SettingOwnerIdRequest`, `$settings.SettingsServiceClient` from Task 1; `GrpcChannelFactory.channelFor(...)` / `GrpcChannelFactory.interceptors` from `lib/services/grpc/grpc_channel_factory.dart` (unchanged); `ApiConfig.grpcHost` / `grpcPort` / `grpcAuthority` from `lib/config/api_config.dart` (unchanged).
- Produces (for Task 4): `GrpcSettingsService.getByOwnerId({required int ownerId, String ownerUuid})` → `Future<Settings?>` (`null` = no settings row yet); `GrpcSettingsService.persistSettings(Settings settings)` → `Future<Settings>`; `GrpcSettingsService.toProto(Settings)` → `$settings.Setting`; `GrpcSettingsService.toDomain($settings.Setting)` → `Settings`; `GrpcSettingsService.positionToWire(ContextMenuPosition)` → `String`.

- [ ] **Step 1: Write the failing test**

Create `test/grpc_settings_service_test.dart`:

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/models/resource.dart';
import 'package:socialgym_mobile/services/grpc/grpc_settings_service.dart';
import 'package:socialgym_mobile/src/generated/grpc/settings.pb.dart' as $settings;

void main() {
  group('GrpcSettingsService.positionToWire', () {
    test('capitalizes the position name so the backend Position::from_string matches it', () {
      expect(GrpcSettingsService.positionToWire(ContextMenuPosition.left), 'Left');
      expect(GrpcSettingsService.positionToWire(ContextMenuPosition.top), 'Top');
      expect(GrpcSettingsService.positionToWire(ContextMenuPosition.right), 'Right');
      expect(GrpcSettingsService.positionToWire(ContextMenuPosition.bottom), 'Bottom');
    });
  });

  group('GrpcSettingsService.toProto / toDomain', () {
    test('round-trips a fully-populated Settings through the proto message', () {
      final settings = Settings(
        id: 5,
        uuid: 'setting-uuid',
        personId: 9,
        personUuid: 'person-uuid',
        language: 'pt_BR',
        theme: 'dark',
        notificationsEnabled: false,
        contextMenuPosition: ContextMenuPosition.bottom,
        homePage: Pages.gallery,
      );

      final proto = GrpcSettingsService.toProto(settings);
      expect(proto.id, 5);
      expect(proto.uuid, 'setting-uuid');
      expect(proto.ownerId, 9);
      expect(proto.ownerUuid, 'person-uuid');
      expect(proto.language, 'pt_BR');
      expect(proto.theme, 'dark');
      expect(proto.notificationsEnabled, false);
      expect(proto.contextMenuPosition, 'Bottom');
      expect(proto.homePage, 'gallery');

      // Simulate the wire round-trip (encode/decode) before mapping back.
      final decoded = $settings.Setting.fromBuffer(proto.writeToBuffer());
      final domain = GrpcSettingsService.toDomain(decoded);

      expect(domain.id, 5);
      expect(domain.uuid, 'setting-uuid');
      expect(domain.personId, 9);
      expect(domain.personUuid, 'person-uuid');
      expect(domain.language, 'pt_BR');
      expect(domain.theme, 'dark');
      expect(domain.notificationsEnabled, false);
      expect(domain.contextMenuPosition, ContextMenuPosition.bottom);
      expect(domain.homePage, Pages.gallery);
    });

    test('toDomain parses a lowercase contextMenuPosition from the backend without erroring', () {
      final proto = $settings.Setting(contextMenuPosition: 'left', homePage: 'feed');
      final domain = GrpcSettingsService.toDomain(proto);
      expect(domain.contextMenuPosition, ContextMenuPosition.left);
    });
  });
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `flutter test test/grpc_settings_service_test.dart`
Expected: FAIL with "Target of URI doesn't exist: 'package:socialgym_mobile/services/grpc/grpc_settings_service.dart'" (file doesn't exist yet).

- [ ] **Step 3: Write the implementation**

Create `lib/services/grpc/grpc_settings_service.dart`:

```dart
import 'package:grpc/grpc.dart' as grpc;
import 'package:socialgym_mobile/services/grpc/grpc_channel_factory.dart';

import '../../config/api_config.dart';
import '../../models/resource.dart';
import '../../src/generated/grpc/settings.pbgrpc.dart' as $settings;

/// gRPC client façade for the SettingsService.
///
/// Mirrors [GrpcPersonService]'s shape: a lazily-created generated client,
/// plus domain<->proto mapping kept next to the calls that need it.
class GrpcSettingsService {
  GrpcSettingsService._();

  static $settings.SettingsServiceClient? _client;

  /// Fetches the settings row for the given person.
  ///
  /// Returns `null` if the person has no settings row yet (fresh account) —
  /// callers should treat that as "use defaults", not as an error.
  static Future<Settings?> getByOwnerId({
    required int ownerId,
    String ownerUuid = '',
  }) async {
    try {
      final response = await _ensureClient().getByOwnerIds(
        $settings.SettingOwnerIdRequest(ownerId: ownerId, ownerUuid: ownerUuid),
        options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
      );
      return toDomain(response);
    } on grpc.GrpcError catch (e) {
      if (e.code == grpc.StatusCode.notFound) return null;
      rethrow;
    }
  }

  /// Persists (inserts or updates) the given settings and returns the
  /// server's copy. Callers must pass through the previously-loaded
  /// `id`/`uuid` (if any) or the backend will insert a duplicate row.
  static Future<Settings> persistSettings(Settings settings) async {
    final response = await _ensureClient().persistSettings(
      toProto(settings),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return toDomain(response);
  }

  /// Maps the app's [Settings] to the wire [$settings.Setting] message.
  static $settings.Setting toProto(Settings settings) {
    return $settings.Setting(
      id: settings.id ?? 0,
      uuid: settings.uuid ?? '',
      ownerId: settings.personId ?? 0,
      ownerUuid: settings.personUuid ?? '',
      language: settings.language ?? 'en',
      theme: settings.theme ?? 'default',
      notificationsEnabled: settings.notificationsEnabled ?? true,
      contextMenuPosition: positionToWire(
        settings.contextMenuPosition ?? ContextMenuPosition.left,
      ),
      homePage: (settings.homePage ?? Pages.feed).toStringValue(),
    );
  }

  /// Maps a wire [$settings.Setting] message back to the app's [Settings].
  static Settings toDomain($settings.Setting proto) {
    return Settings(
      id: proto.id,
      uuid: proto.uuid,
      personId: proto.ownerId,
      personUuid: proto.ownerUuid,
      language: proto.language,
      theme: proto.theme,
      notificationsEnabled: proto.notificationsEnabled,
      contextMenuPosition: ContextMenuPosition.fromString(proto.contextMenuPosition),
      homePage: Pages.fromString(proto.homePage),
      createdAt: proto.createdAt.isEmpty ? null : DateTime.tryParse(proto.createdAt),
      updatedAt: proto.updatedAt.isEmpty ? null : DateTime.tryParse(proto.updatedAt),
    );
  }

  /// The backend's `Position::from_string`
  /// (workout/business/src/domain/enums.rs) only matches capitalized values
  /// ("Left", "Top", "Right", "Bottom") and silently falls back to `Left`
  /// for anything else. [ContextMenuPosition]'s canonical Dart
  /// representation stays lowercase; this capitalizes only for the wire so
  /// the backend actually stores the chosen value.
  static String positionToWire(ContextMenuPosition position) {
    final name = position.toStringValue();
    return name[0].toUpperCase() + name.substring(1);
  }

  static $settings.SettingsServiceClient _ensureClient() {
    if (_client != null) return _client!;

    final channel = GrpcChannelFactory.channelFor(
      host: ApiConfig.grpcHost,
      port: ApiConfig.grpcPort,
      authority: ApiConfig.grpcAuthority,
    );
    _client = $settings.SettingsServiceClient(channel, interceptors: GrpcChannelFactory.interceptors);
    return _client!;
  }

  /// Closes the underlying gRPC client reference. Call on app shutdown.
  static Future<void> shutdown() async {
    _client = null;
  }
}
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `flutter test test/grpc_settings_service_test.dart`
Expected: PASS (all 3 tests).

- [ ] **Step 5: Commit**

```bash
git add lib/services/grpc/grpc_settings_service.dart test/grpc_settings_service_test.dart
git commit -m "Add GrpcSettingsService client wrapper for SettingsService"
```

---

### Task 4: `SettingsProvider` gRPC integration

**Files:**
- Modify: `lib/providers/settings_provider.dart`

**Interfaces:**
- Consumes: `GrpcSettingsService.getByOwnerId`, `GrpcSettingsService.persistSettings` (Task 3).
- Produces (for Task 5): `SettingsProvider.loading` (bool), `SettingsProvider.saving` (bool), `SettingsProvider.error` (String?), `Future<void> fetchFromServer({required int personId, String personUuid})`, `Future<bool> persistToServer(Settings updated)`.

No automated test for this task: it's thin orchestration over a live network call with no dependency-injection seam, and no mocking pattern exists in this repo to fake that boundary (see Global Constraints). Verified instead by `dart analyze` plus the manual end-to-end check in Task 8.

- [ ] **Step 1: Add state fields and getters**

In `lib/providers/settings_provider.dart`, after `bool _isLoaded = false;` add:

```dart
  bool _loading = false;
  bool _saving = false;
  String? _error;
```

and after `bool get isLoaded => _isLoaded;` add:

```dart
  bool get loading => _loading;
  bool get saving => _saving;
  String? get error => _error;
```

- [ ] **Step 2: Add the import**

At the top of the file, after `import '../models/resource.dart';` add:

```dart
import '../services/grpc/grpc_settings_service.dart';
```

- [ ] **Step 3: Add `fetchFromServer` and `persistToServer`**

At the end of the class, before the final closing `}`, add:

```dart
  /// Refreshes settings from the backend over gRPC.
  ///
  /// If the person has no settings row yet, the current cached/default
  /// settings are left untouched (not an error — just nothing to sync yet).
  /// Any other failure sets [error] but does not clear the current settings,
  /// since the page already has cached/default values to show.
  Future<void> fetchFromServer({required int personId, String personUuid = ''}) async {
    _loading = true;
    _error = null;
    notifyListeners();

    try {
      final fetched = await GrpcSettingsService.getByOwnerId(
        ownerId: personId,
        ownerUuid: personUuid,
      );
      if (fetched != null) {
        await applySettings(fetched);
      }
    } catch (e) {
      _error = 'Failed to refresh settings.';
    } finally {
      _loading = false;
      notifyListeners();
    }
  }

  /// Persists [updated] to the backend over gRPC. Returns `true` on success
  /// (and applies the server's response as the new local settings), `false`
  /// on failure (leaving the caller's form values untouched so nothing is
  /// lost).
  Future<bool> persistToServer(Settings updated) async {
    _saving = true;
    _error = null;
    notifyListeners();

    try {
      final persisted = await GrpcSettingsService.persistSettings(updated);
      await applySettings(persisted);
      return true;
    } catch (e) {
      _error = 'Failed to save settings.';
      return false;
    } finally {
      _saving = false;
      notifyListeners();
    }
  }
```

- [ ] **Step 4: Verify it compiles and existing tests still pass**

Run: `dart analyze lib/providers/settings_provider.dart && flutter test`
Expected: `No issues found!` and all tests PASS.

- [ ] **Step 5: Commit**

```bash
git add lib/providers/settings_provider.dart
git commit -m "Add gRPC fetch/persist to SettingsProvider"
```

---

### Task 5: `SettingsPage`

**Files:**
- Create: `lib/pages/settings/settings_page.dart`

**Interfaces:**
- Consumes: `SettingsProvider` (Task 4), `PersonProvider.person` (`lib/providers/person_provider.dart`, unchanged — `.id`, `.uuid` on the returned `Person`), `LocaleProvider` (`lib/providers/locale_provider.dart`, unchanged — `localeNames`, `currentLocaleKey`, `applyUserSettingsLanguage`), `MainLayout` (`lib/widgets/main_layout.dart`, unchanged), `Settings`/`ContextMenuPosition`/`Pages` (`lib/models/resource.dart`).
- Produces (for Task 6): `SettingsPage` widget (`const SettingsPage({super.key})`), to be routed at `/settings`.

No widget test for this page — no precedent exists for full-page widget tests in this repo (see Global Constraints). Verified via `dart analyze` plus the manual walkthrough in Task 8.

- [ ] **Step 1: Add the new l10n keys this page needs**

This page references localization keys added in Task 7. Write the page now (referencing keys that don't exist yet is fine — `flutter analyze` on the generated localizations file will simply fail until Task 7 adds them); Task 7 must run before the app builds. If you'd rather build task-by-task without a broken intermediate state, do Task 7 first and swap it with this task — there is no other dependency between them.

- [ ] **Step 2: Write `lib/pages/settings/settings_page.dart`**

```dart
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../l10n/app_localizations.dart';
import '../../models/resource.dart';
import '../../providers/locale_provider.dart';
import '../../providers/person_provider.dart';
import '../../providers/settings_provider.dart';
import '../../widgets/main_layout.dart';

class SettingsPage extends StatefulWidget {
  const SettingsPage({super.key});

  @override
  State<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  late String _language;
  late bool _notificationsEnabled;
  late ContextMenuPosition _contextMenuPosition;
  late Pages _homePage;
  bool _dirty = false;

  @override
  void initState() {
    super.initState();
    final current = context.read<SettingsProvider>().settings ?? Settings();
    _language = current.language ?? 'en';
    _notificationsEnabled = current.notificationsEnabled ?? true;
    _contextMenuPosition = current.contextMenuPosition ?? ContextMenuPosition.left;
    _homePage = current.homePage ?? Pages.feed;

    WidgetsBinding.instance.addPostFrameCallback((_) => _refreshFromServer());
  }

  Future<void> _refreshFromServer() async {
    final person = context.read<PersonProvider>().person;
    if (person == null) return;

    final settingsProvider = context.read<SettingsProvider>();
    await settingsProvider.fetchFromServer(personId: person.id, personUuid: person.uuid);
    if (!mounted || _dirty) return;

    final refreshed = settingsProvider.settings;
    if (refreshed == null) return;
    setState(() {
      _language = refreshed.language ?? _language;
      _notificationsEnabled = refreshed.notificationsEnabled ?? _notificationsEnabled;
      _contextMenuPosition = refreshed.contextMenuPosition ?? _contextMenuPosition;
      _homePage = refreshed.homePage ?? _homePage;
    });
  }

  Future<void> _save() async {
    final l10n = AppLocalizations.of(context)!;
    final person = context.read<PersonProvider>().person;
    if (person == null) return;

    final settingsProvider = context.read<SettingsProvider>();
    final base = settingsProvider.settings ?? Settings();
    final languageChanged = base.language != _language;

    final updated = base.copyWith(
      personId: person.id,
      personUuid: person.uuid,
      language: _language,
      notificationsEnabled: _notificationsEnabled,
      contextMenuPosition: _contextMenuPosition,
      homePage: _homePage,
    );

    final success = await settingsProvider.persistToServer(updated);
    if (!mounted) return;

    if (success) {
      setState(() => _dirty = false);
      if (languageChanged) {
        await context.read<LocaleProvider>().applyUserSettingsLanguage(_language);
      }
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(l10n.settingsSaveSuccess)));
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(l10n.settingsSaveError),
          action: SnackBarAction(label: l10n.buttonRetry, onPressed: _save),
        ),
      );
    }
  }

  String _homePageLabel(AppLocalizations l10n, Pages page) {
    switch (page) {
      case Pages.feed:
        return l10n.settingsHomePageFeed;
      case Pages.gallery:
        return l10n.settingsHomePageGallery;
    }
  }

  String _positionLabel(AppLocalizations l10n, ContextMenuPosition position) {
    switch (position) {
      case ContextMenuPosition.left:
        return l10n.settingsContextMenuPositionLeft;
      case ContextMenuPosition.top:
        return l10n.settingsContextMenuPositionTop;
      case ContextMenuPosition.right:
        return l10n.settingsContextMenuPositionRight;
      case ContextMenuPosition.bottom:
        return l10n.settingsContextMenuPositionBottom;
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final settingsProvider = context.watch<SettingsProvider>();

    return MainLayout(
      currentRoute: '/settings',
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(l10n.settingsTitle, style: Theme.of(context).textTheme.headlineSmall),
            const SizedBox(height: 16),
            if (settingsProvider.error != null && !settingsProvider.saving)
              Padding(
                padding: const EdgeInsets.only(bottom: 12),
                child: Text(
                  l10n.settingsLoadError,
                  style: const TextStyle(color: Colors.redAccent),
                ),
              ),
            Text(l10n.settingsLanguage, style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 8),
            DropdownButtonFormField<String>(
              initialValue: _language,
              items: LocaleProvider.localeNames.entries
                  .map((entry) => DropdownMenuItem(value: entry.key, child: Text(entry.value)))
                  .toList(),
              onChanged: (value) {
                if (value == null) return;
                setState(() {
                  _language = value;
                  _dirty = true;
                });
              },
            ),
            const SizedBox(height: 24),
            SwitchListTile(
              contentPadding: EdgeInsets.zero,
              title: Text(l10n.settingsNotificationsEnabled),
              value: _notificationsEnabled,
              onChanged: (value) => setState(() {
                _notificationsEnabled = value;
                _dirty = true;
              }),
            ),
            const SizedBox(height: 16),
            Text(l10n.settingsContextMenuPosition, style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 8),
            SegmentedButton<ContextMenuPosition>(
              segments: ContextMenuPosition.values
                  .map(
                    (position) => ButtonSegment(
                      value: position,
                      label: Text(_positionLabel(l10n, position)),
                    ),
                  )
                  .toList(),
              selected: {_contextMenuPosition},
              onSelectionChanged: (selection) => setState(() {
                _contextMenuPosition = selection.first;
                _dirty = true;
              }),
            ),
            const SizedBox(height: 24),
            Text(l10n.settingsHomePage, style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 8),
            DropdownButtonFormField<Pages>(
              initialValue: _homePage,
              items: Pages.values
                  .map((page) => DropdownMenuItem(value: page, child: Text(_homePageLabel(l10n, page))))
                  .toList(),
              onChanged: (value) {
                if (value == null) return;
                setState(() {
                  _homePage = value;
                  _dirty = true;
                });
              },
            ),
            const SizedBox(height: 32),
            SizedBox(
              width: double.infinity,
              child: FilledButton(
                onPressed: settingsProvider.saving ? null : _save,
                child: settingsProvider.saving
                    ? const SizedBox(
                        width: 20,
                        height: 20,
                        child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white),
                      )
                    : Text(l10n.buttonSave),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
```

- [ ] **Step 3: Verify it compiles (after Task 7 adds the l10n keys)**

Run: `dart analyze lib/pages/settings/settings_page.dart`
Expected: `No issues found!`

- [ ] **Step 4: Commit**

```bash
git add lib/pages/settings/settings_page.dart
git commit -m "Add SettingsPage"
```

---

### Task 6: Wire navigation

**Files:**
- Modify: `lib/main.dart`
- Modify: `lib/widgets/sidebar_menu.dart:287-293`
- Modify: `lib/widgets/profile_menu.dart:108-112` and `:238-247`

**Interfaces:**
- Consumes: `SettingsPage` (Task 5).

- [ ] **Step 1: Register the route in `lib/main.dart`**

Add the import alongside the other page imports (after `import 'pages/profile/profile_page.dart';`):

```dart
import 'pages/settings/settings_page.dart';
```

Add the route entry in the `routes` map (after `'/notifications': (context) => const NotificationsPage(),`):

```dart
              '/settings': (context) => const SettingsPage(),
```

- [ ] **Step 2: Wire the sidebar's Settings item**

In `lib/widgets/sidebar_menu.dart`, in `_buildAlwaysItems`, replace:

```dart
      _SidebarItem(
        icon: Icons.settings_outlined,
        activeIcon: Icons.settings,
        label: l10n.menuSettings,
        isCollapsed: isCollapsed,
        onTap: () => _showComingSoon(context, l10n),
      ),
```

with:

```dart
      _SidebarItem(
        icon: Icons.settings_outlined,
        activeIcon: Icons.settings,
        label: l10n.menuSettings,
        isActive: currentRoute == '/settings',
        isCollapsed: isCollapsed,
        onTap: () => _go(context, '/settings'),
      ),
```

- [ ] **Step 3: Wire `profile_menu.dart`'s `PopupMenuButton` path**

In `lib/widgets/profile_menu.dart`, in `_handleMenuAction`, replace:

```dart
      case 'settings':
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(AppLocalizations.of(context)!.comingSoon)),
        );
        break;
```

with:

```dart
      case 'settings':
        Navigator.of(context).pushNamedAndRemoveUntil('/settings', (route) => false);
        break;
```

- [ ] **Step 4: Wire `profile_menu.dart`'s modal (`_ProfileMenuModal`) path**

In the same file, in `_ProfileMenuModal.build`, replace:

```dart
                          _MenuOption(
                            icon: Icons.settings_outlined,
                            label: l10n.menuSettings,
                            onPressed: () {
                              onClose?.call();
                              ScaffoldMessenger.of(context).showSnackBar(
                                SnackBar(content: Text(l10n.comingSoon)),
                              );
                            },
                          ),
```

with:

```dart
                          _MenuOption(
                            icon: Icons.settings_outlined,
                            label: l10n.menuSettings,
                            onPressed: () {
                              onClose?.call();
                              Navigator.of(context).pushNamedAndRemoveUntil(
                                '/settings',
                                (route) => false,
                              );
                            },
                          ),
```

- [ ] **Step 5: Verify it compiles**

Run: `dart analyze lib/main.dart lib/widgets/sidebar_menu.dart lib/widgets/profile_menu.dart`
Expected: `No issues found!`

- [ ] **Step 6: Commit**

```bash
git add lib/main.dart lib/widgets/sidebar_menu.dart lib/widgets/profile_menu.dart
git commit -m "Wire Settings menu entries to the new SettingsPage"
```

---

### Task 7: Localization keys

**Files:**
- Modify: `lib/l10n/app_en.arb`
- Modify: `lib/l10n/app_es.arb`
- Modify: `lib/l10n/app_fr.arb`
- Modify: `lib/l10n/app_nl.arb`
- Modify: `lib/l10n/app_pt.arb`
- Modify: `lib/l10n/app_pt_BR.arb`

**Interfaces:**
- Produces: `AppLocalizations.settingsTitle`, `.settingsLanguage`, `.settingsNotificationsEnabled`, `.settingsContextMenuPosition`, `.settingsContextMenuPositionLeft/Top/Right/Bottom`, `.settingsHomePage`, `.settingsHomePageFeed/Gallery`, `.settingsSaveSuccess`, `.settingsSaveError`, `.settingsLoadError` — consumed by Task 5's `SettingsPage`.

- [ ] **Step 1: Add keys to `lib/l10n/app_en.arb`**

Insert immediately after the `"openSettings": "Open Settings",` line (before the blank line that precedes `"workoutTitle"`):

```json
  "settingsTitle": "Settings",
  "settingsLanguage": "Language",
  "settingsNotificationsEnabled": "Enable notifications",
  "settingsContextMenuPosition": "Menu position",
  "settingsContextMenuPositionLeft": "Left",
  "settingsContextMenuPositionTop": "Top",
  "settingsContextMenuPositionRight": "Right",
  "settingsContextMenuPositionBottom": "Bottom",
  "settingsHomePage": "Home page",
  "settingsHomePageFeed": "Feed",
  "settingsHomePageGallery": "Gallery",
  "settingsSaveSuccess": "Settings saved",
  "settingsSaveError": "Could not save settings",
  "settingsLoadError": "Could not refresh settings from the server",
```

- [ ] **Step 2: Add the Spanish translations to `lib/l10n/app_es.arb`**

Insert the equivalent block at the same relative position (after that file's `openSettings` key):

```json
  "settingsTitle": "Ajustes",
  "settingsLanguage": "Idioma",
  "settingsNotificationsEnabled": "Activar notificaciones",
  "settingsContextMenuPosition": "Posición del menú",
  "settingsContextMenuPositionLeft": "Izquierda",
  "settingsContextMenuPositionTop": "Arriba",
  "settingsContextMenuPositionRight": "Derecha",
  "settingsContextMenuPositionBottom": "Abajo",
  "settingsHomePage": "Página de inicio",
  "settingsHomePageFeed": "Feed",
  "settingsHomePageGallery": "Galería",
  "settingsSaveSuccess": "Ajustes guardados",
  "settingsSaveError": "No se pudieron guardar los ajustes",
  "settingsLoadError": "No se pudieron actualizar los ajustes desde el servidor",
```

- [ ] **Step 3: Add the French translations to `lib/l10n/app_fr.arb`**

```json
  "settingsTitle": "Paramètres",
  "settingsLanguage": "Langue",
  "settingsNotificationsEnabled": "Activer les notifications",
  "settingsContextMenuPosition": "Position du menu",
  "settingsContextMenuPositionLeft": "Gauche",
  "settingsContextMenuPositionTop": "Haut",
  "settingsContextMenuPositionRight": "Droite",
  "settingsContextMenuPositionBottom": "Bas",
  "settingsHomePage": "Page d'accueil",
  "settingsHomePageFeed": "Fil d'actualité",
  "settingsHomePageGallery": "Galerie",
  "settingsSaveSuccess": "Paramètres enregistrés",
  "settingsSaveError": "Impossible d'enregistrer les paramètres",
  "settingsLoadError": "Impossible d'actualiser les paramètres depuis le serveur",
```

- [ ] **Step 4: Add the Dutch translations to `lib/l10n/app_nl.arb`**

```json
  "settingsTitle": "Instellingen",
  "settingsLanguage": "Taal",
  "settingsNotificationsEnabled": "Meldingen inschakelen",
  "settingsContextMenuPosition": "Menupositie",
  "settingsContextMenuPositionLeft": "Links",
  "settingsContextMenuPositionTop": "Boven",
  "settingsContextMenuPositionRight": "Rechts",
  "settingsContextMenuPositionBottom": "Onder",
  "settingsHomePage": "Startpagina",
  "settingsHomePageFeed": "Feed",
  "settingsHomePageGallery": "Galerij",
  "settingsSaveSuccess": "Instellingen opgeslagen",
  "settingsSaveError": "Instellingen konden niet worden opgeslagen",
  "settingsLoadError": "Instellingen konden niet worden vernieuwd vanaf de server",
```

- [ ] **Step 5: Add the Portuguese translations to `lib/l10n/app_pt.arb` and `lib/l10n/app_pt_BR.arb`**

Use the same block in both files:

```json
  "settingsTitle": "Configurações",
  "settingsLanguage": "Idioma",
  "settingsNotificationsEnabled": "Ativar notificações",
  "settingsContextMenuPosition": "Posição do menu",
  "settingsContextMenuPositionLeft": "Esquerda",
  "settingsContextMenuPositionTop": "Topo",
  "settingsContextMenuPositionRight": "Direita",
  "settingsContextMenuPositionBottom": "Base",
  "settingsHomePage": "Página inicial",
  "settingsHomePageFeed": "Feed",
  "settingsHomePageGallery": "Galeria",
  "settingsSaveSuccess": "Configurações salvas",
  "settingsSaveError": "Não foi possível salvar as configurações",
  "settingsLoadError": "Não foi possível atualizar as configurações do servidor",
```

- [ ] **Step 6: Regenerate localizations**

Run: `flutter gen-l10n`
Expected: Completes with no errors; `lib/l10n/app_localizations*.dart` are updated/regenerated with the new getters.

- [ ] **Step 7: Verify the whole app compiles now**

Run: `flutter analyze`
Expected: `No issues found!` (this is the point where `SettingsPage`'s references to the new l10n keys, added in Task 5, finally resolve).

- [ ] **Step 8: Commit**

```bash
git add lib/l10n/
git commit -m "Add settings page localization keys"
```

---

### Task 8: Full verification pass

**Files:** none (verification only)

- [ ] **Step 1: Run the full analyzer**

Run: `flutter analyze`
Expected: `No issues found!`

- [ ] **Step 2: Run the full test suite**

Run: `flutter test`
Expected: All tests PASS, including the new `settings_copy_with_test.dart` and `grpc_settings_service_test.dart`.

- [ ] **Step 3: Manual smoke test**

Run the app against a real or local `workout` backend (`flutter run`), sign in, and:
1. Open Settings from the sidebar (mobile drawer) — confirm the form loads with either the account's existing settings or sensible defaults, no crash.
2. Change the language, notifications toggle, context menu position, and home page, then tap Save — confirm a success snackbar appears and the change to context menu position visibly changes where the profile popup renders (top/bottom vs. left/right), and the language actually switches the UI immediately.
3. Re-open Settings — confirm the changes persisted (reloaded from the server, not just local cache).
4. Open Settings from the profile menu (top-right avatar) — confirm it also navigates correctly.
5. Turn off networking (or point `ApiConfig` at an unreachable host) and tap Save — confirm the error banner/snackbar with Retry appears and the form doesn't lose the user's edits.

- [ ] **Step 4: Report results**

If all steps pass, the feature is complete. If step 3 finds a real backend issue (e.g., the `Position` capitalization workaround doesn't match production behavior), stop and re-check `workout/business/src/domain/enums.rs`'s `Position::from_string` match arms before changing anything else.
