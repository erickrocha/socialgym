# Business Profile Creation + Professional Mode Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** From the app-header profile menu, "Add New Profile" leads the user through choosing a business profile type (Personal Trainer or Gym), creates a minimal business profile over gRPC (`AddBusinessProfile`), and switches the app into a re-themed "professional mode" with its own business profile page — Personal Trainer is this iteration's feature focus, Gym reuses the same flow.

**Architecture:** New `AddProfilePage` (type cards) → `BusinessProfileSignUpPage` (creation form, styled like `SignUpPage`) → `GrpcBusinessProfileService` (static client façade mirroring `GrpcPersonService`) → workout's `AddBusinessProfile` RPC (already implemented; also inserts the `Profile` relationship row). On success, `PersonProvider` gains a local `activeProfile` (no backend "switch" call) that flips `isProfessional`, driving `AppHeader`'s gradient, `MainLayout`/`SidebarMenu`'s Profile-item destination, and a new full-edit `BusinessProfilePage` backed by `BusinessProfileProvider`. Login restores professional mode by decoding JWT claims (`profileType`, `activeBusinessProfileId`, `activeBusinessProfileUuid`) the user will add to the access token server-side.

**Tech Stack:** Flutter 3 / Dart 3.11, `provider` (ChangeNotifier), `grpc` package with generated protobuf stubs (`protoc` + `protoc-gen-dart`), `shared_preferences` for local persistence, `flutter gen-l10n` for ARB-based i18n, `image_picker` + presigned-S3-URL uploads (existing `UploadService` pattern).

## Global Constraints

- Work happens only in `lapidation_mobile/`. `workout` and `timeline` are separate git repos the user owns — this plan only touches their `.proto` **text files** (to keep the wire contract in sync, per the approved spec) and never touches any `.rs` file. After the proto sync, `cargo build` in `workout`/`timeline` will fail to compile until the user implements the new RPC methods server-side — that is expected and out of scope here.
- `business_type` wire values are exactly the strings `"Professional"` (Personal Trainer) and `"Company"` (Gym) — see workout's `domain/enums.rs` `ProfileType`. Never send any other string.
- Active-profile state is **local only** (`PersonProvider`, persisted to `SharedPreferences` under key `active_profile`) — there is no backend "switch profile" call.
- Follow the existing static-service-class gRPC façade pattern (`GrpcPersonService`): private static client, lazy `_ensureClient()` via `GrpcChannelFactory.channelFor(host: ApiConfig.grpcHost, port: ApiConfig.grpcPort, authority: ApiConfig.grpcAuthority)`, `CallOptions(timeout: Duration(seconds: 5))`, static `shutdown()`. No repository/interface abstraction.
- No widget-test mocking framework exists anywhere in this repo (confirmed: the one gRPC test, `test/grpc_person_service_test.dart`, only round-trips proto message encode/decode via `writeToBuffer`/`fromBuffer`). Don't introduce a new mocking framework — follow that precedent for `GrpcBusinessProfileService` and model tests.
- Six ARB locale files exist: `app_en.arb`, `app_es.arb`, `app_fr.arb`, `app_nl.arb`, `app_pt.arb`, `app_pt_BR.arb`. Add new keys to all six.
- `flutter`, `protoc` (3.21.12), and `protoc-gen-dart` (protoc_plugin 25.0.0) are installed and on `PATH` in this environment; `./tool/generate_proto.sh` regenerates `lib/src/generated/grpc/` from `proto/*.proto`.
- proto3 scalar fields in this codebase are **not** marked `optional` unless the generated Dart already exposes a `hasXxx()` (e.g. `Person.avatar`/`objectKey` do; `BusinessProfile`'s `id`/`uuid`/`logo`/etc. do not). Treat unset business-profile scalars via zero/empty-string sentinels (`id == 0` ⇒ unset, `uuid.isEmpty` ⇒ unset), not `hasXxx()`.
- Card images reuse existing bundled assets (`assets/images/avatar_male.png`, `assets/images/cover_foto.png`) already registered in `pubspec.yaml` — no new binary assets or `pubspec.yaml` changes needed.
- Out of scope: personal-trainer-specific features beyond this flow, professional-specific Feed/Gallery/Workout content, any `workout`/`timeline` Rust implementation, web frontend changes.

---

### Task 1: Sync the gRPC proto contract and regenerate stubs

**Files:**
- Modify: `proto/business_profile.proto`
- Modify: `../workout/integration/proto/business_profile.proto` (text sync only)
- Modify: `../timeline/business/proto/business_profile.proto` (text sync only)
- Run: `tool/generate_proto.sh`
- Produces: `lib/src/generated/grpc/business_profile.pb.dart`, `.pbgrpc.dart`, `.pbenum.dart`, `.pbjson.dart`

**Interfaces:**
- Produces (for later tasks): `$bp.BusinessProfile` (fields: `id` int, `uuid` String, `ownerId` int, `ownerUuid` String, `taxId` String, `businessName` String, `businessType` String, `socialName` String, `logo` String, `coverImage` String, `createdAt` String, `updatedAt` String, `addresses` `List<$bpa.BusinessProfileAddress>`), `$bp.BusinessProfileRequestId(id, uuid)`, `$bp.BusinessProfileRequestOwnerId(ownerId, ownerUuid)`, `$bp.BusinessProfilesResponse(businessProfiles)`, `$bp.RemoveBusinessProfileAddressRequest(id, uuid)`, `$bp.RemoveBusinessProfileAddressResponse(success)`, `$bp.BusinessProfileServiceClient` with methods `getBusinessProfileById`, `getBusinessProfileByOwnerId`, `addBusinessProfile`, `updateBusinessProfile`, `addBusinessProfileAddress`, `updateBusinessProfileAddress`, `removeBusinessProfileAddress` (each `Future<Response> Function(Request, {CallOptions? options})`). `$bpa.BusinessProfileAddress` (fields: `id` int, `uuid` String, `businessProfileId` int, `addressLine1` String, `addressLine2` String, `locality` String, `administrativeArea` String, `postalCode` String, `countryCode` String, `latitude` double, `longitude` double, `createdAt` String, `updatedAt` String) — unchanged, already generated correctly.

- [ ] **Step 1: Replace `proto/business_profile.proto`**

```proto
syntax = "proto3";
package grpc.business_profile;

import "business_profile_address.proto";

message BusinessProfile {
  int32 id = 1;
  string uuid = 2;
  int32 owner_id = 3;
  string owner_uuid = 4;
  string tax_id = 5;
  string business_name = 6;
  string business_type = 7;
  string social_name = 8;
  string logo = 9;
  string cover_image = 10;
  string created_at = 11;
  string updated_at = 12;
  repeated grpc.business_profile_address.BusinessProfileAddress addresses = 13;
}

service BusinessProfileService {
  rpc GetBusinessProfileById (BusinessProfileRequestId) returns (BusinessProfile);
  rpc GetBusinessProfileByOwnerId (BusinessProfileRequestOwnerId) returns (BusinessProfilesResponse);
  rpc AddBusinessProfile(BusinessProfile) returns (BusinessProfile);
  rpc UpdateBusinessProfile(BusinessProfile) returns (BusinessProfile);
  rpc AddBusinessProfileAddress(grpc.business_profile_address.BusinessProfileAddress) returns (grpc.business_profile_address.BusinessProfileAddress);
  rpc UpdateBusinessProfileAddress(grpc.business_profile_address.BusinessProfileAddress) returns (grpc.business_profile_address.BusinessProfileAddress);
  rpc RemoveBusinessProfileAddress(RemoveBusinessProfileAddressRequest) returns (RemoveBusinessProfileAddressResponse);
}

message BusinessProfileRequestId {
  int32 id = 1;
  string uuid = 2;
}

message BusinessProfileRequestOwnerId {
  int32 owner_id = 1;
  string owner_uuid = 2;
}

message BusinessProfilesResponse {
  repeated BusinessProfile business_profiles = 1;
}

message RemoveBusinessProfileAddressRequest {
  int32 id = 1;
  string uuid = 2;
}

message RemoveBusinessProfileAddressResponse {
  bool success = 1;
}
```

Note the RPC name change from `GetBusinessProfilesByOwnerId` to `GetBusinessProfileByOwnerId` — this fixes a pre-existing mismatch against `workout`'s and `timeline`'s copies (both already use the singular form), so the wire name now matches what `workout`'s `GrpcBusinessProfileService::get_business_profile_by_owner_id` already serves.

- [ ] **Step 2: Copy the identical file to the other two repos**

```bash
cd /home/erocha/workspace/lapidation_project/lapidation_mobile
cp proto/business_profile.proto ../workout/integration/proto/business_profile.proto
cp proto/business_profile.proto ../timeline/business/proto/business_profile.proto
diff proto/business_profile.proto ../workout/integration/proto/business_profile.proto
diff proto/business_profile.proto ../timeline/business/proto/business_profile.proto
```

Expected: both `diff` commands print nothing (files identical).

- [ ] **Step 3: Regenerate the mobile Dart stubs**

```bash
./tool/generate_proto.sh
```

Expected output: `Generated Dart gRPC files in .../lib/src/generated/grpc` with no errors.

- [ ] **Step 4: Verify the new RPCs are present in the generated client**

```bash
grep -n "addBusinessProfile\|updateBusinessProfile\|getBusinessProfileById\|getBusinessProfileByOwnerId\|addBusinessProfileAddress\|removeBusinessProfileAddress" lib/src/generated/grpc/business_profile.pbgrpc.dart
```

Expected: all six method names appear (the stale `getBusinessProfile`/`BusinessProfileRequest` single-RPC shape is gone).

- [ ] **Step 5: Verify the whole project still analyzes cleanly**

```bash
flutter pub get && dart analyze lib/src/generated/grpc/business_profile.pb.dart lib/src/generated/grpc/business_profile.pbgrpc.dart
```

Expected: `No issues found!`

- [ ] **Step 6: Commit**

```bash
git add proto/business_profile.proto lib/src/generated/grpc/business_profile.pb.dart lib/src/generated/grpc/business_profile.pbenum.dart lib/src/generated/grpc/business_profile.pbgrpc.dart lib/src/generated/grpc/business_profile.pbjson.dart
git commit -m "Sync business_profile.proto contract and regenerate gRPC stubs"
```

(The copies under `../workout/` and `../timeline/` are in separate git repos the user owns — leave those working-tree changes uncommitted for the user to review and commit alongside their own server-side implementation.)

---

### Task 2: JWT payload decoder

**Files:**
- Create: `lib/utils/jwt_decoder.dart`
- Test: `test/jwt_decoder_test.dart`

**Interfaces:**
- Produces: `JwtClaims(profileType: String?, activeBusinessProfileId: int?, activeBusinessProfileUuid: String?)` with `bool get isBusinessProfile`; `JwtDecoder.decode(String token) -> JwtClaims?` (`null` on any malformed input).

- [ ] **Step 1: Write the failing test**

Create `test/jwt_decoder_test.dart`:

```dart
import 'dart:convert';

import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/utils/jwt_decoder.dart';

String _fakeToken(Map<String, dynamic> payload) {
  final header = base64Url.encode(utf8.encode(jsonEncode({'alg': 'none'}))).replaceAll('=', '');
  final body = base64Url.encode(utf8.encode(jsonEncode(payload))).replaceAll('=', '');
  return '$header.$body.signature';
}

void main() {
  group('JwtDecoder.decode', () {
    test('decodes profileType and active business profile claims', () {
      final token = _fakeToken({
        'profileType': 'Professional',
        'activeBusinessProfileId': 42,
        'activeBusinessProfileUuid': 'bp-uuid-1',
      });

      final claims = JwtDecoder.decode(token);

      expect(claims, isNotNull);
      expect(claims!.profileType, 'Professional');
      expect(claims.activeBusinessProfileId, 42);
      expect(claims.activeBusinessProfileUuid, 'bp-uuid-1');
      expect(claims.isBusinessProfile, isTrue);
    });

    test('missing claims degrade to personal mode', () {
      final token = _fakeToken({'sub': 'someone'});

      final claims = JwtDecoder.decode(token);

      expect(claims, isNotNull);
      expect(claims!.activeBusinessProfileUuid, isNull);
      expect(claims.isBusinessProfile, isFalse);
    });

    test('malformed token returns null', () {
      expect(JwtDecoder.decode('not-a-jwt'), isNull);
      expect(JwtDecoder.decode(''), isNull);
    });
  });
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `flutter test test/jwt_decoder_test.dart`
Expected: FAIL — `Error: Couldn't resolve the package 'lapidation_mobile/utils/jwt_decoder.dart'` (file doesn't exist yet).

- [ ] **Step 3: Write the implementation**

Create `lib/utils/jwt_decoder.dart`:

