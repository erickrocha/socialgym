# Mobile User Settings Edit Page — Design

Date: 2026-07-08
Branch: `feature/notification_and_settings`

## Goal

Add a page in `lapidation_mobile` where the logged-in user can view and edit their
`Settings` (language, notification preference, context-menu position, home page),
persisted through the `workout` service's gRPC `SettingsService`
(`workout/integration/proto/settings.proto`). The backend (entity, gateway, use
case, gRPC service, migration) is already implemented and out of scope — this
spec covers only the Flutter app.

`theme` exists on the `Setting` proto/domain model but is not wired to any actual
theming logic anywhere in the app today (no dark/light mode exists). It is
carried through save/load so it isn't lost, but is not exposed as an editable
field in this iteration.

## Current state (relevant facts from exploration)

- `lapidation_mobile/proto/settings.proto` is already staged on this branch and
  byte-for-byte identical to `workout/integration/proto/settings.proto`. No
  proto changes needed.
- No generated Dart gRPC stubs exist yet for settings
  (`lib/src/generated/grpc/settings.pb*.dart` are missing).
- `lib/providers/settings_provider.dart` exists today as a **local-storage-only**
  cache: `applySettings()`/`_saveToStorage()` persist to `SharedPreferences`,
  and there's no network call anywhere in this provider. It's populated once at
  login via a REST call (`resourceProvider.fetchResources(...,
  onSettingsReceived: ...)` in `sign_in_page.dart`), not gRPC.
- `lib/models/resource.dart`'s `Settings.copyWith` has a bug: it only threads
  `language`, `homePage`, `contextMenuPosition` through and silently resets
  `id`, `uuid`, `personId`, `personUuid`, `theme`, `notificationsEnabled` to
  constructor defaults (`id: 0`, `personId: 0`, etc.) on every call. The
  backend's `SettingsGateway::persist` (`workout/business/src/gateway/settings_gateway.rs`)
  calls SeaORM's `ActiveModel::save()`, which **inserts a new row whenever `id`
  is unset** rather than updating in place. This means any current caller of
  `copyWith` (e.g. `SettingsProvider.setLanguage`) already silently orphans the
  settings row it meant to update. This must be fixed as part of this work,
  since the new Save flow depends on round-tripping `id`/`uuid` correctly.
- Two existing UI entry points already say "Settings" but are dead:
  `lib/widgets/sidebar_menu.dart` (`_showComingSoon`) and
  `lib/widgets/profile_menu.dart` (snackbar with `l10n.comingSoon`). Both are to
  be wired to the new page.
- Auth token / TLS is handled transparently for all gRPC calls via the global
  `GrpcAuthInterceptor` + `GrpcChannelFactory` (see
  `lib/services/grpc/grpc_channel_factory.dart`) — no per-call auth wiring
  needed, same as `GrpcPersonService`.
- `l10n.yaml` uses `flutter gen-l10n` (`generate: true` in `pubspec.yaml`);
  ARB files live in `lib/l10n/app_{en,es,fr,nl,pt,pt_BR}.arb` and localizations
  are generated, not hand-written, from those.
- No existing test in this repo mocks the gRPC client boundary — the one
  precedent (`test/grpc_person_service_test.dart`) only round-trips proto
  message encode/decode. There is no widget-test precedent for full pages
  either (`NotificationsPage` has none). Testing in this spec follows those
  precedents rather than introducing a new mocking pattern.

## Architecture

Follows the existing `GrpcPersonService` → provider → page pattern used
throughout this codebase (no new abstractions):

```
SettingsPage (UI, form state)
      │  reads/calls
      ▼
SettingsProvider (ChangeNotifier, already in MultiProvider)
      │  calls
      ▼
GrpcSettingsService (static class, new)
      │  uses generated stub
      ▼
SettingsServiceClient (generated from settings.proto)
      │  gRPC (TLS + auth interceptor, already global)
      ▼
workout::integration::GrpcSettingService (already implemented)
```

