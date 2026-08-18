# Business Profile creation + Professional mode — Design

**Date:** 2026-07-16
**Scope:** `lapidation_mobile` only. Backend (`workout`) changes are owned by the user; this spec documents the contracts the mobile code assumes.

## Goal

From the app-header profile menu, "Add New Profile" leads the user through choosing a business profile type (Personal Trainer or Gym), creating a minimal business profile over gRPC, and landing in a re-themed "professional mode" of the app with a dedicated business profile page. First-iteration feature focus is Personal Trainer; Gym uses the same creation flow.

## Background facts (verified in code)

- `/add-profile` is referenced in `lib/widgets/profile_menu.dart` (lines 174, 368) but **not registered** in `main.dart` routes — tapping it today throws. This feature fills that route.
- Dart gRPC stubs for `BusinessProfileService` already exist in `lib/src/generated/grpc/business_profile.pbgrpc.dart` (regenerated via `./tool/generate_proto.sh`).
- Backend `AddBusinessProfile` (workout `BusinessProfileUseCase::add`) persists the business profile **and** inserts the person↔profile relationship row — one RPC is sufficient for creation.
- Backend `business_type` values are the strings `"Professional"` (Personal Trainer) and `"Company"` (Gym) — see workout `domain/enums.rs` `ProfileType`.
- `AppColors` already defines the professional identity: `professionalSecondary` (#1B1795), `professionalSecondaryHover/Disabled`, `professionalGradient3`.
- Existing image upload uses a presigned-URL flow (`UploadService.uploadToS3`); the backend already exposes `GET /workout/api/business-profiles/{id}/upload/{image_type}`.
- Header tabs and sidebar items are data-driven from `NavSection` (`lib/config/nav_section.dart`) through `MainLayout`.
- `PersonProvider.switchProfile()` is currently a TODO stub; `person.profiles` (from REST `/me`) is a `List<Profile>` where `Profile` carries `personId/personUuid`, `profileId/profileUuid`, `type`, display name/avatar.

## Decisions (from brainstorming)

1. **Creation form fields:** business name, social name, tax id (all required). `business_type` comes from the tapped card. Logo/cover/addresses are handled on the business profile page, not the creation form.
2. **Active-profile state is local:** held in `PersonProvider`, persisted in SharedPreferences. No backend "switch" call from mobile.
3. **Professional Feed/Gallery/Workout:** reuse the existing pages unchanged; only theming and profile navigation change.
4. **Business profile page is a full edit page:** editable fields, logo/cover upload, address CRUD.
5. **Missing backend operations are assumed to exist as gRPC methods**; proto files may be changed and must be synced across the three copies. The user implements the workout side.
6. **Gym card is active** and leads to the same form with `business_type = "Company"`.
7. **Token claims:** after login the access token carries `profileType`, `activeBusinessProfileId`, `activeBusinessProfileUuid`. Mobile decodes the JWT payload (plain base64) to restore professional mode after sign-in.

## Architecture

### New files

| File | Purpose |
|---|---|
| `lib/pages/business_profile/add_profile_page.dart` | Type-selection page (two cards), route `/add-profile` |
| `lib/pages/business_profile/business_profile_sign_up_page.dart` | Creation form, route `/add-profile/form` |
| `lib/pages/business_profile/business_profile_page.dart` | Full edit page, route `/business-profile` |
| `lib/models/business_profile.dart` | Domain model + `fromProto`/`toProto` mapping |
| `lib/models/business_profile_address.dart` | Address model + proto mapping |
| `lib/services/grpc/grpc_business_profile_service.dart` | gRPC client façade (mirrors `GrpcPersonService`) |
| `lib/providers/business_profile_provider.dart` | Active business profile data + edit operations |
| `lib/utils/jwt_decoder.dart` | Base64 JWT payload decode (no new package) |

### Modified files

| File | Change |
|---|---|
| `lib/main.dart` | Register the 3 new routes; make theme react to professional mode; register `BusinessProfileProvider` |
| `lib/providers/person_provider.dart` | `activeProfile`, `isProfessional`, real `switchProfile`, persistence, clear on logout |
| `lib/widgets/app_header.dart` | Gradient swap when professional |
| `lib/widgets/sidebar_menu.dart` | Profile item routes to `/business-profile` when professional |
| `lib/widgets/main_layout.dart` | `onProfilePressed` routes to `/business-profile` when professional |
| `lib/widgets/profile_menu.dart` | Tapping a profile entry triggers real switch; "Personal account" entry switches back |
| `lib/l10n/*.arb` | New strings (card titles/descriptions, form labels, errors, page titles) |
| `proto/business_profile.proto` (+ mirrors) | New RPCs (below) |

### 1. Type-selection page — `/add-profile`

`AddProfilePage`, wrapped in `MainLayout`. A const list of card descriptors (asset image, l10n title key, l10n description key, `businessType` string) renders one card per type:

- **Personal Trainer** → `business_type = "Professional"`
- **Gym** → `business_type = "Company"`

Hard-coded now; the const list is the seam for a future backend-driven catalog. Tapping a card: `Navigator.pushNamed('/add-profile/form', arguments: businessType)`.

Card images: two new bundled assets under `assets/images/` (placeholder illustrations acceptable initially; registered in `pubspec.yaml`).

### 2. Creation form — `/add-profile/form`

`BusinessProfileSignUpPage`, visual language copied from `sign_up_page.dart` (validators, loading state, error banner). Fields:

- Business name — required
- Social name — required
- Tax id — required

Submit builds a proto `BusinessProfile` with `ownerId = person.id`, `ownerUuid = person.uuid`, the three fields, and `businessType` from the route argument, then calls the existing `AddBusinessProfile` RPC via `GrpcBusinessProfileService`.

On success, in order:
1. `PersonProvider.fetchMe(token)` — refreshes `person.profiles` to include the new profile.
2. `PersonProvider.switchProfile(index, token)` — enters professional mode; the index is found by matching the returned business profile uuid against `person.profiles[].profileUuid`.
3. `BusinessProfileProvider.load(uuid)` — fetches full business profile.
4. `pushNamedAndRemoveUntil('/feed')` — professional home.

Failure: inline error banner, form stays filled, retry allowed.

### 3. Professional mode state

**`PersonProvider` additions:**

- `Profile? _activeProfile` — `null` means personal mode.
- `bool get isProfessional => _activeProfile != null`.
- `Future<bool> switchProfile(int index, String token)` — implemented for real: sets `_activeProfile`, persists to SharedPreferences (key `active_profile`, JSON), notifies. Index maps into `person.profiles`; a sentinel (index `-1` or dedicated method `switchToPersonal()`) clears it.
- `clear()` (logout) also clears `_activeProfile` and its stored key.
- `_loadFromStorage()` also restores `active_profile`.

**`BusinessProfileProvider`:**

- `BusinessProfile? current`, `loading/updating/error` fields following the `PersonProvider` pattern.
- `load({int? id, String? uuid})` → `GetBusinessProfileById`.
- `update(BusinessProfile)` → `UpdateBusinessProfile`.
- `addAddress/updateAddress/removeAddress` → the new address RPCs, then re-`load`.
- `uploadLogo/uploadCover(XFile)` → presigned URL via REST route + `UploadService.uploadToS3`, then re-`load`.
- Cleared on logout and when switching back to personal.

**Login restore:** after successful sign-in, decode the JWT payload (split on `.`, base64Url-decode segment 1, `jsonDecode`). If `profileType` indicates a business profile and `activeBusinessProfileUuid` is non-empty, find the matching entry in `person.profiles` and set it active; otherwise personal mode. Missing claims → personal mode (backward compatible).

### 4. Theming & navigation

- `main.dart`: the existing `Consumer<LocaleProvider>` becomes a `Consumer2<LocaleProvider, PersonProvider>` (or nested `Selector` on `isProfessional`). Theme when professional: `ColorScheme.fromSeed(seedColor: AppColors.professionalSecondary, surface: AppColors.background)`.
- `AppHeader`: `gradient: isProfessional ? AppColors.professionalGradient3 : AppColors.gradient3` (read via `context.watch<PersonProvider>()`).
- `NavSection` enum unchanged — Feed/Gallery/Workout tabs remain, pages reused as-is.
- Sidebar `Profile` item and header `onProfilePressed`: destination is `/business-profile` when `isProfessional`, else `/profile`.
- `ProfileMenu`: profile entries call the real `switchProfile`; add a "Personal account" entry (person's own name/avatar) that switches back to personal mode. After any switch, navigate to `/feed` (root of the new identity).

### 5. Business profile page — `/business-profile`

`BusinessProfilePage`, wrapped in `MainLayout`, modeled on the person `ProfilePage`:

- **Cover + logo** with pick/upload actions using the existing presigned-URL route `GET /workout/api/business-profiles/{id}/upload/{image_type}` and `UploadService.uploadToS3`.
- **Fields section**: business name, social name, tax id — editable, saved through `UpdateBusinessProfile`. Business type displayed read-only.
- **Addresses section**: list of `BusinessProfileAddress`; add/edit via a form (fields per `business_profile_address.proto`), delete with confirmation. Each mutation goes through its RPC and refreshes the profile.

If the page is opened while `BusinessProfileProvider.current` is null (e.g. app restart), it self-loads using `activeProfile.profileUuid`.

### 6. gRPC contract additions

Append to `BusinessProfileService` in `proto/business_profile.proto`, then sync byte-identical copies to `workout/integration/proto/` and `timeline/business/proto/`, and run `./tool/generate_proto.sh`:

```proto
rpc UpdateBusinessProfile(BusinessProfile) returns (BusinessProfile);
rpc AddBusinessProfileAddress(grpc.business_profile_address.BusinessProfileAddress) returns (grpc.business_profile_address.BusinessProfileAddress);
rpc UpdateBusinessProfileAddress(grpc.business_profile_address.BusinessProfileAddress) returns (grpc.business_profile_address.BusinessProfileAddress);
rpc RemoveBusinessProfileAddress(RemoveBusinessProfileAddressRequest) returns (RemoveBusinessProfileAddressResponse);

message RemoveBusinessProfileAddressRequest {
  int32 id = 1;
  string uuid = 2;
}

message RemoveBusinessProfileAddressResponse {
  bool success = 1;
}
```

Workout-side implementation of these methods is out of scope here (user-owned). Mobile treats them as existing; until the server implements them, calls return `UNIMPLEMENTED`, which surfaces through the standard error banner.

`GrpcBusinessProfileService` mirrors `GrpcPersonService`: private static client, `GrpcChannelFactory.channelFor(...)` with `ApiConfig` host/port/authority, `GrpcChannelFactory.interceptors`, 5-second `CallOptions` timeout, `shutdown()` hook.

### 7. Error handling

- gRPC/REST failures set the provider `error` string; pages show the same inline error banner pattern as `sign_up_page.dart`.
- Creation is a single RPC; no partial-state cleanup needed on mobile (backend owns transactional behavior).
- Malformed/missing JWT claims degrade to personal mode silently.

### 8. Testing

Under `test/`, following the repo's existing test conventions:

- **Unit:** `PersonProvider` switch/restore/persistence logic (SharedPreferences mock); JWT payload decoding (valid, missing claims, malformed token); `BusinessProfile` proto↔model mapping.
- **Widget:** `AddProfilePage` renders both cards and passes the right `businessType`; form validation on `BusinessProfileSignUpPage`; `AppHeader` gradient switches with professional mode.
- gRPC service façades tested with mocked clients (as done for `GrpcPersonService` consumers).

## Out of scope

- Personal-trainer-specific features beyond this flow (future iterations).
- Professional-specific content for Feed/Gallery/Workout.
- Any `workout`/`timeline` backend implementation (user-owned; contracts above).
- Web frontend changes.