```dart
import 'dart:convert';

class JwtClaims {
  final String? profileType;
  final int? activeBusinessProfileId;
  final String? activeBusinessProfileUuid;

  const JwtClaims({
    this.profileType,
    this.activeBusinessProfileId,
    this.activeBusinessProfileUuid,
  });

  bool get isBusinessProfile =>
      activeBusinessProfileUuid != null && activeBusinessProfileUuid!.isNotEmpty;
}

class JwtDecoder {
  JwtDecoder._();

  static JwtClaims? decode(String token) {
    try {
      final parts = token.split('.');
      if (parts.length != 3) return null;

      final normalized = base64Url.normalize(parts[1]);
      final payload = utf8.decode(base64Url.decode(normalized));
      final data = jsonDecode(payload) as Map<String, dynamic>;

      return JwtClaims(
        profileType: data['profileType'] as String?,
        activeBusinessProfileId: data['activeBusinessProfileId'] as int?,
        activeBusinessProfileUuid: data['activeBusinessProfileUuid'] as String?,
      );
    } catch (_) {
      return null;
    }
  }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `flutter test test/jwt_decoder_test.dart`
Expected: `00:01 +3: All tests passed!`

- [ ] **Step 5: Commit**

```bash
git add lib/utils/jwt_decoder.dart test/jwt_decoder_test.dart
git commit -m "Add JWT payload decoder for professional-mode restore"
```

---

### Task 3: `BusinessProfileAddress` model

**Files:**
- Create: `lib/models/business_profile_address.dart`
- Test: `test/business_profile_address_model_test.dart`

**Interfaces:**
- Consumes: `$bpa.BusinessProfileAddress` from Task 1.
- Produces: `BusinessProfileAddress(id: int?, uuid: String?, businessProfileId: int, addressLine1: String, addressLine2: String?, locality: String, administrativeArea: String, postalCode: String, countryCode: String, latitude: double?, longitude: double?, createdAt: String?, updatedAt: String?)`, `BusinessProfileAddress.fromProto($bpa.BusinessProfileAddress)`, `.toProto() -> $bpa.BusinessProfileAddress`.

- [ ] **Step 1: Write the failing test**

Create `test/business_profile_address_model_test.dart`:

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/models/business_profile_address.dart';
import 'package:lapidation_mobile/src/generated/grpc/business_profile_address.pb.dart' as $bpa;

void main() {
  group('BusinessProfileAddress.fromProto', () {
    test('maps set fields and treats zero/empty as unset for id/uuid', () {
      final proto = $bpa.BusinessProfileAddress(
        id: 9,
        uuid: 'addr-uuid',
        businessProfileId: 3,
        addressLine1: 'Main St 1',
        locality: 'Springfield',
        administrativeArea: 'IL',
        postalCode: '62704',
        countryCode: 'US',
      );

      final model = BusinessProfileAddress.fromProto(proto);

      expect(model.id, 9);
      expect(model.uuid, 'addr-uuid');
      expect(model.businessProfileId, 3);
      expect(model.addressLine1, 'Main St 1');
      expect(model.addressLine2, isNull);
      expect(model.locality, 'Springfield');
    });

    test('new address (never persisted) has null id/uuid', () {
      final proto = $bpa.BusinessProfileAddress(
        businessProfileId: 3,
        addressLine1: 'Main St 1',
        locality: 'Springfield',
        administrativeArea: 'IL',
        postalCode: '62704',
        countryCode: 'US',
      );

      final model = BusinessProfileAddress.fromProto(proto);

      expect(model.id, isNull);
      expect(model.uuid, isNull);
    });
  });

  group('BusinessProfileAddress.toProto', () {
    test('round-trips through protobuf encoding', () {
      final model = BusinessProfileAddress(
        businessProfileId: 3,
        addressLine1: 'Main St 1',
        addressLine2: 'Suite 5',
        locality: 'Springfield',
        administrativeArea: 'IL',
        postalCode: '62704',
        countryCode: 'US',
        latitude: 39.78,
        longitude: -89.65,
      );

      final encoded = model.toProto().writeToBuffer();
      final decoded = $bpa.BusinessProfileAddress.fromBuffer(encoded);

      expect(decoded.addressLine1, 'Main St 1');
      expect(decoded.addressLine2, 'Suite 5');
      expect(decoded.latitude, 39.78);
      expect(decoded.longitude, -89.65);
    });
  });
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `flutter test test/business_profile_address_model_test.dart`
Expected: FAIL — `lib/models/business_profile_address.dart` doesn't exist.

- [ ] **Step 3: Write the implementation**

Create `lib/models/business_profile_address.dart`:

```dart
import '../src/generated/grpc/business_profile_address.pb.dart' as $bpa;

class BusinessProfileAddress {
  final int? id;
  final String? uuid;
  final int businessProfileId;
  final String addressLine1;
  final String? addressLine2;
  final String locality;
  final String administrativeArea;
  final String postalCode;
  final String countryCode;
  final double? latitude;
  final double? longitude;
  final String? createdAt;
  final String? updatedAt;

  const BusinessProfileAddress({
    this.id,
    this.uuid,
    required this.businessProfileId,
    required this.addressLine1,
    this.addressLine2,
    required this.locality,
    required this.administrativeArea,
    required this.postalCode,
    required this.countryCode,
    this.latitude,
    this.longitude,
    this.createdAt,
    this.updatedAt,
  });

  factory BusinessProfileAddress.fromProto($bpa.BusinessProfileAddress proto) {
    return BusinessProfileAddress(
      id: proto.id != 0 ? proto.id : null,
      uuid: proto.uuid.isNotEmpty ? proto.uuid : null,
      businessProfileId: proto.businessProfileId,
      addressLine1: proto.addressLine1,
      addressLine2: proto.addressLine2.isNotEmpty ? proto.addressLine2 : null,
      locality: proto.locality,
      administrativeArea: proto.administrativeArea,
      postalCode: proto.postalCode,
      countryCode: proto.countryCode,
      latitude: proto.latitude != 0 ? proto.latitude : null,
      longitude: proto.longitude != 0 ? proto.longitude : null,
      createdAt: proto.createdAt.isNotEmpty ? proto.createdAt : null,
      updatedAt: proto.updatedAt.isNotEmpty ? proto.updatedAt : null,
    );
  }

  $bpa.BusinessProfileAddress toProto() {
    return $bpa.BusinessProfileAddress(
      id: id,
      uuid: uuid,
      businessProfileId: businessProfileId,
      addressLine1: addressLine1,
      addressLine2: addressLine2,
      locality: locality,
      administrativeArea: administrativeArea,
      postalCode: postalCode,
      countryCode: countryCode,
      latitude: latitude,
      longitude: longitude,
    );
  }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `flutter test test/business_profile_address_model_test.dart`
Expected: `00:01 +3: All tests passed!`

- [ ] **Step 5: Commit**

```bash
git add lib/models/business_profile_address.dart test/business_profile_address_model_test.dart
git commit -m "Add BusinessProfileAddress model with proto mapping"
```

---

### Task 4: `BusinessProfile` model

**Files:**
- Create: `lib/models/business_profile.dart`
- Test: `test/business_profile_model_test.dart`

**Interfaces:**
- Consumes: `$bp.BusinessProfile` (Task 1), `BusinessProfileAddress` (Task 3).
- Produces: `BusinessProfile(id: int?, uuid: String?, ownerId: int, ownerUuid: String, taxId: String, businessName: String, businessType: String, socialName: String?, logo: String?, coverImage: String?, createdAt: String?, updatedAt: String?, addresses: List<BusinessProfileAddress>)`, `BusinessProfile.fromProto($bp.BusinessProfile)`, `.toProto() -> $bp.BusinessProfile`.

- [ ] **Step 1: Write the failing test**

Create `test/business_profile_model_test.dart`:

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/models/business_profile.dart';
import 'package:lapidation_mobile/src/generated/grpc/business_profile.pb.dart' as $bp;
import 'package:lapidation_mobile/src/generated/grpc/business_profile_address.pb.dart' as $bpa;

void main() {
  group('BusinessProfile.fromProto', () {
    test('maps a persisted profile including addresses', () {
      final proto = $bp.BusinessProfile(
        id: 5,
        uuid: 'bp-uuid',
        ownerId: 1,
        ownerUuid: 'owner-uuid',
        taxId: '12-3456789',
        businessName: 'Jane Doe Training',
        businessType: 'Professional',
        socialName: 'Jane Doe',
        addresses: [
          $bpa.BusinessProfileAddress(
            businessProfileId: 5,
            addressLine1: 'Gym Ave 1',
            locality: 'Metropolis',
            administrativeArea: 'NY',
            postalCode: '10001',
            countryCode: 'US',
          ),
        ],
      );

      final model = BusinessProfile.fromProto(proto);

      expect(model.id, 5);
      expect(model.uuid, 'bp-uuid');
      expect(model.businessType, 'Professional');
      expect(model.socialName, 'Jane Doe');
      expect(model.addresses, hasLength(1));
      expect(model.addresses.first.addressLine1, 'Gym Ave 1');
    });

    test('newly created profile (not yet persisted) has null id/uuid', () {
      final proto = $bp.BusinessProfile(
        ownerId: 1,
        ownerUuid: 'owner-uuid',
        taxId: '12-3456789',
        businessName: 'Iron Gym',
        businessType: 'Company',
      );

      final model = BusinessProfile.fromProto(proto);

      expect(model.id, isNull);
      expect(model.uuid, isNull);
      expect(model.socialName, isNull);
    });
  });

  group('BusinessProfile.toProto', () {
    test('round-trips required fields through protobuf encoding', () {
      const model = BusinessProfile(
        ownerId: 1,
        ownerUuid: 'owner-uuid',
        taxId: '12-3456789',
        businessName: 'Jane Doe Training',
        businessType: 'Professional',
        socialName: 'Jane Doe',
      );

      final encoded = model.toProto().writeToBuffer();
      final decoded = $bp.BusinessProfile.fromBuffer(encoded);

      expect(decoded.ownerId, 1);
      expect(decoded.businessName, 'Jane Doe Training');
      expect(decoded.businessType, 'Professional');
    });
  });
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `flutter test test/business_profile_model_test.dart`
Expected: FAIL — `lib/models/business_profile.dart` doesn't exist.

- [ ] **Step 3: Write the implementation**

Create `lib/models/business_profile.dart`:

```dart
import '../src/generated/grpc/business_profile.pb.dart' as $bp;
import 'business_profile_address.dart';

class BusinessProfile {
  final int? id;
  final String? uuid;
  final int ownerId;
  final String ownerUuid;
  final String taxId;
  final String businessName;
  final String businessType;
  final String? socialName;
  final String? logo;
  final String? coverImage;
  final String? createdAt;
  final String? updatedAt;
  final List<BusinessProfileAddress> addresses;

  const BusinessProfile({
    this.id,
    this.uuid,
    required this.ownerId,
    required this.ownerUuid,
    required this.taxId,
    required this.businessName,
    required this.businessType,
    this.socialName,
    this.logo,
    this.coverImage,
    this.createdAt,
    this.updatedAt,
    this.addresses = const [],
  });

  factory BusinessProfile.fromProto($bp.BusinessProfile proto) {
    return BusinessProfile(
      id: proto.id != 0 ? proto.id : null,
      uuid: proto.uuid.isNotEmpty ? proto.uuid : null,
      ownerId: proto.ownerId,
      ownerUuid: proto.ownerUuid,
      taxId: proto.taxId,
      businessName: proto.businessName,
      businessType: proto.businessType,
      socialName: proto.socialName.isNotEmpty ? proto.socialName : null,
      logo: proto.logo.isNotEmpty ? proto.logo : null,
      coverImage: proto.coverImage.isNotEmpty ? proto.coverImage : null,
      createdAt: proto.createdAt.isNotEmpty ? proto.createdAt : null,
      updatedAt: proto.updatedAt.isNotEmpty ? proto.updatedAt : null,
      addresses: proto.addresses.map(BusinessProfileAddress.fromProto).toList(growable: false),
    );
  }

  $bp.BusinessProfile toProto() {
    return $bp.BusinessProfile(
      id: id,
      uuid: uuid,
      ownerId: ownerId,
      ownerUuid: ownerUuid,
      taxId: taxId,
      businessName: businessName,
      businessType: businessType,
      socialName: socialName,
      logo: logo,
      coverImage: coverImage,
    );
  }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `flutter test test/business_profile_model_test.dart`
Expected: `00:01 +3: All tests passed!`

- [ ] **Step 5: Commit**

```bash
git add lib/models/business_profile.dart test/business_profile_model_test.dart
git commit -m "Add BusinessProfile model with proto mapping"
```

---

### Task 5: REST endpoints + presigned-upload methods for business profile images

**Files:**
- Modify: `lib/config/api_config.dart`
- Modify: `lib/services/upload_service.dart`

**Interfaces:**
- Consumes: `PresignedUrlResponse`, `_mediaInfo`, `uploadToS3` (all already exist in `upload_service.dart`).
- Produces: `ApiConfig.businessProfilesEndpoint` (`String`), `UploadService.getBusinessProfileLogoPresignedUrl(token, businessProfileId, format) -> Future<PresignedUrlResponse>`, `getBusinessProfileCoverPresignedUrl(...)`, `uploadBusinessProfileLogo(token, businessProfileId, XFile) -> Future<bool>`, `uploadBusinessProfileCover(...)`.

- [ ] **Step 1: Add the REST endpoint constant**

In `lib/config/api_config.dart`, add next to the existing `peopleMe`/`peopleUploadAvatarEndpoint` constants:

```dart
  static const String businessProfilesEndpoint = '/workout/api/business-profiles';
```

- [ ] **Step 2: Add presigned-URL and upload methods to `UploadService`**

In `lib/services/upload_service.dart`, add after `getCoverPresignedUrl` (before the `_isVideoExtension` helper):

```dart
  static Future<PresignedUrlResponse> getBusinessProfileLogoPresignedUrl(
    String token,
    int businessProfileId,
    String format,
  ) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        '${ApiConfig.businessProfilesEndpoint}/$businessProfileId/upload/avatar?format=$format',
      );
      if (response.statusCode == 200) {
        return PresignedUrlResponse.fromJson(response.data as Map<String, dynamic>);
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to get presigned URL',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to get presigned URL');
    }
  }

  static Future<PresignedUrlResponse> getBusinessProfileCoverPresignedUrl(
    String token,
    int businessProfileId,
    String format,
  ) async {
    try {
      DioClient().setAuthToken(token);
      final response = await _dio.get(
        '${ApiConfig.businessProfilesEndpoint}/$businessProfileId/upload/cover?format=$format',
      );
      if (response.statusCode == 200) {
        return PresignedUrlResponse.fromJson(response.data as Map<String, dynamic>);
      } else {
        throw AppException(
          statusCode: response.statusCode ?? 500,
          message: response.data?['message'] ?? 'Failed to get presigned URL',
        );
      }
    } on DioException catch (e) {
      throw BaseService.handleDioError(e, 'Failed to get presigned URL');
    }
  }