## Components

### 1. Generated gRPC stubs

Run `./tool/generate_proto.sh` to produce
`lib/src/generated/grpc/settings.pb.dart`, `settings.pbgrpc.dart`,
`settings.pbenum.dart`, `settings.pbjson.dart` from the existing
`proto/settings.proto`. No `.proto` edits.

### 2. Fix `Settings.copyWith` (`lib/models/resource.dart`)

Rewrite to thread every field through by default, and add a
`notificationsEnabled` parameter (needed for the new toggle):

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

Existing callers (`setLanguage`, `setContextMenuPosition`, home-page setter)
keep working unchanged but stop losing `id`/`personId`/`personUuid`.

### 3. `GrpcSettingsService` (new file `lib/services/grpc/grpc_settings_service.dart`)

Static class mirroring `GrpcPersonService`:

- `static Future<Settings?> getByOwnerId({required int ownerId, String ownerUuid = ''})`
  — calls `SettingIdRequest`/`SettingOwnerIdRequest` → `GetByOwnerIds`; maps
  proto `Setting` → `Settings`. Returns `null` if the call fails with
  `StatusCode.notFound` (new user, no settings row yet) — does not throw for
  that case. Any other `GrpcError` is rethrown for the caller to handle.
- `static Future<Settings> persistSettings(Settings settings)` — maps
  `Settings` → proto `Setting` (including `id`/`uuid` so the backend updates
  in place rather than inserting), calls `PersistSettings`, maps the response
  back to `Settings`.
- Lazily-built static `SettingsServiceClient` via
  `GrpcChannelFactory.channelFor(...)` + `GrpcChannelFactory.interceptors`,
  same as `GrpcPersonService`. `shutdown()` method for symmetry.

### 4. `SettingsProvider` extensions (`lib/providers/settings_provider.dart`)