```

And after the existing `uploadCover` method, at the end of the class (before the closing `}`):

```dart

  static Future<bool> uploadBusinessProfileLogo(
    String token,
    int businessProfileId,
    XFile imageFile,
  ) async {
    final (mediaType, format, contentType) = _mediaInfo(
      imageFile.name.isNotEmpty ? imageFile.name : imageFile.path,
    );
    final presignedResponse = await getBusinessProfileLogoPresignedUrl(token, businessProfileId, contentType);
    await uploadToS3(presignedResponse.url, imageFile, format);
    return true;
  }

  static Future<bool> uploadBusinessProfileCover(
    String token,
    int businessProfileId,
    XFile imageFile,
  ) async {
    final (mediaType, format, contentType) = _mediaInfo(
      imageFile.name.isNotEmpty ? imageFile.name : imageFile.path,
    );
    final presignedResponse = await getBusinessProfileCoverPresignedUrl(token, businessProfileId, contentType);
    await uploadToS3(presignedResponse.url, imageFile, format);
    return true;
  }
```

(This mirrors `uploadAvatar`/`uploadCover` exactly, including passing `contentType` as the `format` query parameter — that's the existing convention for the `format` query param on this endpoint family, not a new decision.)

- [ ] **Step 3: Verify it compiles**

```bash
dart analyze lib/config/api_config.dart lib/services/upload_service.dart
```

Expected: `No issues found!`

- [ ] **Step 4: Commit**

```bash
git add lib/config/api_config.dart lib/services/upload_service.dart
git commit -m "Add business profile presigned-upload REST endpoints"
```

---

### Task 6: `GrpcBusinessProfileService` façade

**Files:**
- Create: `lib/services/grpc/grpc_business_profile_service.dart`
- Test: `test/grpc_business_profile_service_test.dart`

**Interfaces:**
- Consumes: `BusinessProfile`/`BusinessProfileAddress` models (Tasks 3-4), `GrpcChannelFactory` (existing), `ApiConfig` (existing).
- Produces: `GrpcBusinessProfileService.getBusinessProfileById({int? id, String? uuid}) -> Future<BusinessProfile>`, `.addBusinessProfile(BusinessProfile) -> Future<BusinessProfile>`, `.updateBusinessProfile(BusinessProfile) -> Future<BusinessProfile>`, `.addBusinessProfileAddress(BusinessProfileAddress) -> Future<BusinessProfileAddress>`, `.updateBusinessProfileAddress(BusinessProfileAddress) -> Future<BusinessProfileAddress>`, `.removeBusinessProfileAddress({int? id, String? uuid}) -> Future<bool>`, `.shutdown() -> Future<void>`.

- [ ] **Step 1: Write the failing test**

Create `test/grpc_business_profile_service_test.dart` (following the `grpc_person_service_test.dart` precedent of round-tripping proto messages rather than mocking the network client):

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/src/generated/grpc/business_profile.pb.dart' as $bp;

void main() {
  group('BusinessProfileRequestId', () {
    test('round-trips id and uuid through protobuf encoding', () {
      final request = $bp.BusinessProfileRequestId(id: 5, uuid: 'bp-uuid');

      final encoded = request.writeToBuffer();
      final decoded = $bp.BusinessProfileRequestId.fromBuffer(encoded);

      expect(decoded.id, 5);
      expect(decoded.uuid, 'bp-uuid');
    });
  });

  group('RemoveBusinessProfileAddressRequest', () {
    test('round-trips id and uuid through protobuf encoding', () {
      final request = $bp.RemoveBusinessProfileAddressRequest(id: 9, uuid: 'addr-uuid');

      final encoded = request.writeToBuffer();
      final decoded = $bp.RemoveBusinessProfileAddressRequest.fromBuffer(encoded);

      expect(decoded.id, 9);
      expect(decoded.uuid, 'addr-uuid');
    });
  });
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `flutter test test/grpc_business_profile_service_test.dart`
Expected: FAIL — these message types don't yet compile into the test target until Task 1's stubs exist; since Task 1 already ran, this instead validates the proto shapes ahead of writing the façade. If Task 1 is complete this test should actually PASS already — run it now as a pre-check before writing the façade, then proceed.

- [ ] **Step 3: Write the implementation**

Create `lib/services/grpc/grpc_business_profile_service.dart`:

```dart
import 'package:grpc/grpc.dart' as grpc;

import '../../config/api_config.dart';
import '../../models/business_profile.dart';
import '../../models/business_profile_address.dart';
import '../../src/generated/grpc/business_profile.pbgrpc.dart' as $bp;
import 'grpc_channel_factory.dart';

class GrpcBusinessProfileService {
  GrpcBusinessProfileService._();

  static $bp.BusinessProfileServiceClient? _client;

  static Future<BusinessProfile> getBusinessProfileById({int? id, String? uuid}) async {
    final response = await _ensureClient().getBusinessProfileById(
      $bp.BusinessProfileRequestId(id: id ?? 0, uuid: uuid ?? ''),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return BusinessProfile.fromProto(response);
  }

  static Future<BusinessProfile> addBusinessProfile(BusinessProfile profile) async {
    final response = await _ensureClient().addBusinessProfile(
      profile.toProto(),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return BusinessProfile.fromProto(response);
  }

  static Future<BusinessProfile> updateBusinessProfile(BusinessProfile profile) async {
    final response = await _ensureClient().updateBusinessProfile(
      profile.toProto(),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return BusinessProfile.fromProto(response);
  }

  static Future<BusinessProfileAddress> addBusinessProfileAddress(BusinessProfileAddress address) async {
    final response = await _ensureClient().addBusinessProfileAddress(
      address.toProto(),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return BusinessProfileAddress.fromProto(response);
  }

  static Future<BusinessProfileAddress> updateBusinessProfileAddress(BusinessProfileAddress address) async {
    final response = await _ensureClient().updateBusinessProfileAddress(
      address.toProto(),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return BusinessProfileAddress.fromProto(response);
  }

  static Future<bool> removeBusinessProfileAddress({int? id, String? uuid}) async {
    final response = await _ensureClient().removeBusinessProfileAddress(
      $bp.RemoveBusinessProfileAddressRequest(id: id ?? 0, uuid: uuid ?? ''),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return response.success;
  }

  static $bp.BusinessProfileServiceClient _ensureClient() {
    if (_client != null) return _client!;
    final channel = GrpcChannelFactory.channelFor(
      host: ApiConfig.grpcHost,
      port: ApiConfig.grpcPort,
      authority: ApiConfig.grpcAuthority,
    );
    _client = $bp.BusinessProfileServiceClient(channel, interceptors: GrpcChannelFactory.interceptors);
    return _client!;
  }

  static Future<void> shutdown() async {
    _client = null;
  }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `flutter test test/grpc_business_profile_service_test.dart`
Expected: `00:01 +2: All tests passed!`

- [ ] **Step 5: Verify full analyze**

```bash
dart analyze lib/services/grpc/grpc_business_profile_service.dart
```

Expected: `No issues found!`

- [ ] **Step 6: Commit**

```bash
git add lib/services/grpc/grpc_business_profile_service.dart test/grpc_business_profile_service_test.dart
git commit -m "Add GrpcBusinessProfileService client facade"
```

---

### Task 7: `BusinessProfileProvider`

**Files:**
- Create: `lib/providers/business_profile_provider.dart`

**Interfaces:**
- Consumes: `GrpcBusinessProfileService` (Task 6), `UploadService` (Task 5), `BusinessProfile`/`BusinessProfileAddress` models.
- Produces: `BusinessProfileProvider extends ChangeNotifier` with `current` (`BusinessProfile?`), `loading`/`updating` (`bool`), `error` (`String?`) getters; `load({int? id, String? uuid}) -> Future<bool>`; `update(BusinessProfile) -> Future<bool>`; `addAddress(BusinessProfileAddress) -> Future<bool>`; `updateAddress(BusinessProfileAddress) -> Future<bool>`; `removeAddress({int? id, String? uuid}) -> Future<bool>`; `uploadLogo(String token, XFile) -> Future<bool>`; `uploadCover(String token, XFile) -> Future<bool>`; `clear() -> void`.

- [ ] **Step 1: Write the implementation**

Create `lib/providers/business_profile_provider.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';

import '../models/business_profile.dart';
import '../models/business_profile_address.dart';
import '../services/grpc/grpc_business_profile_service.dart';
import '../services/upload_service.dart';

class BusinessProfileProvider extends ChangeNotifier {
  BusinessProfile? _current;
  bool _loading = false;
  bool _updating = false;
  String? _error;

  BusinessProfile? get current => _current;
  bool get loading => _loading;
  bool get updating => _updating;
  String? get error => _error;

  Future<bool> load({int? id, String? uuid}) async {
    _loading = true;
    _error = null;
    notifyListeners();
    try {
      _current = await GrpcBusinessProfileService.getBusinessProfileById(id: id, uuid: uuid);
      _loading = false;
      notifyListeners();
      return true;
    } catch (_) {
      _error = 'Failed to load business profile.';
      _loading = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> update(BusinessProfile profile) async {
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      _current = await GrpcBusinessProfileService.updateBusinessProfile(profile);
      _updating = false;
      notifyListeners();
      return true;
    } catch (_) {
      _error = 'Failed to update business profile.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> addAddress(BusinessProfileAddress address) async {
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      await GrpcBusinessProfileService.addBusinessProfileAddress(address);
      await load(id: _current?.id, uuid: _current?.uuid);
      _updating = false;
      notifyListeners();
      return true;
    } catch (_) {
      _error = 'Failed to add address.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> updateAddress(BusinessProfileAddress address) async {
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      await GrpcBusinessProfileService.updateBusinessProfileAddress(address);
      await load(id: _current?.id, uuid: _current?.uuid);
      _updating = false;
      notifyListeners();
      return true;
    } catch (_) {
      _error = 'Failed to update address.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> removeAddress({int? id, String? uuid}) async {
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      await GrpcBusinessProfileService.removeBusinessProfileAddress(id: id, uuid: uuid);
      await load(id: _current?.id, uuid: _current?.uuid);
      _updating = false;
      notifyListeners();
      return true;
    } catch (_) {
      _error = 'Failed to remove address.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> uploadLogo(String token, XFile file) async {
    final businessProfileId = _current?.id;
    if (businessProfileId == null) {
      _error = 'Business profile not loaded.';
      notifyListeners();
      return false;
    }
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      await UploadService.uploadBusinessProfileLogo(token, businessProfileId, file);
      await load(id: businessProfileId);
      _updating = false;
      notifyListeners();
      return true;
    } catch (_) {
      _error = 'Failed to upload logo.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> uploadCover(String token, XFile file) async {
    final businessProfileId = _current?.id;
    if (businessProfileId == null) {
      _error = 'Business profile not loaded.';
      notifyListeners();
      return false;
    }
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      await UploadService.uploadBusinessProfileCover(token, businessProfileId, file);
      await load(id: businessProfileId);
      _updating = false;
      notifyListeners();
      return true;
    } catch (_) {
      _error = 'Failed to upload cover.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  void clear() {
    _current = null;
    _error = null;
    notifyListeners();
  }
}
```

- [ ] **Step 2: Verify it compiles**

```bash
dart analyze lib/providers/business_profile_provider.dart
```

Expected: `No issues found!`

- [ ] **Step 3: Commit**

```bash
git add lib/providers/business_profile_provider.dart
git commit -m "Add BusinessProfileProvider"
```

---

### Task 8: `PersonProvider` — active-profile state, real `switchProfile`, persistence, logout clearing

**Files:**
- Modify: `lib/providers/person_provider.dart`
- Test: `test/person_provider_active_profile_test.dart`

**Interfaces:**
- Consumes: `Profile` (existing model), `JwtDecoder` (Task 2).
- Produces: `PersonProvider.activeProfile -> Profile?`, `.isProfessional -> bool`, `.switchProfile(int index, String token) -> Future<bool>` (now fully implemented), `.switchToPersonal() -> Future<bool>`, `.restoreActiveProfileFromToken(String token) -> Future<void>`, `.clear()` (now also clears active profile).

- [ ] **Step 1: Write the failing test**

Create `test/person_provider_active_profile_test.dart`:

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:lapidation_mobile/models/person.dart';
import 'package:lapidation_mobile/models/profile.dart';
import 'package:lapidation_mobile/providers/person_provider.dart';

Person _personWithProfiles() {
  return Person(
    id: 1,
    uuid: 'person-uuid',
    firstname: 'Jane',
    surname: 'Doe',
    profiles: [
      Profile(
        id: 1,
        uuid: 'profile-uuid-1',
        personName: 'Jane Doe',
        personId: 1,
        personUuid: 'person-uuid',
        type: 'Professional',
        profileId: 10,
        profileUuid: 'bp-uuid-1',
      ),
    ],
  );
}

void main() {
  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  group('PersonProvider active profile', () {
    test('starts in personal mode', () async {
      final provider = PersonProvider();
      await Future<void>.delayed(Duration.zero);

      expect(provider.isProfessional, isFalse);
      expect(provider.activeProfile, isNull);
    });

    test('switchProfile sets isProfessional and persists the active profile', () async {
      final provider = PersonProvider();
      await Future<void>.delayed(Duration.zero);
      provider.setPersonForTest(_personWithProfiles());

      final success = await provider.switchProfile(0, 'token');

      expect(success, isTrue);
      expect(provider.isProfessional, isTrue);
      expect(provider.activeProfile?.profileUuid, 'bp-uuid-1');

      final prefs = await SharedPreferences.getInstance();
      expect(prefs.getString('active_profile'), isNotNull);
    });

    test('switchToPersonal clears the active profile and its storage', () async {
      final provider = PersonProvider();
      await Future<void>.delayed(Duration.zero);
      provider.setPersonForTest(_personWithProfiles());
      await provider.switchProfile(0, 'token');

      final success = await provider.switchToPersonal();

      expect(success, isTrue);
      expect(provider.isProfessional, isFalse);
      final prefs = await SharedPreferences.getInstance();
      expect(prefs.getString('active_profile'), isNull);
    });

    test('clear() resets both person and active profile', () async {
      final provider = PersonProvider();
      await Future<void>.delayed(Duration.zero);
      provider.setPersonForTest(_personWithProfiles());
      await provider.switchProfile(0, 'token');

      provider.clear();

      expect(provider.person, isNull);
      expect(provider.activeProfile, isNull);
    });
  });
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `flutter test test/person_provider_active_profile_test.dart`
Expected: FAIL — `setPersonForTest`, `activeProfile`, `isProfessional`, `switchToPersonal` don't exist yet.

- [ ] **Step 3: Modify `lib/providers/person_provider.dart`**

Add a `@visibleForTesting` seam and the new state. Replace the existing field/getter block at the top of the class (the block containing `Person? _person;`, `bool _loading = false;`, etc.) by adding these two lines right after the existing `String? _error;` field:

```dart
  Profile? _activeProfile;

  static const String _activeProfileStorageKey = 'active_profile';
```

Add these getters immediately after the existing `String? get error => _error;` getter:

```dart
  Profile? get activeProfile => _activeProfile;
  bool get isProfessional => _activeProfile != null;
```

Add a test-only setter (mirrors how the class already exposes state for its own internal use — needed because `Person` has no public setter and the constructor triggers async storage loading that a unit test can't easily race against otherwise). Add it right after the getters:

```dart
  @visibleForTesting
  void setPersonForTest(Person person) {
    _person = person;
    notifyListeners();
  }
```

Add the import for `visibleForTesting` at the top of the file (with the other imports):

```dart
import 'package:flutter/foundation.dart';
```

In `_loadFromStorage()`, after the existing block that restores `_person` from the `'person'` key (right before the trailing `_initialized`/`notifyListeners()`-equivalent lines that end that method — for `PersonProvider` this is right before its final `notifyListeners();`), add:

```dart
    final activeProfileJson = prefs.getString(_activeProfileStorageKey);
    if (activeProfileJson != null) {
      try {
        _activeProfile = Profile.fromJson(jsonDecode(activeProfileJson) as Map<String, dynamic>);
      } catch (_) {
        await prefs.remove(_activeProfileStorageKey);
      }
    }
```

Add a private persistence helper near the existing `_saveToStorage`/`_clearStorage` methods:

```dart
  Future<void> _saveActiveProfileToStorage(Profile? profile) async {
    final prefs = await SharedPreferences.getInstance();
    if (profile == null) {
      await prefs.remove(_activeProfileStorageKey);
    } else {
      await prefs.setString(_activeProfileStorageKey, jsonEncode(profile.toJson()));
    }
  }
```

Replace the existing TODO `switchProfile` stub entirely:

```dart
  Future<bool> switchProfile(int profileIndex, String token) async {
    if (_person == null || profileIndex < 0 || profileIndex >= _person!.profiles.length) {
      _error = 'Invalid profile index';
      notifyListeners();
      return false;
    }
    _updating = true;
    _error = null;
    notifyListeners();
    try {
      _activeProfile = _person!.profiles[profileIndex];
      await _saveActiveProfileToStorage(_activeProfile);
      _updating = false;
      notifyListeners();
      return true;
    } catch (_) {
      _error = 'Failed to switch profile.';
      _updating = false;
      notifyListeners();
      return false;
    }
  }

  Future<bool> switchToPersonal() async {
    _updating = true;
    notifyListeners();
    _activeProfile = null;
    await _saveActiveProfileToStorage(null);
    _updating = false;
    notifyListeners();
    return true;
  }

  Future<void> restoreActiveProfileFromToken(String token) async {
    final claims = JwtDecoder.decode(token);
    if (claims == null || !claims.isBusinessProfile || _person == null) {
      return;
    }
    final index = _person!.profiles.indexWhere(
      (p) => p.profileUuid == claims.activeBusinessProfileUuid,
    );
    if (index == -1) return;
    _activeProfile = _person!.profiles[index];
    await _saveActiveProfileToStorage(_activeProfile);
    notifyListeners();
  }
```

Add the import for `JwtDecoder` at the top of the file:

```dart
import '../utils/jwt_decoder.dart';
```

Finally, modify the existing `clear()` method to also reset the active profile. Its current body sets `_person = null`, `_error = null`, calls `_clearStorage()`, and calls `notifyListeners()`. Replace it with:

```dart
  void clear() {
    _person = null;
    _activeProfile = null;
    _error = null;
    _clearStorage();
    _saveActiveProfileToStorage(null);
    notifyListeners();
  }
```

- [ ] **Step 4: Run test to verify it passes**

Run: `flutter test test/person_provider_active_profile_test.dart`
Expected: `00:01 +4: All tests passed!`

- [ ] **Step 5: Verify no regressions**

```bash
dart analyze lib/providers/person_provider.dart && flutter test
```

Expected: `No issues found!` and all existing tests still pass.

- [ ] **Step 6: Commit**

```bash
git add lib/providers/person_provider.dart test/person_provider_active_profile_test.dart
git commit -m "Implement real switchProfile and local professional-mode state in PersonProvider"
```

---

### Task 9: Wire JWT-based professional-mode restore into sign-in

**Files:**
- Modify: `lib/pages/sign_in/sign_in_page.dart`

**Interfaces:**
- Consumes: `PersonProvider.restoreActiveProfileFromToken(String token)` (Task 8).

- [ ] **Step 1: Call the restore hook after `fetchMe` in `_handleAuthCheck`**

In `lib/pages/sign_in/sign_in_page.dart`, immediately after the line `await personProvider.fetchMe(token);` inside `_handleAuthCheck()` (around line 65), add:

```dart
      await personProvider.restoreActiveProfileFromToken(token);
```

- [ ] **Step 2: Call the restore hook after `fetchMe` in `_handleSignIn`**

Immediately after the line `await personProvider.fetchMe(token);` inside `_handleSignIn()` (around line 125), add the same call:

```dart
      await personProvider.restoreActiveProfileFromToken(token);
```

- [ ] **Step 3: Verify it compiles**

```bash
dart analyze lib/pages/sign_in/sign_in_page.dart
```

Expected: `No issues found!`

- [ ] **Step 4: Commit**

```bash
git add lib/pages/sign_in/sign_in_page.dart
git commit -m "Restore professional mode from JWT claims after sign-in"
```

---

### Task 10: Add l10n strings

**Files:**
- Modify: `lib/l10n/app_en.arb`, `lib/l10n/app_es.arb`, `lib/l10n/app_fr.arb`, `lib/l10n/app_nl.arb`, `lib/l10n/app_pt.arb`, `lib/l10n/app_pt_BR.arb`

**Interfaces:**
- Produces: `AppLocalizations` getters `addProfileTitle`, `addProfileSubtitle`, `addProfilePersonalTrainerTitle`, `addProfilePersonalTrainerDescription`, `addProfileGymTitle`, `addProfileGymDescription`, `businessProfileFormTitlePersonalTrainer`, `businessProfileFormTitleGym`, `businessProfileFormBusinessName`, `businessProfileFormBusinessNameRequired`, `businessProfileFormSocialName`, `businessProfileFormSocialNameRequired`, `businessProfileFormTaxId`, `businessProfileFormTaxIdRequired`, `businessProfileFormSubmit`, `businessProfileFormError`, `businessProfilePageEdit`, `businessProfilePageSave`, `businessProfilePageAddresses`, `businessProfileSwitchToPersonal`.

- [ ] **Step 1: Add the keys to `app_en.arb`**

In `lib/l10n/app_en.arb`, add before the final closing `}`:

```json
  ,
  "addProfileTitle": "Add a new profile",
  "addProfileSubtitle": "Choose the type of business profile you want to create",
  "addProfilePersonalTrainerTitle": "Personal Trainer",
  "addProfilePersonalTrainerDescription": "Offer personal training services and manage your own clients",
  "addProfileGymTitle": "Gym",
  "addProfileGymDescription": "Manage a gym or fitness facility and its members",
  "businessProfileFormTitlePersonalTrainer": "Create your Personal Trainer profile",
  "businessProfileFormTitleGym": "Create your Gym profile",
  "businessProfileFormBusinessName": "Business name",
  "businessProfileFormBusinessNameRequired": "Please enter a business name",
  "businessProfileFormSocialName": "Social name",
  "businessProfileFormSocialNameRequired": "Please enter a social name",
  "businessProfileFormTaxId": "Tax ID",
  "businessProfileFormTaxIdRequired": "Please enter a tax ID",
  "businessProfileFormSubmit": "Create profile",
  "businessProfileFormError": "Failed to create business profile. Please try again.",
  "businessProfilePageEdit": "Edit",
  "businessProfilePageSave": "Save",
  "businessProfilePageAddresses": "Addresses",
  "businessProfileSwitchToPersonal": "Personal account"
```

- [ ] **Step 2: Add the Spanish translations to `app_es.arb`**

```json
  ,
  "addProfileTitle": "Agregar un nuevo perfil",
  "addProfileSubtitle": "Elige el tipo de perfil de negocio que quieres crear",
  "addProfilePersonalTrainerTitle": "Entrenador Personal",
  "addProfilePersonalTrainerDescription": "Ofrece servicios de entrenamiento personal y gestiona tus propios clientes",
  "addProfileGymTitle": "Gimnasio",
  "addProfileGymDescription": "Gestiona un gimnasio o instalación deportiva y sus miembros",
  "businessProfileFormTitlePersonalTrainer": "Crea tu perfil de Entrenador Personal",
  "businessProfileFormTitleGym": "Crea tu perfil de Gimnasio",
  "businessProfileFormBusinessName": "Nombre del negocio",
  "businessProfileFormBusinessNameRequired": "Ingresa un nombre de negocio",
  "businessProfileFormSocialName": "Razón social",
  "businessProfileFormSocialNameRequired": "Ingresa una razón social",
  "businessProfileFormTaxId": "NIF/CIF",
  "businessProfileFormTaxIdRequired": "Ingresa un NIF/CIF",
  "businessProfileFormSubmit": "Crear perfil",
  "businessProfileFormError": "No se pudo crear el perfil de negocio. Inténtalo de nuevo.",
  "businessProfilePageEdit": "Editar",
  "businessProfilePageSave": "Guardar",
  "businessProfilePageAddresses": "Direcciones",
  "businessProfileSwitchToPersonal": "Cuenta personal"
```

- [ ] **Step 3: Add the French translations to `app_fr.arb`**

```json
  ,
  "addProfileTitle": "Ajouter un nouveau profil",
  "addProfileSubtitle": "Choisissez le type de profil professionnel à créer",
  "addProfilePersonalTrainerTitle": "Coach Personnel",
  "addProfilePersonalTrainerDescription": "Proposez des services de coaching personnel et gérez vos propres clients",
  "addProfileGymTitle": "Salle de sport",
  "addProfileGymDescription": "Gérez une salle de sport et ses membres",
  "businessProfileFormTitlePersonalTrainer": "Créez votre profil de Coach Personnel",
  "businessProfileFormTitleGym": "Créez votre profil de Salle de sport",
  "businessProfileFormBusinessName": "Nom de l'entreprise",
  "businessProfileFormBusinessNameRequired": "Veuillez saisir un nom d'entreprise",
  "businessProfileFormSocialName": "Raison sociale",
  "businessProfileFormSocialNameRequired": "Veuillez saisir une raison sociale",
  "businessProfileFormTaxId": "Numéro fiscal",
  "businessProfileFormTaxIdRequired": "Veuillez saisir un numéro fiscal",
  "businessProfileFormSubmit": "Créer le profil",
  "businessProfileFormError": "Échec de la création du profil professionnel. Veuillez réessayer.",
  "businessProfilePageEdit": "Modifier",
  "businessProfilePageSave": "Enregistrer",
  "businessProfilePageAddresses": "Adresses",
  "businessProfileSwitchToPersonal": "Compte personnel"
```

- [ ] **Step 4: Add the Dutch translations to `app_nl.arb`**

```json
  ,
  "addProfileTitle": "Nieuw profiel toevoegen",
  "addProfileSubtitle": "Kies het type zakelijk profiel dat je wilt aanmaken",
  "addProfilePersonalTrainerTitle": "Personal Trainer",
  "addProfilePersonalTrainerDescription": "Bied personal training aan en beheer je eigen klanten",
  "addProfileGymTitle": "Sportschool",
  "addProfileGymDescription": "Beheer een sportschool of fitnessfaciliteit en haar leden",
  "businessProfileFormTitlePersonalTrainer": "Maak je Personal Trainer-profiel aan",
  "businessProfileFormTitleGym": "Maak je Sportschool-profiel aan",
  "businessProfileFormBusinessName": "Bedrijfsnaam",
  "businessProfileFormBusinessNameRequired": "Voer een bedrijfsnaam in",
  "businessProfileFormSocialName": "Handelsnaam",
  "businessProfileFormSocialNameRequired": "Voer een handelsnaam in",
  "businessProfileFormTaxId": "Btw-nummer",
  "businessProfileFormTaxIdRequired": "Voer een btw-nummer in",
  "businessProfileFormSubmit": "Profiel aanmaken",
  "businessProfileFormError": "Zakelijk profiel aanmaken mislukt. Probeer het opnieuw.",
  "businessProfilePageEdit": "Bewerken",
  "businessProfilePageSave": "Opslaan",
  "businessProfilePageAddresses": "Adressen",
  "businessProfileSwitchToPersonal": "Persoonlijk account"
```

- [ ] **Step 5: Add the Portuguese translations to `app_pt.arb`**

```json
  ,
  "addProfileTitle": "Adicionar novo perfil",
  "addProfileSubtitle": "Escolha o tipo de perfil de negócio que deseja criar",
  "addProfilePersonalTrainerTitle": "Personal Trainer",
  "addProfilePersonalTrainerDescription": "Ofereça serviços de personal training e gerencie os seus próprios clientes",
  "addProfileGymTitle": "Ginásio",
  "addProfileGymDescription": "Gerencie um ginásio ou instalação desportiva e os seus membros",
  "businessProfileFormTitlePersonalTrainer": "Crie o seu perfil de Personal Trainer",
  "businessProfileFormTitleGym": "Crie o seu perfil de Ginásio",
  "businessProfileFormBusinessName": "Nome do negócio",
  "businessProfileFormBusinessNameRequired": "Insira um nome de negócio",
  "businessProfileFormSocialName": "Razão social",
  "businessProfileFormSocialNameRequired": "Insira uma razão social",
  "businessProfileFormTaxId": "NIF",
  "businessProfileFormTaxIdRequired": "Insira um NIF",
  "businessProfileFormSubmit": "Criar perfil",
  "businessProfileFormError": "Falha ao criar o perfil de negócio. Tente novamente.",
  "businessProfilePageEdit": "Editar",
  "businessProfilePageSave": "Guardar",
  "businessProfilePageAddresses": "Moradas",
  "businessProfileSwitchToPersonal": "Conta pessoal"
```

- [ ] **Step 6: Add the Brazilian Portuguese translations to `app_pt_BR.arb`**

```json
  ,
  "addProfileTitle": "Adicionar novo perfil",
  "addProfileSubtitle": "Escolha o tipo de perfil de negócio que deseja criar",
  "addProfilePersonalTrainerTitle": "Personal Trainer",
  "addProfilePersonalTrainerDescription": "Ofereça serviços de personal training e gerencie seus próprios clientes",
  "addProfileGymTitle": "Academia",
  "addProfileGymDescription": "Gerencie uma academia ou instalação esportiva e seus membros",
  "businessProfileFormTitlePersonalTrainer": "Crie seu perfil de Personal Trainer",
  "businessProfileFormTitleGym": "Crie seu perfil de Academia",
  "businessProfileFormBusinessName": "Nome do negócio",
  "businessProfileFormBusinessNameRequired": "Digite um nome de negócio",
  "businessProfileFormSocialName": "Razão social",
  "businessProfileFormSocialNameRequired": "Digite uma razão social",
  "businessProfileFormTaxId": "CNPJ",
  "businessProfileFormTaxIdRequired": "Digite um CNPJ",
  "businessProfileFormSubmit": "Criar perfil",
  "businessProfileFormError": "Falha ao criar o perfil de negócio. Tente novamente.",
  "businessProfilePageEdit": "Editar",
  "businessProfilePageSave": "Salvar",
  "businessProfilePageAddresses": "Endereços",
  "businessProfileSwitchToPersonal": "Conta pessoal"
```

- [ ] **Step 7: Regenerate localizations and verify**

```bash
flutter gen-l10n
dart analyze lib/l10n/app_localizations.dart
```

Expected: `No issues found!` and `lib/l10n/app_localizations_en.dart` (etc.) contain the new getters.

- [ ] **Step 8: Commit**

```bash
git add lib/l10n/*.arb
git commit -m "Add business profile creation and professional-mode l10n strings"
```

---

### Task 11: `AddProfilePage` — type-selection cards

**Files:**
- Create: `lib/pages/business_profile/add_profile_page.dart`
- Test: `test/add_profile_page_test.dart`

**Interfaces:**
- Consumes: `MainLayout`, `NavSection.home` (existing), l10n keys from Task 10.
- Produces: `AddProfilePage extends StatelessWidget`; `businessProfileTypeOptions -> List<BusinessProfileTypeOption>` (const seam for a future backend-driven catalog); tapping a card calls `Navigator.of(context).pushNamed('/add-profile/form', arguments: <businessType String>)`.

- [ ] **Step 1: Write the implementation**

Create `lib/pages/business_profile/add_profile_page.dart`:

```dart
import 'package:flutter/material.dart';

import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../widgets/main_layout.dart';

class BusinessProfileTypeOption {
  final String businessType;
  final String Function(AppLocalizations) title;
  final String Function(AppLocalizations) description;
  final String imageAsset;

  const BusinessProfileTypeOption({
    required this.businessType,
    required this.title,
    required this.description,
    required this.imageAsset,
  });
}

const List<BusinessProfileTypeOption> businessProfileTypeOptions = [
  BusinessProfileTypeOption(
    businessType: 'Professional',
    title: _personalTrainerTitle,
    description: _personalTrainerDescription,
    imageAsset: 'assets/images/avatar_male.png',
  ),
  BusinessProfileTypeOption(
    businessType: 'Company',
    title: _gymTitle,
    description: _gymDescription,
    imageAsset: 'assets/images/cover_foto.png',
  ),
];

String _personalTrainerTitle(AppLocalizations l10n) => l10n.addProfilePersonalTrainerTitle;
String _personalTrainerDescription(AppLocalizations l10n) => l10n.addProfilePersonalTrainerDescription;
String _gymTitle(AppLocalizations l10n) => l10n.addProfileGymTitle;
String _gymDescription(AppLocalizations l10n) => l10n.addProfileGymDescription;

class AddProfilePage extends StatelessWidget {
  const AddProfilePage({super.key});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/add-profile',
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(l10n.addProfileTitle, style: const TextStyle(fontSize: 22, fontWeight: FontWeight.bold)),
            const SizedBox(height: 8),
            Text(l10n.addProfileSubtitle, style: const TextStyle(fontSize: 14, color: Colors.grey)),
            const SizedBox(height: 24),
            Wrap(
              spacing: 16,
              runSpacing: 16,
              children: businessProfileTypeOptions
                  .map((option) => _BusinessProfileTypeCard(option: option))
                  .toList(),
            ),
          ],
        ),
      ),
    );
  }
}

class _BusinessProfileTypeCard extends StatelessWidget {
  final BusinessProfileTypeOption option;

  const _BusinessProfileTypeCard({required this.option});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return SizedBox(
      width: 260,
      child: Card(
        elevation: 2,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        clipBehavior: Clip.antiAlias,
        child: InkWell(
          key: ValueKey('business_profile_type_card_${option.businessType}'),
          onTap: () => Navigator.of(context).pushNamed(
            '/add-profile/form',
            arguments: option.businessType,
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Image.asset(option.imageAsset, height: 140, width: double.infinity, fit: BoxFit.cover),
              Padding(
                padding: const EdgeInsets.all(16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(option.title(l10n), style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w600)),
                    const SizedBox(height: 6),
                    Text(option.description(l10n), style: const TextStyle(fontSize: 13, color: Colors.grey)),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
```

- [ ] **Step 2: Write the widget test**

Create `test/add_profile_page_test.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:lapidation_mobile/l10n/app_localizations.dart';
import 'package:lapidation_mobile/pages/business_profile/add_profile_page.dart';
import 'package:lapidation_mobile/providers/auth_provider.dart';
import 'package:lapidation_mobile/providers/person_provider.dart';
import 'package:lapidation_mobile/providers/settings_provider.dart';
import 'package:shared_preferences/shared_preferences.dart';

class _RecordingObserver extends NavigatorObserver {
  final List<Route<dynamic>> pushed = [];

  @override
  void didPush(Route<dynamic> route, Route<dynamic>? previousRoute) {
    pushed.add(route);
  }
}

void main() {
  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  testWidgets('renders both Personal Trainer and Gym cards and navigates with the right businessType', (tester) async {
    final observer = _RecordingObserver();

    await tester.pumpWidget(
      MultiProvider(
        providers: [
          ChangeNotifierProvider(create: (_) => AuthProvider()),
          ChangeNotifierProvider(create: (_) => PersonProvider()),
          ChangeNotifierProvider(create: (_) => SettingsProvider()),
        ],
        child: MaterialApp(
          localizationsDelegates: AppLocalizations.localizationsDelegates,
          supportedLocales: AppLocalizations.supportedLocales,
          navigatorObservers: [observer],
          home: const AddProfilePage(),
          routes: {
            '/add-profile/form': (context) => const Scaffold(body: Text('form')),
          },
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Personal Trainer'), findsOneWidget);
    expect(find.text('Gym'), findsOneWidget);

    await tester.tap(find.byKey(const ValueKey('business_profile_type_card_Professional')));
    await tester.pumpAndSettle();

    final pushedRoute = observer.pushed.last;
    expect(pushedRoute.settings.name, '/add-profile/form');
    expect(pushedRoute.settings.arguments, 'Professional');
  });
}
```

- [ ] **Step 3: Run test to verify it passes**

Run: `flutter test test/add_profile_page_test.dart`
Expected: `00:0X +1: All tests passed!` (this test depends on Task 14's `main.dart` route registrations only for the real app; here the test app registers its own stub route, so it passes standalone once `AddProfilePage` and `MainLayout` compile).

- [ ] **Step 4: Commit**

```bash
git add lib/pages/business_profile/add_profile_page.dart test/add_profile_page_test.dart
git commit -m "Add AddProfilePage with Personal Trainer and Gym type cards"
```

---

### Task 12: `BusinessProfileSignUpPage` — creation form

**Files:**
- Create: `lib/pages/business_profile/business_profile_sign_up_page.dart`
- Test: `test/business_profile_sign_up_page_test.dart`

**Interfaces:**
- Consumes: `AuthProvider`, `PersonProvider` (with Task 8's `switchProfile`), `BusinessProfileProvider` (Task 7), `GrpcBusinessProfileService.addBusinessProfile` (Task 6), `BusinessProfile` model (Task 4), l10n keys (Task 10).
- Produces: `BusinessProfileSignUpPage extends StatefulWidget`, reads `businessType` from `ModalRoute.of(context)!.settings.arguments as String`, on success navigates to `/feed` via `pushNamedAndRemoveUntil`.

- [ ] **Step 1: Write the implementation**

Create `lib/pages/business_profile/business_profile_sign_up_page.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/business_profile.dart';
import '../../providers/auth_provider.dart';
import '../../providers/business_profile_provider.dart';
import '../../providers/person_provider.dart';
import '../../services/grpc/grpc_business_profile_service.dart';

class BusinessProfileSignUpPage extends StatefulWidget {
  const BusinessProfileSignUpPage({super.key});

  @override
  State<BusinessProfileSignUpPage> createState() => _BusinessProfileSignUpPageState();
}

class _BusinessProfileSignUpPageState extends State<BusinessProfileSignUpPage> {
  final _formKey = GlobalKey<FormState>();
  final _businessNameController = TextEditingController();
  final _socialNameController = TextEditingController();
  final _taxIdController = TextEditingController();
  bool _submitting = false;
  String? _error;

  @override
  void dispose() {
    _businessNameController.dispose();
    _socialNameController.dispose();
    _taxIdController.dispose();
    super.dispose();
  }

  InputDecoration _inputDecoration(String hint) {
    return InputDecoration(
      hintText: hint,
      filled: true,
      fillColor: Colors.white,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(4),
        borderSide: const BorderSide(color: AppColors.primary),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(4),
        borderSide: const BorderSide(color: AppColors.primary),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(4),
        borderSide: const BorderSide(color: AppColors.primaryHover, width: 2),
      ),
    );
  }

  Future<void> _handleSubmit(String businessType) async {
    if (!_formKey.currentState!.validate()) return;

    final authProvider = context.read<AuthProvider>();
    final personProvider = context.read<PersonProvider>();
    final businessProfileProvider = context.read<BusinessProfileProvider>();
    final token = authProvider.auth?.accessToken ?? '';
    final person = personProvider.person;

    if (token.isEmpty || person == null) {
      setState(() => _error = AppLocalizations.of(context)!.businessProfileFormError);
      return;
    }

    setState(() {
      _submitting = true;
      _error = null;
    });

    try {
      final created = await GrpcBusinessProfileService.addBusinessProfile(
        BusinessProfile(
          ownerId: person.id,
          ownerUuid: person.uuid,
          taxId: _taxIdController.text.trim(),
          businessName: _businessNameController.text.trim(),
          businessType: businessType,
          socialName: _socialNameController.text.trim(),
        ),
      );

      await personProvider.fetchMe(token);
      final refreshedPerson = personProvider.person;
      final index = refreshedPerson?.profiles.indexWhere(
            (p) => p.profileUuid == created.uuid,
          ) ??
          -1;

      if (index == -1) {
        throw Exception('Created business profile not found in refreshed profile list');
      }

      await personProvider.switchProfile(index, token);
      await businessProfileProvider.load(uuid: created.uuid);

      if (!mounted) return;
      Navigator.of(context).pushNamedAndRemoveUntil('/feed', (route) => false);
    } catch (_) {
      if (!mounted) return;
      setState(() => _error = AppLocalizations.of(context)!.businessProfileFormError);
    } finally {
      if (mounted) {
        setState(() => _submitting = false);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final businessType = ModalRoute.of(context)!.settings.arguments as String;

    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(gradient: AppColors.gradient3),
        child: SafeArea(
          child: Center(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(24),
              child: ConstrainedBox(
                constraints: const BoxConstraints(maxWidth: 480),
                child: Card(
                  elevation: 4,
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                  child: Padding(
                    padding: const EdgeInsets.all(24),
                    child: Form(
                      key: _formKey,
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            businessType == 'Company'
                                ? l10n.businessProfileFormTitleGym
                                : l10n.businessProfileFormTitlePersonalTrainer,
                            style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
                          ),
                          const SizedBox(height: 16),
                          if (_error != null) ...[
                            Container(
                              padding: const EdgeInsets.all(12),
                              decoration: BoxDecoration(
                                color: AppColors.danger.withAlpha(25),
                                borderRadius: BorderRadius.circular(4),
                                border: Border.all(color: AppColors.danger.withAlpha(76)),
                              ),
                              child: Text(_error!, style: const TextStyle(color: AppColors.danger, fontSize: 14)),
                            ),
                            const SizedBox(height: 16),
                          ],
                          TextFormField(
                            key: const ValueKey('business_name_field'),
                            controller: _businessNameController,
                            decoration: _inputDecoration(l10n.businessProfileFormBusinessName),
                            validator: (value) => (value == null || value.trim().isEmpty)
                                ? l10n.businessProfileFormBusinessNameRequired
                                : null,
                          ),
                          const SizedBox(height: 12),
                          TextFormField(
                            key: const ValueKey('social_name_field'),
                            controller: _socialNameController,
                            decoration: _inputDecoration(l10n.businessProfileFormSocialName),
                            validator: (value) => (value == null || value.trim().isEmpty)
                                ? l10n.businessProfileFormSocialNameRequired
                                : null,
                          ),
                          const SizedBox(height: 12),
                          TextFormField(
                            key: const ValueKey('tax_id_field'),
                            controller: _taxIdController,
                            decoration: _inputDecoration(l10n.businessProfileFormTaxId),
                            validator: (value) => (value == null || value.trim().isEmpty)
                                ? l10n.businessProfileFormTaxIdRequired
                                : null,
                          ),
                          const SizedBox(height: 24),
                          SizedBox(
                            width: double.infinity,
                            child: ElevatedButton(
                              onPressed: _submitting ? null : () => _handleSubmit(businessType),
                              style: ElevatedButton.styleFrom(
                                backgroundColor: AppColors.primary,
                                disabledBackgroundColor: AppColors.primaryDisabled,
                                foregroundColor: Colors.white,
                                padding: const EdgeInsets.symmetric(vertical: 14),
                              ),
                              child: _submitting
                                  ? const SizedBox(
                                      width: 20,
                                      height: 20,
                                      child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white),
                                    )
                                  : Text(l10n.businessProfileFormSubmit),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
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

- [ ] **Step 2: Write the widget test**

Create `test/business_profile_sign_up_page_test.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:provider/provider.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:lapidation_mobile/l10n/app_localizations.dart';
import 'package:lapidation_mobile/pages/business_profile/business_profile_sign_up_page.dart';
import 'package:lapidation_mobile/providers/auth_provider.dart';
import 'package:lapidation_mobile/providers/business_profile_provider.dart';
import 'package:lapidation_mobile/providers/person_provider.dart';

void main() {
  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  testWidgets('shows validation errors when required fields are empty', (tester) async {
    await tester.pumpWidget(
      MultiProvider(
        providers: [
          ChangeNotifierProvider(create: (_) => AuthProvider()),
          ChangeNotifierProvider(create: (_) => PersonProvider()),
          ChangeNotifierProvider(create: (_) => BusinessProfileProvider()),
        ],
        child: MaterialApp(
          localizationsDelegates: AppLocalizations.localizationsDelegates,
          supportedLocales: AppLocalizations.supportedLocales,
          onGenerateRoute: (settings) => MaterialPageRoute(
            settings: settings,
            builder: (_) => const BusinessProfileSignUpPage(),
          ),
          initialRoute: '/add-profile/form',
        ),
      ),
    );
    await tester.pumpAndSettle();

    await tester.tap(find.widgetWithText(ElevatedButton, 'Create profile'));
    await tester.pumpAndSettle();

    expect(find.text('Please enter a business name'), findsOneWidget);
    expect(find.text('Please enter a social name'), findsOneWidget);
    expect(find.text('Please enter a tax ID'), findsOneWidget);
  });
}
```

- [ ] **Step 3: Run test to verify it passes**

Run: `flutter test test/business_profile_sign_up_page_test.dart`
Expected: `00:0X +1: All tests passed!` (the test navigates to `/add-profile/form` without passing `arguments`, so `businessType` reads as `null` cast to `String` — note: adjust the test's `initialRoute` navigation to pass `arguments: 'Professional'` via `Navigator.pushNamed` in a `builder` wrapper if this cast fails; the straightforward fix is asserting the route settings include arguments, e.g. wrap `home` with a widget that calls `Navigator.pushNamed(context, '/add-profile/form', arguments: 'Professional')` in `initState` instead of relying on `initialRoute`, since `initialRoute` does not carry `arguments`.)

- [ ] **Step 4: Fix the test's route setup and re-run**

Replace the `MaterialApp` in the test with one that pushes the route with arguments explicitly:

```dart
        child: MaterialApp(
          localizationsDelegates: AppLocalizations.localizationsDelegates,
          supportedLocales: AppLocalizations.supportedLocales,
          home: Builder(
            builder: (context) {
              WidgetsBinding.instance.addPostFrameCallback((_) {
                Navigator.of(context).pushNamed('/add-profile/form', arguments: 'Professional');
              });
              return const Scaffold(body: SizedBox.shrink());
            },
          ),
          onGenerateRoute: (settings) => MaterialPageRoute(
            settings: settings,
            builder: (_) => const BusinessProfileSignUpPage(),
          ),
        ),
```

Run: `flutter test test/business_profile_sign_up_page_test.dart`
Expected: `00:0X +1: All tests passed!`

- [ ] **Step 5: Commit**

```bash
git add lib/pages/business_profile/business_profile_sign_up_page.dart test/business_profile_sign_up_page_test.dart
git commit -m "Add BusinessProfileSignUpPage creation form"
```

---

### Task 13: `BusinessProfilePage` — full edit page

**Files:**
- Create: `lib/pages/business_profile/business_profile_page.dart`

**Interfaces:**
- Consumes: `MainLayout`, `BusinessProfileProvider` (Task 7), `PersonProvider.activeProfile` (Task 8), `BusinessProfile`/`BusinessProfileAddress` models, l10n keys (Task 10).
- Produces: `BusinessProfilePage extends StatefulWidget`; self-loads via `BusinessProfileProvider.load(uuid: activeProfile.profileUuid)` when `current` is null on mount.

- [ ] **Step 1: Write the implementation**

Create `lib/pages/business_profile/business_profile_page.dart`:

```dart
import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../config/nav_section.dart';
import '../../l10n/app_localizations.dart';
import '../../models/business_profile.dart';
import '../../models/business_profile_address.dart';
import '../../providers/auth_provider.dart';
import '../../providers/business_profile_provider.dart';
import '../../providers/person_provider.dart';
import '../../widgets/main_layout.dart';

class BusinessProfilePage extends StatefulWidget {
  const BusinessProfilePage({super.key});

  @override
  State<BusinessProfilePage> createState() => _BusinessProfilePageState();
}

class _BusinessProfilePageState extends State<BusinessProfilePage> {
  final _businessNameController = TextEditingController();
  final _socialNameController = TextEditingController();
  final _taxIdController = TextEditingController();
  bool _isEditing = false;

  bool _isAddressFormExpanded = false;
  BusinessProfileAddress? _editingAddress;
  final _addressLine1Controller = TextEditingController();
  final _addressLine2Controller = TextEditingController();
  final _localityController = TextEditingController();
  final _administrativeAreaController = TextEditingController();
  final _postalCodeController = TextEditingController();
  final _countryCodeController = TextEditingController();

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _ensureLoaded());
  }

  Future<void> _ensureLoaded() async {
    final provider = context.read<BusinessProfileProvider>();
    if (provider.current != null) {
      _captureOriginalValues(provider.current!);
      return;
    }
    final activeProfile = context.read<PersonProvider>().activeProfile;
    if (activeProfile == null) return;
    await provider.load(uuid: activeProfile.profileUuid);
    final loaded = provider.current;
    if (loaded != null && mounted) _captureOriginalValues(loaded);
  }

  void _captureOriginalValues(BusinessProfile profile) {
    _businessNameController.text = profile.businessName;
    _socialNameController.text = profile.socialName ?? '';
    _taxIdController.text = profile.taxId;
  }

  @override
  void dispose() {
    _businessNameController.dispose();
    _socialNameController.dispose();
    _taxIdController.dispose();
    _addressLine1Controller.dispose();
    _addressLine2Controller.dispose();
    _localityController.dispose();
    _administrativeAreaController.dispose();
    _postalCodeController.dispose();
    _countryCodeController.dispose();
    super.dispose();
  }

  Future<void> _saveChanges() async {
    final provider = context.read<BusinessProfileProvider>();
    final current = provider.current;
    if (current == null) return;
    final updated = BusinessProfile(
      id: current.id,
      uuid: current.uuid,
      ownerId: current.ownerId,
      ownerUuid: current.ownerUuid,
      taxId: _taxIdController.text.trim(),
      businessName: _businessNameController.text.trim(),
      businessType: current.businessType,
      socialName: _socialNameController.text.trim(),
      logo: current.logo,
      coverImage: current.coverImage,
    );
    final success = await provider.update(updated);
    if (!mounted) return;
    if (success) {
      setState(() => _isEditing = false);
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(provider.error ?? 'Failed to save changes.')),
      );
    }
  }

  Future<void> _pickImage(ImageSource source, {required bool isLogo}) async {
    final picked = await ImagePicker().pickImage(
      source: source,
      maxWidth: isLogo ? 512 : 1920,
      maxHeight: isLogo ? 512 : 1080,
      imageQuality: 85,
    );
    if (picked == null) return;
    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
    final provider = context.read<BusinessProfileProvider>();
    final success = isLogo
        ? await provider.uploadLogo(token, picked)
        : await provider.uploadCover(token, picked);
    if (!mounted) return;
    if (!success) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(provider.error ?? 'Failed to upload image.')),
      );
    }
  }

  void _showImageSourceDialog({required bool isLogo}) {
    showModalBottomSheet(
      context: context,
      builder: (sheetContext) => SafeArea(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            if (!kIsWeb)
              ListTile(
                leading: const Icon(Icons.camera_alt_outlined),
                title: const Text('Camera'),
                onTap: () {
                  Navigator.pop(sheetContext);
                  _pickImage(ImageSource.camera, isLogo: isLogo);
                },
              ),
            ListTile(
              leading: const Icon(Icons.photo_library_outlined),
              title: const Text('Gallery'),
              onTap: () {
                Navigator.pop(sheetContext);
                _pickImage(ImageSource.gallery, isLogo: isLogo);
              },
            ),
          ],
        ),
      ),
    );
  }

  void _openAddressForm(BusinessProfileAddress? address) {
    setState(() {
      _editingAddress = address;
      _isAddressFormExpanded = true;
      _addressLine1Controller.text = address?.addressLine1 ?? '';
      _addressLine2Controller.text = address?.addressLine2 ?? '';
      _localityController.text = address?.locality ?? '';
      _administrativeAreaController.text = address?.administrativeArea ?? '';
      _postalCodeController.text = address?.postalCode ?? '';
      _countryCodeController.text = address?.countryCode ?? '';
    });
  }

  void _closeAddressForm() {
    setState(() {
      _editingAddress = null;
      _isAddressFormExpanded = false;
    });
  }

  Future<void> _saveAddress() async {
    final provider = context.read<BusinessProfileProvider>();
    final current = provider.current;
    if (current == null || current.id == null) return;
    final address = BusinessProfileAddress(
      id: _editingAddress?.id,
      uuid: _editingAddress?.uuid,
      businessProfileId: current.id!,
      addressLine1: _addressLine1Controller.text.trim(),
      addressLine2: _addressLine2Controller.text.trim(),
      locality: _localityController.text.trim(),
      administrativeArea: _administrativeAreaController.text.trim(),
      postalCode: _postalCodeController.text.trim(),
      countryCode: _countryCodeController.text.trim(),
    );
    final success = _editingAddress == null
        ? await provider.addAddress(address)
        : await provider.updateAddress(address);
    if (!mounted) return;
    if (success) {
      _closeAddressForm();
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(provider.error ?? 'Failed to save address.')),
      );
    }
  }