New state: `bool loading`, `bool saving`, `String? error` (mirrors
`NotificationsProvider`'s `loading`/`error` fields).

New methods:

- `Future<void> fetchFromServer({required int personId, String personUuid = ''})`
  — sets `loading = true`, calls `GrpcSettingsService.getByOwnerId`; on
  success calls `applySettings(...)`; on `null` (not found) leaves current
  cached/default settings as-is; on thrown error sets `error` (non-fatal —
  the page already has cached/default values to show). Always sets
  `loading = false` and notifies.
- `Future<bool> persistToServer(Settings updated)` — sets `saving = true`,
  clears `error`, calls `GrpcSettingsService.persistSettings`; on success
  calls `applySettings(response)` and returns `true`; on failure sets `error`
  and returns `false` (caller keeps the form open with the user's edits
  intact — no data loss on a failed save). Always sets `saving = false` and
  notifies.

### 5. `SettingsPage` (new file `lib/pages/settings/settings_page.dart`)

`StatefulWidget` wrapped in `MainLayout` (same shell as
`NotificationsPage`/`FriendsPage`), route `/settings`.

**Load flow:** `initState` reads `context.read<SettingsProvider>().settings`
(or `Settings()` defaults if null) synchronously to seed local form fields, so
the form renders immediately with no spinner. A post-frame callback then calls
`fetchFromServer(personId: ..., personUuid: ...)` (from
`context.read<PersonProvider>().person`) to refresh in the background. A
local `_dirty` flag (set `true` on any field change) prevents the background
refresh from clobbering in-progress edits — if `_dirty` is `true` when the
fetch completes, the fetched values are discarded (the form already has newer
user input).

**Form fields** (local `State` variables, initialized from the loaded
`Settings`):

| Field | Widget | Source of options |
|---|---|---|
| Language | Dropdown | Same locale list used by `LanguageSelectorButton` (`lib/widgets/language_selector.dart`) |
| Notifications enabled | `SwitchListTile` | bool |
| Context menu position | Segmented control | `ContextMenuPosition.values` |
| Home page | Dropdown | `Pages.values` |

**Save button:** disabled while `saving`. On tap:
1. Build updated `Settings` via the fixed `copyWith`, carrying the
   originally-loaded `id`/`uuid`/`personId`/`personUuid`/`theme` forward.
2. Call `persistToServer(updated)`.
3. On success: snackbar confirmation, clear `_dirty`; if `language` changed,
   call `context.read<LocaleProvider>().applyUserSettingsLanguage(newLanguage)`
   so the change applies immediately (mirrors the existing sign-in flow in
   `sign_in_page.dart`).
4. On failure: inline error banner with a Retry action (same visual pattern as
   `NotificationsProvider`'s error banner in `NotificationsPage`), edits
   preserved, user can retry without re-entering anything.

### 6. Navigation wiring

- `lib/main.dart`: register `'/settings': (context) => const SettingsPage()`
  in the `MaterialApp.routes` map.
- `lib/widgets/sidebar_menu.dart`: replace the settings item's
  `onTap: () => _showComingSoon(context, l10n)` with
  `onTap: () => _go(context, '/settings')`.
- `lib/widgets/profile_menu.dart`: replace the settings `_MenuOption`'s
  snackbar body with
  `onClose?.call(); Navigator.of(context).pushNamedAndRemoveUntil('/settings', (route) => false);`
  (same pattern as the Friends option immediately above it).

### 7. i18n

Add keys to all six ARB files (`app_en.arb`, `app_es.arb`, `app_fr.arb`,
`app_nl.arb`, `app_pt.arb`, `app_pt_BR.arb`):
`settingsTitle`, `settingsLanguage`, `settingsNotificationsEnabled`,
`settingsContextMenuPosition`, `settingsContextMenuPositionLeft/Top/Right/Bottom`,
`settingsHomePage`, `settingsHomePageFeed`, `settingsHomePageGallery`,
`settingsSave`, `settingsSaveSuccess`, `settingsSaveError`. Run
`flutter gen-l10n` (or `flutter pub get`, which triggers it) to regenerate
`app_localizations*.dart`. `l10n.menuSettings` and `l10n.comingSoon` already
exist and are unaffected.

## Error handling

- **Load failure** (network/backend error other than not-found): non-blocking
  — form still shows cached/default values; a small dismissible banner notes
  the refresh failed, no retry button needed (Save will attempt a fresh write
  regardless).
- **Save failure**: blocking-in-place — Save button re-enables, inline error
  banner with Retry, form values preserved untouched so the user doesn't
  retype anything.
- **No settings row yet** (new user, `getByOwnerId` → not found): treated as
  success with defaults; first Save performs an insert (`id` unset → backend
  inserts, matches existing "new settings" path already used at signup).

## Testing

- Regression unit test for the fixed `Settings.copyWith` (asserts `id`,
  `personId`, `personUuid`, `theme`, `notificationsEnabled` survive a
  single-field update) — closes the bug found during exploration.
- Proto round-trip test for the `Setting` message analogous to
  `test/grpc_person_service_test.dart` (encode/decode via
  `writeToBuffer`/`fromBuffer`, confirming field mapping in
  `GrpcSettingsService`'s proto↔domain conversion functions, tested as pure
  functions rather than through a live/mocked network client).
- No widget/golden test for `SettingsPage` and no gRPC-client-mocking test for
  `SettingsProvider` — neither pattern exists elsewhere in this codebase
  (confirmed: `NotificationsPage` has no widget test; no test in the repo
  mocks a generated gRPC client), so this spec does not introduce one.

## Out of scope

- Backend changes (already complete).
- Wiring `theme` to actual app theming.
- Changing how settings are fetched at login (`resourceProvider.fetchResources`
  REST flow stays as-is; this page is an independent gRPC-backed read/write
  path layered on top of the same `SettingsProvider`).