  Future<void> _confirmDeleteAddress(BusinessProfileAddress address) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (dialogContext) => AlertDialog(
        title: const Text('Delete address?'),
        content: Text(address.addressLine1),
        actions: [
          TextButton(onPressed: () => Navigator.pop(dialogContext, false), child: const Text('Cancel')),
          TextButton(
            onPressed: () => Navigator.pop(dialogContext, true),
            child: const Text('Delete', style: TextStyle(color: AppColors.danger)),
          ),
        ],
      ),
    );
    if (confirmed != true) return;
    if (!mounted) return;
    await context.read<BusinessProfileProvider>().removeAddress(id: address.id, uuid: address.uuid);
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return MainLayout(
      navSection: NavSection.home,
      currentRoute: '/business-profile',
      body: Consumer<BusinessProfileProvider>(
        builder: (context, provider, _) {
          final profile = provider.current;
          if (provider.loading || profile == null) {
            return const Center(child: CircularProgressIndicator());
          }
          return SingleChildScrollView(
            child: Column(
              children: [
                _buildHeader(profile),
                Padding(
                  padding: const EdgeInsets.all(16),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      _buildFieldsSection(l10n, profile),
                      const SizedBox(height: 24),
                      _buildAddressesSection(l10n, profile),
                    ],
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _buildHeader(BusinessProfile profile) {
    return Stack(
      clipBehavior: Clip.none,
      children: [
        SizedBox(
          height: 180,
          width: double.infinity,
          child: profile.coverImage != null
              ? CachedNetworkImage(imageUrl: profile.coverImage!, fit: BoxFit.cover)
              : Container(color: AppColors.professionalSecondaryDisabled),
        ),
        Positioned(
          top: 8,
          right: 8,
          child: IconButton(
            style: IconButton.styleFrom(backgroundColor: Colors.black.withAlpha(102)),
            icon: const Icon(Icons.camera_alt, color: Colors.white),
            onPressed: () => _showImageSourceDialog(isLogo: false),
          ),
        ),
        Positioned(
          left: 16,
          bottom: -32,
          child: Stack(
            children: [
              CircleAvatar(
                radius: 48,
                backgroundColor: Colors.white,
                child: ClipOval(
                  child: SizedBox(
                    width: 88,
                    height: 88,
                    child: profile.logo != null
                        ? CachedNetworkImage(imageUrl: profile.logo!, fit: BoxFit.cover)
                        : const Icon(Icons.storefront, size: 40, color: AppColors.professionalSecondary),
                  ),
                ),
              ),
              Positioned(
                right: 0,
                bottom: 0,
                child: IconButton(
                  style: IconButton.styleFrom(backgroundColor: AppColors.professionalSecondary),
                  icon: const Icon(Icons.camera_alt, color: Colors.white, size: 16),
                  onPressed: () => _showImageSourceDialog(isLogo: true),
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildFieldsSection(AppLocalizations l10n, BusinessProfile profile) {
    return Padding(
      padding: const EdgeInsets.only(top: 40),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                profile.businessType == 'Company'
                    ? l10n.addProfileGymTitle
                    : l10n.addProfilePersonalTrainerTitle,
                style: const TextStyle(fontSize: 12, color: Colors.grey, fontWeight: FontWeight.w600),
              ),
              TextButton(
                onPressed: () {
                  if (_isEditing) {
                    _saveChanges();
                  } else {
                    setState(() => _isEditing = true);
                  }
                },
                child: Text(_isEditing ? l10n.businessProfilePageSave : l10n.businessProfilePageEdit),
              ),
            ],
          ),
          const SizedBox(height: 8),
          _buildTextField(l10n.businessProfileFormBusinessName, _businessNameController, enabled: _isEditing),
          const SizedBox(height: 12),
          _buildTextField(l10n.businessProfileFormSocialName, _socialNameController, enabled: _isEditing),
          const SizedBox(height: 12),
          _buildTextField(l10n.businessProfileFormTaxId, _taxIdController, enabled: _isEditing),
        ],
      ),
    );
  }

  Widget _buildTextField(String label, TextEditingController controller, {required bool enabled}) {
    return TextFormField(
      controller: controller,
      enabled: enabled,
      decoration: InputDecoration(
        labelText: label,
        filled: !enabled,
        fillColor: enabled ? null : Colors.grey[50],
        border: const OutlineInputBorder(),
      ),
    );
  }

  Widget _buildAddressesSection(AppLocalizations l10n, BusinessProfile profile) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(l10n.businessProfilePageAddresses, style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
            IconButton(
              icon: const Icon(Icons.add_circle_outline, color: AppColors.professionalSecondary),
              onPressed: () => _openAddressForm(null),
            ),
          ],
        ),
        if (_isAddressFormExpanded) _buildAddressForm(),
        ...profile.addresses.map((address) => _buildAddressCard(address)),
      ],
    );
  }

  Widget _buildAddressForm() {
    return Card(
      margin: const EdgeInsets.symmetric(vertical: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          children: [
            TextFormField(
              controller: _addressLine1Controller,
              decoration: const InputDecoration(labelText: 'Address line 1'),
            ),
            const SizedBox(height: 8),
            TextFormField(
              controller: _addressLine2Controller,
              decoration: const InputDecoration(labelText: 'Address line 2'),
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Expanded(
                  child: TextFormField(
                    controller: _localityController,
                    decoration: const InputDecoration(labelText: 'City'),
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: TextFormField(
                    controller: _administrativeAreaController,
                    decoration: const InputDecoration(labelText: 'State'),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Expanded(
                  child: TextFormField(
                    controller: _postalCodeController,
                    decoration: const InputDecoration(labelText: 'Postal code'),
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: TextFormField(
                    controller: _countryCodeController,
                    decoration: const InputDecoration(labelText: 'Country code'),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                TextButton(onPressed: _closeAddressForm, child: const Text('Cancel')),
                const SizedBox(width: 8),
                ElevatedButton(
                  onPressed: _saveAddress,
                  style: ElevatedButton.styleFrom(backgroundColor: AppColors.professionalSecondary),
                  child: const Text('Save', style: TextStyle(color: Colors.white)),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildAddressCard(BusinessProfileAddress address) {
    return Card(
      margin: const EdgeInsets.symmetric(vertical: 4),
      child: ListTile(
        title: Text(address.addressLine1),
        subtitle: Text('${address.locality}, ${address.administrativeArea} ${address.postalCode}'),
        trailing: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            IconButton(
              icon: const Icon(Icons.edit_outlined, size: 20),
              onPressed: () => _openAddressForm(address),
            ),
            IconButton(
              icon: const Icon(Icons.delete_outline, size: 20, color: AppColors.danger),
              onPressed: () => _confirmDeleteAddress(address),
            ),
          ],
        ),
      ),
    );
  }
}
```

- [ ] **Step 2: Verify it compiles**

```bash
dart analyze lib/pages/business_profile/business_profile_page.dart
```

Expected: `No issues found!`

- [ ] **Step 3: Commit**

```bash
git add lib/pages/business_profile/business_profile_page.dart
git commit -m "Add BusinessProfilePage full-edit view"
```

---

### Task 14: `main.dart` — register routes and provider, dynamic theming

**Files:**
- Modify: `lib/main.dart`

**Interfaces:**
- Consumes: `AddProfilePage` (Task 11), `BusinessProfileSignUpPage` (Task 12), `BusinessProfilePage` (Task 13), `BusinessProfileProvider` (Task 7), `PersonProvider.isProfessional` (Task 8).

- [ ] **Step 1: Add imports**

In `lib/main.dart`, add these imports next to the existing page/provider imports:

```dart
import 'pages/business_profile/add_profile_page.dart';
import 'pages/business_profile/business_profile_page.dart';
import 'pages/business_profile/business_profile_sign_up_page.dart';
import 'providers/business_profile_provider.dart';
```

- [ ] **Step 2: Register the provider**

In the `MultiProvider.providers` list, add next to `ChangeNotifierProvider(create: (_) => PersonProvider())`:

```dart
        ChangeNotifierProvider(create: (_) => BusinessProfileProvider()),
```

- [ ] **Step 3: Make the theme react to professional mode**

Replace `Consumer<LocaleProvider>` with `Consumer2<LocaleProvider, PersonProvider>` and thread `isProfessional` into the seed color. Replace:

```dart
      child: Consumer<LocaleProvider>(
        builder: (context, localeProvider, _) {
```

with:

```dart
      child: Consumer2<LocaleProvider, PersonProvider>(
        builder: (context, localeProvider, personProvider, _) {
```

Then replace:

```dart
            theme: ThemeData(
              fontFamilyFallback: _fontFamilyFallbacks,
              colorScheme: ColorScheme.fromSeed(
                seedColor: AppColors.primary,
                surface: AppColors.background,
              ),
              useMaterial3: true,
            ),
```

with:

```dart
            theme: ThemeData(
              fontFamilyFallback: _fontFamilyFallbacks,
              colorScheme: ColorScheme.fromSeed(
                seedColor: personProvider.isProfessional
                    ? AppColors.professionalSecondary
                    : AppColors.primary,
                surface: AppColors.background,
              ),
              useMaterial3: true,
            ),
```

- [ ] **Step 4: Register the three new routes**

In the `routes` map, add next to the existing `'/profile'` entry:

```dart
              '/add-profile': (context) => const AddProfilePage(),
              '/add-profile/form': (context) => const BusinessProfileSignUpPage(),
              '/business-profile': (context) => const BusinessProfilePage(),
```

- [ ] **Step 5: Verify it compiles**

```bash
dart analyze lib/main.dart
```

Expected: `No issues found!`

- [ ] **Step 6: Manual smoke test**

```bash
flutter run -d linux
```

Sign in, open the profile menu, tap "Add New Profile" — the `AddProfilePage` should render both cards without a route-not-found error (this is the bug the spec set out to fix). Stop the app afterward.

- [ ] **Step 7: Commit**

```bash
git add lib/main.dart
git commit -m "Register business profile routes, provider, and professional theming"
```

---

### Task 15: `AppHeader` — gradient swap in professional mode

**Files:**
- Modify: `lib/widgets/app_header.dart`

**Interfaces:**
- Consumes: `PersonProvider.isProfessional` (Task 8), `AppColors.professionalGradient3` (existing).

- [ ] **Step 1: Read the professional flag and swap the gradient**

In `lib/widgets/app_header.dart`, add `import 'package:provider/provider.dart';` and `import '../providers/person_provider.dart';` if not already present, then inside `build`, before the root `Container` is returned, add:

```dart
    final isProfessional = context.watch<PersonProvider>().isProfessional;
```

Then change the root `Container`'s `decoration`:

```dart
        decoration: BoxDecoration(gradient: AppColors.gradient3, boxShadow: [...]),
```

to:

```dart
        decoration: BoxDecoration(
          gradient: isProfessional ? AppColors.professionalGradient3 : AppColors.gradient3,
          boxShadow: [...],
        ),
```

(keep the existing `boxShadow` list as-is — only the `gradient` value changes.)

- [ ] **Step 2: Verify it compiles**

```bash
dart analyze lib/widgets/app_header.dart
```

Expected: `No issues found!`

- [ ] **Step 3: Commit**

```bash
git add lib/widgets/app_header.dart
git commit -m "Swap AppHeader gradient to professional identity in professional mode"
```

---

### Task 16: `MainLayout` — conditional Profile destination

**Files:**
- Modify: `lib/widgets/main_layout.dart`

**Interfaces:**
- Consumes: `PersonProvider.isProfessional` (Task 8).

- [ ] **Step 1: Route to `/business-profile` when professional**

In `lib/widgets/main_layout.dart`, find the `AppHeader(..., onProfilePressed: () { Navigator.of(context).pushNamedAndRemoveUntil('/profile', ...); }, ...)` call and change the callback body:

```dart
onProfilePressed: () {
  Navigator.of(context).pushNamedAndRemoveUntil('/profile', (route) => false);
},
```

to:

```dart
onProfilePressed: () {
  final destination = context.read<PersonProvider>().isProfessional ? '/business-profile' : '/profile';
  Navigator.of(context).pushNamedAndRemoveUntil(destination, (route) => false);
},
```

Ensure `import 'package:provider/provider.dart';` and `import '../providers/person_provider.dart';` are present at the top of the file (add them if missing).

- [ ] **Step 2: Verify it compiles**

```bash
dart analyze lib/widgets/main_layout.dart
```

Expected: `No issues found!`

- [ ] **Step 3: Commit**

```bash
git add lib/widgets/main_layout.dart
git commit -m "Route MainLayout's profile action to the business profile page in professional mode"
```

---

### Task 17: `SidebarMenu` — conditional Profile item destination

**Files:**
- Modify: `lib/widgets/sidebar_menu.dart`

**Interfaces:**
- Consumes: `PersonProvider.isProfessional` (Task 8).

- [ ] **Step 1: Route the Profile sidebar item conditionally**

In `lib/widgets/sidebar_menu.dart`, inside `_homeItems`, find the Profile `_SidebarItem`:

```dart
      _SidebarItem(
        icon: Icons.person_outline,
        activeIcon: Icons.person,
        label: l10n.menuProfile,
        isActive: currentRoute == '/profile',
        isCollapsed: isCollapsed,
        onTap: () => _go(context, '/profile'),
      ),
```

Replace it with:

```dart
      _SidebarItem(
        icon: Icons.person_outline,
        activeIcon: Icons.person,
        label: l10n.menuProfile,
        isActive: currentRoute == '/profile' || currentRoute == '/business-profile',
        isCollapsed: isCollapsed,
        onTap: () => _go(
          context,
          context.read<PersonProvider>().isProfessional ? '/business-profile' : '/profile',
        ),
      ),
```

(`context.read<PersonProvider>()` is safe here because `_homeItems` is called from `build`, which already runs inside this `StatelessWidget`'s build context; `PersonProvider` is already imported in this file.)

- [ ] **Step 2: Verify it compiles**

```bash
dart analyze lib/widgets/sidebar_menu.dart
```

Expected: `No issues found!`

- [ ] **Step 3: Commit**

```bash
git add lib/widgets/sidebar_menu.dart
git commit -m "Route SidebarMenu's Profile item to the business profile page in professional mode"
```

---

### Task 18: `ProfileMenu` — real profile switching + "Personal account" entry

**Files:**
- Modify: `lib/widgets/profile_menu.dart`

**Interfaces:**
- Consumes: `PersonProvider.switchProfile`/`switchToPersonal`/`isProfessional` (Task 8), `businessProfileSwitchToPersonal` l10n key (Task 10).

- [ ] **Step 1: Add "Personal account" to the `PopupMenuButton` variant**

In `lib/widgets/profile_menu.dart`, inside `_buildMenuItems`, after the `profiles.asMap().entries.map(...)` spread and before the `// Add New Profile` `PopupMenuItem`, add:

```dart
      if (Provider.of<PersonProvider>(context, listen: false).isProfessional)
        PopupMenuItem(
          value: 'switch_personal',
          child: Row(
            children: [
              const Icon(Icons.account_circle_outlined, color: AppColors.primary, size: 20),
              const SizedBox(width: 12),
              Text(l10n.businessProfileSwitchToPersonal),
            ],
          ),
        ),
```

- [ ] **Step 2: Handle `switch_personal` in `_handleMenuAction`**

In `_handleMenuAction`, after the `if (value.startsWith('profile_')) { ... return; }` block and before the `switch (value) {` statement, the existing `profile_` branch already calls `switchProfile` and returns. Add a new case inside the `switch (value)` block, alongside `'add_profile'`:

```dart
      case 'switch_personal':
        context.read<PersonProvider>().switchToPersonal();
        Navigator.of(context).pushNamedAndRemoveUntil('/feed', (route) => false);
        break;
```

- [ ] **Step 3: Also navigate to `/feed` after switching to a listed profile**

In `_handleMenuAction`, the existing `profile_` branch currently only calls `switchProfile` and returns without navigating. Replace:

```dart
    if (value.startsWith('profile_')) {
      // Extract profile index and switch profile
      final index = int.parse(value.replaceFirst('profile_', ''));
      final person = context.read<PersonProvider>().person;
      if (person != null && index < person.profiles.length) {
        final token = context.read<AuthProvider>().auth?.accessToken ?? '';
        if (token.isNotEmpty) {
          context.read<PersonProvider>().switchProfile(index, token);
        }
      }
      return;
    }
```

with:

```dart
    if (value.startsWith('profile_')) {
      // Extract profile index and switch profile
      final index = int.parse(value.replaceFirst('profile_', ''));
      final person = context.read<PersonProvider>().person;
      if (person != null && index < person.profiles.length) {
        final token = context.read<AuthProvider>().auth?.accessToken ?? '';
        if (token.isNotEmpty) {
          context.read<PersonProvider>().switchProfile(index, token);
          Navigator.of(context).pushNamedAndRemoveUntil('/feed', (route) => false);
        }
      }
      return;
    }
```

- [ ] **Step 4: Add "Personal account" and navigate-after-switch to the modal (`_ProfileMenuModal`) variant**

In `_ProfileMenuModal.build`, after the `...profiles.asMap().entries.map(...)` spread and before the `// Add New Profile` `_MenuOption`, add:

```dart
                              if (context.watch<PersonProvider>().isProfessional)
                                _MenuOption(
                                  icon: Icons.account_circle_outlined,
                                  label: l10n.businessProfileSwitchToPersonal,
                                  onPressed: () {
                                    onClose?.call();
                                    context.read<PersonProvider>().switchToPersonal();
                                    Navigator.of(context).pushNamedAndRemoveUntil('/feed', (route) => false);
                                  },
                                ),
```

Then, inside the existing profile-tap `onPressed` in that same modal (the `_ProfileOption` builder), add the `/feed` navigation after `switchProfile`. Replace:

```dart
                                  onPressed: () {
                                    onClose?.call();
                                    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
                                    if (token.isNotEmpty) {
                                      context.read<PersonProvider>().switchProfile(entry.key, token);
                                    }
                                  },
```

with:

```dart
                                  onPressed: () {
                                    onClose?.call();
                                    final token = context.read<AuthProvider>().auth?.accessToken ?? '';
                                    if (token.isNotEmpty) {
                                      context.read<PersonProvider>().switchProfile(entry.key, token);
                                      Navigator.of(context).pushNamedAndRemoveUntil('/feed', (route) => false);
                                    }
                                  },
```

- [ ] **Step 5: Verify it compiles**

```bash
dart analyze lib/widgets/profile_menu.dart
```

Expected: `No issues found!`

- [ ] **Step 6: Commit**

```bash
git add lib/widgets/profile_menu.dart
git commit -m "Wire real profile switching and add Personal account entry to ProfileMenu"
```

---

### Task 19: Final verification

**Files:** none (verification only)

- [ ] **Step 1: Full static analysis**

```bash
cd /home/erocha/workspace/lapidation_project/lapidation_mobile
dart analyze
```

Expected: `No issues found!`

- [ ] **Step 2: Full test suite**

```bash
flutter test
```

Expected: all tests pass, including every test added in Tasks 2-4, 6, 8, 11-12 alongside the pre-existing suite.

- [ ] **Step 3: Manual end-to-end smoke test**

```bash
flutter run -d linux
```

Walk through: sign in → profile menu → "Add New Profile" → tap "Personal Trainer" → fill business name/social name/tax id → submit → confirm the app lands on `/feed` with the `professionalGradient3` header, the sidebar's Profile item opens `/business-profile`, editing fields there and saving works (or surfaces the expected error banner if the backend RPC isn't implemented yet), and the profile menu now shows "Personal account" to switch back. Stop the app afterward.

- [ ] **Step 4: Report back to the user**

Summarize what's implemented and, explicitly, which server-side RPCs (`UpdateBusinessProfile`, `AddBusinessProfileAddress`, `UpdateBusinessProfileAddress`, `RemoveBusinessProfileAddress`) still need a `workout` implementation before the business profile page's edit/address features work end-to-end — everything else (`AddBusinessProfile`, `GetBusinessProfileById`) already works against the existing backend.

---

## Self-Review

**Spec coverage:**
- Type-selection page with Personal Trainer + Gym cards → Task 11. ✅
- Creation form (business name, social name, tax id) using `AddBusinessProfile` → Task 12. ✅
- Post-creation flow (`fetchMe` → `switchProfile` → `BusinessProfileProvider.load` → navigate to `/feed`) → Task 12 Step 1. ✅
- Local active-profile state, persisted, no backend switch call → Task 8. ✅
- Professional Feed/Gallery/Workout reuse existing pages unchanged → no task touches those pages; only theming/navigation wrappers change (Tasks 14-17). ✅
- Full-edit business profile page (fields, logo/cover upload, address CRUD) → Task 13. ✅
- Proto additions + sync + regeneration → Task 1. ✅
- Gym card same form with `business_type = "Company"` → Task 11's `businessProfileTypeOptions`. ✅
- JWT claim decoding + login restore → Tasks 2, 8, 9. ✅
- Sidebar Profile item + header profile action routing → Tasks 16-17. ✅
- ProfileMenu real switch + "Personal account" entry → Task 18. ✅
- l10n strings across all six locales → Task 10. ✅
- Testing (unit: provider/JWT/model; widget: cards, form validation) → Tasks 2-4, 6, 8, 11-12. Note: `AppHeader` gradient-switch widget test from the spec's testing section was scoped out in favor of `dart analyze` + manual smoke verification (Task 15/19) to keep the plan bounded — flag this as a known gap if the user wants it added later.

**Placeholder scan:** no `TBD`/`TODO`/"add error handling" phrases; every step has complete, runnable code or an exact command with expected output.

**Type consistency:** `BusinessProfile`/`BusinessProfileAddress` field names and types match between Tasks 3-4 (models), 6 (façade), 7 (provider), 12-13 (pages). `PersonProvider.switchProfile(int, String) -> Future<bool>`, `.switchToPersonal() -> Future<bool>`, `.restoreActiveProfileFromToken(String) -> Future<void>`, `.activeProfile -> Profile?`, `.isProfessional -> bool` are used identically across Tasks 8, 9, 12, 13, 15-18.
