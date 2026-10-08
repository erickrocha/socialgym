// This is a generated file - do not edit.
//
// Generated from business_profile.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'business_profile_address.pb.dart' as $1;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class BusinessProfile extends $pb.GeneratedMessage {
  factory BusinessProfile({
    $core.int? id,
    $core.String? uuid,
    $core.int? ownerId,
    $core.String? ownerUuid,
    $core.String? taxId,
    $core.String? businessName,
    $core.String? businessType,
    $core.String? socialName,
    $core.String? objectKey,
    $core.String? logo,
    $core.String? coverImage,
    $core.String? createdAt,
    $core.String? updatedAt,
    $core.Iterable<$1.BusinessProfileAddress>? addresses,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    if (ownerId != null) result.ownerId = ownerId;
    if (ownerUuid != null) result.ownerUuid = ownerUuid;
    if (taxId != null) result.taxId = taxId;
    if (businessName != null) result.businessName = businessName;
    if (businessType != null) result.businessType = businessType;
    if (socialName != null) result.socialName = socialName;
    if (objectKey != null) result.objectKey = objectKey;
    if (logo != null) result.logo = logo;
    if (coverImage != null) result.coverImage = coverImage;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (addresses != null) result.addresses.addAll(addresses);
    return result;
  }

  BusinessProfile._();

  factory BusinessProfile.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BusinessProfile.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BusinessProfile',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'grpc.business_profile'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..aI(3, _omitFieldNames ? '' : 'ownerId')
    ..aOS(4, _omitFieldNames ? '' : 'ownerUuid')
    ..aOS(5, _omitFieldNames ? '' : 'taxId')
    ..aOS(6, _omitFieldNames ? '' : 'businessName')
    ..aOS(7, _omitFieldNames ? '' : 'businessType')
    ..aOS(8, _omitFieldNames ? '' : 'socialName')
    ..aOS(9, _omitFieldNames ? '' : 'objectKey')
    ..aOS(10, _omitFieldNames ? '' : 'logo')
    ..aOS(11, _omitFieldNames ? '' : 'coverImage')
    ..aOS(12, _omitFieldNames ? '' : 'createdAt')
    ..aOS(13, _omitFieldNames ? '' : 'updatedAt')
    ..pPM<$1.BusinessProfileAddress>(14, _omitFieldNames ? '' : 'addresses',
        subBuilder: $1.BusinessProfileAddress.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfile clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfile copyWith(void Function(BusinessProfile) updates) =>
      super.copyWith((message) => updates(message as BusinessProfile))
          as BusinessProfile;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BusinessProfile create() => BusinessProfile._();
  @$core.override
  BusinessProfile createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BusinessProfile getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BusinessProfile>(create);
  static BusinessProfile? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get uuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set uuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearUuid() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.int get ownerId => $_getIZ(2);
  @$pb.TagNumber(3)
  set ownerId($core.int value) => $_setSignedInt32(2, value);
  @$pb.TagNumber(3)
  $core.bool hasOwnerId() => $_has(2);
  @$pb.TagNumber(3)
  void clearOwnerId() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get ownerUuid => $_getSZ(3);
  @$pb.TagNumber(4)
  set ownerUuid($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasOwnerUuid() => $_has(3);
  @$pb.TagNumber(4)
  void clearOwnerUuid() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get taxId => $_getSZ(4);
  @$pb.TagNumber(5)
  set taxId($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasTaxId() => $_has(4);
  @$pb.TagNumber(5)
  void clearTaxId() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get businessName => $_getSZ(5);
  @$pb.TagNumber(6)
  set businessName($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasBusinessName() => $_has(5);
  @$pb.TagNumber(6)
  void clearBusinessName() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get businessType => $_getSZ(6);
  @$pb.TagNumber(7)
  set businessType($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasBusinessType() => $_has(6);
  @$pb.TagNumber(7)
  void clearBusinessType() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get socialName => $_getSZ(7);
  @$pb.TagNumber(8)
  set socialName($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasSocialName() => $_has(7);
  @$pb.TagNumber(8)
  void clearSocialName() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get objectKey => $_getSZ(8);
  @$pb.TagNumber(9)
  set objectKey($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasObjectKey() => $_has(8);
  @$pb.TagNumber(9)
  void clearObjectKey() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get logo => $_getSZ(9);
  @$pb.TagNumber(10)
  set logo($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasLogo() => $_has(9);
  @$pb.TagNumber(10)
  void clearLogo() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get coverImage => $_getSZ(10);
  @$pb.TagNumber(11)
  set coverImage($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasCoverImage() => $_has(10);
  @$pb.TagNumber(11)
  void clearCoverImage() => $_clearField(11);

  @$pb.TagNumber(12)
  $core.String get createdAt => $_getSZ(11);
  @$pb.TagNumber(12)
  set createdAt($core.String value) => $_setString(11, value);
  @$pb.TagNumber(12)
  $core.bool hasCreatedAt() => $_has(11);
  @$pb.TagNumber(12)
  void clearCreatedAt() => $_clearField(12);

  @$pb.TagNumber(13)
  $core.String get updatedAt => $_getSZ(12);
  @$pb.TagNumber(13)
  set updatedAt($core.String value) => $_setString(12, value);
  @$pb.TagNumber(13)
  $core.bool hasUpdatedAt() => $_has(12);
  @$pb.TagNumber(13)
  void clearUpdatedAt() => $_clearField(13);

  @$pb.TagNumber(14)
  $pb.PbList<$1.BusinessProfileAddress> get addresses => $_getList(13);
}

class BusinessProfileRequestId extends $pb.GeneratedMessage {
  factory BusinessProfileRequestId({
    $core.int? id,
    $core.String? uuid,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    return result;
  }

  BusinessProfileRequestId._();

  factory BusinessProfileRequestId.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BusinessProfileRequestId.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BusinessProfileRequestId',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'grpc.business_profile'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfileRequestId clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfileRequestId copyWith(
          void Function(BusinessProfileRequestId) updates) =>
      super.copyWith((message) => updates(message as BusinessProfileRequestId))
          as BusinessProfileRequestId;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BusinessProfileRequestId create() => BusinessProfileRequestId._();
  @$core.override
  BusinessProfileRequestId createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BusinessProfileRequestId getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BusinessProfileRequestId>(create);
  static BusinessProfileRequestId? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get uuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set uuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearUuid() => $_clearField(2);
}

class BusinessProfileRequestOwnerId extends $pb.GeneratedMessage {
  factory BusinessProfileRequestOwnerId({
    $core.int? ownerId,
    $core.String? ownerUuid,
  }) {
    final result = create();
    if (ownerId != null) result.ownerId = ownerId;
    if (ownerUuid != null) result.ownerUuid = ownerUuid;
    return result;
  }

  BusinessProfileRequestOwnerId._();

  factory BusinessProfileRequestOwnerId.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BusinessProfileRequestOwnerId.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BusinessProfileRequestOwnerId',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'grpc.business_profile'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'ownerId')
    ..aOS(2, _omitFieldNames ? '' : 'ownerUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfileRequestOwnerId clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfileRequestOwnerId copyWith(
          void Function(BusinessProfileRequestOwnerId) updates) =>
      super.copyWith(
              (message) => updates(message as BusinessProfileRequestOwnerId))
          as BusinessProfileRequestOwnerId;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BusinessProfileRequestOwnerId create() =>
      BusinessProfileRequestOwnerId._();
  @$core.override
  BusinessProfileRequestOwnerId createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BusinessProfileRequestOwnerId getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BusinessProfileRequestOwnerId>(create);
  static BusinessProfileRequestOwnerId? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get ownerId => $_getIZ(0);
  @$pb.TagNumber(1)
  set ownerId($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasOwnerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearOwnerId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get ownerUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set ownerUuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOwnerUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwnerUuid() => $_clearField(2);
}

class BusinessProfilesResponse extends $pb.GeneratedMessage {
  factory BusinessProfilesResponse({
    $core.Iterable<BusinessProfile>? businessProfiles,
  }) {
    final result = create();
    if (businessProfiles != null)
      result.businessProfiles.addAll(businessProfiles);
    return result;
  }

  BusinessProfilesResponse._();

  factory BusinessProfilesResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BusinessProfilesResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BusinessProfilesResponse',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'grpc.business_profile'),
      createEmptyInstance: create)
    ..pPM<BusinessProfile>(1, _omitFieldNames ? '' : 'businessProfiles',
        subBuilder: BusinessProfile.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfilesResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfilesResponse copyWith(
          void Function(BusinessProfilesResponse) updates) =>
      super.copyWith((message) => updates(message as BusinessProfilesResponse))
          as BusinessProfilesResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BusinessProfilesResponse create() => BusinessProfilesResponse._();
  @$core.override
  BusinessProfilesResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BusinessProfilesResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BusinessProfilesResponse>(create);
  static BusinessProfilesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<BusinessProfile> get businessProfiles => $_getList(0);
}

class RemoveBusinessProfileAddressRequest extends $pb.GeneratedMessage {
  factory RemoveBusinessProfileAddressRequest({
    $core.int? id,
    $core.String? uuid,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    return result;
  }

  RemoveBusinessProfileAddressRequest._();

  factory RemoveBusinessProfileAddressRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RemoveBusinessProfileAddressRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoveBusinessProfileAddressRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'grpc.business_profile'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoveBusinessProfileAddressRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoveBusinessProfileAddressRequest copyWith(
          void Function(RemoveBusinessProfileAddressRequest) updates) =>
      super.copyWith((message) =>
              updates(message as RemoveBusinessProfileAddressRequest))
          as RemoveBusinessProfileAddressRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemoveBusinessProfileAddressRequest create() =>
      RemoveBusinessProfileAddressRequest._();
  @$core.override
  RemoveBusinessProfileAddressRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RemoveBusinessProfileAddressRequest getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
          RemoveBusinessProfileAddressRequest>(create);
  static RemoveBusinessProfileAddressRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get uuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set uuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearUuid() => $_clearField(2);
}

class RemoveBusinessProfileAddressResponse extends $pb.GeneratedMessage {
  factory RemoveBusinessProfileAddressResponse({
    $core.bool? success,
  }) {
    final result = create();
    if (success != null) result.success = success;
    return result;
  }

  RemoveBusinessProfileAddressResponse._();

  factory RemoveBusinessProfileAddressResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RemoveBusinessProfileAddressResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemoveBusinessProfileAddressResponse',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'grpc.business_profile'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoveBusinessProfileAddressResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemoveBusinessProfileAddressResponse copyWith(
          void Function(RemoveBusinessProfileAddressResponse) updates) =>
      super.copyWith((message) =>
              updates(message as RemoveBusinessProfileAddressResponse))
          as RemoveBusinessProfileAddressResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemoveBusinessProfileAddressResponse create() =>
      RemoveBusinessProfileAddressResponse._();
  @$core.override
  RemoveBusinessProfileAddressResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RemoveBusinessProfileAddressResponse getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<
          RemoveBusinessProfileAddressResponse>(create);
  static RemoveBusinessProfileAddressResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);
}

class BusinessProfileImageUploadRequest extends $pb.GeneratedMessage {
  factory BusinessProfileImageUploadRequest({
    $core.String? imageType,
    $core.String? format,
  }) {
    final result = create();
    if (imageType != null) result.imageType = imageType;
    if (format != null) result.format = format;
    return result;
  }

  BusinessProfileImageUploadRequest._();

  factory BusinessProfileImageUploadRequest.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BusinessProfileImageUploadRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BusinessProfileImageUploadRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'grpc.business_profile'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'imageType')
    ..aOS(2, _omitFieldNames ? '' : 'format')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfileImageUploadRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfileImageUploadRequest copyWith(
          void Function(BusinessProfileImageUploadRequest) updates) =>
      super.copyWith((message) =>
              updates(message as BusinessProfileImageUploadRequest))
          as BusinessProfileImageUploadRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BusinessProfileImageUploadRequest create() =>
      BusinessProfileImageUploadRequest._();
  @$core.override
  BusinessProfileImageUploadRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BusinessProfileImageUploadRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BusinessProfileImageUploadRequest>(
          create);
  static BusinessProfileImageUploadRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get imageType => $_getSZ(0);
  @$pb.TagNumber(1)
  set imageType($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasImageType() => $_has(0);
  @$pb.TagNumber(1)
  void clearImageType() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get format => $_getSZ(1);
  @$pb.TagNumber(2)
  set format($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasFormat() => $_has(1);
  @$pb.TagNumber(2)
  void clearFormat() => $_clearField(2);
}

class BusinessProfileImageUploadResponse extends $pb.GeneratedMessage {
  factory BusinessProfileImageUploadResponse({
    $core.String? url,
    $core.String? objectKey,
    $core.int? businessProfileId,
  }) {
    final result = create();
    if (url != null) result.url = url;
    if (objectKey != null) result.objectKey = objectKey;
    if (businessProfileId != null) result.businessProfileId = businessProfileId;
    return result;
  }

  BusinessProfileImageUploadResponse._();

  factory BusinessProfileImageUploadResponse.fromBuffer(
          $core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory BusinessProfileImageUploadResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BusinessProfileImageUploadResponse',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'grpc.business_profile'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'url')
    ..aOS(2, _omitFieldNames ? '' : 'objectKey')
    ..aI(3, _omitFieldNames ? '' : 'businessProfileId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfileImageUploadResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  BusinessProfileImageUploadResponse copyWith(
          void Function(BusinessProfileImageUploadResponse) updates) =>
      super.copyWith((message) =>
              updates(message as BusinessProfileImageUploadResponse))
          as BusinessProfileImageUploadResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BusinessProfileImageUploadResponse create() =>
      BusinessProfileImageUploadResponse._();
  @$core.override
  BusinessProfileImageUploadResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static BusinessProfileImageUploadResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BusinessProfileImageUploadResponse>(
          create);
  static BusinessProfileImageUploadResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get url => $_getSZ(0);
  @$pb.TagNumber(1)
  set url($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearUrl() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get objectKey => $_getSZ(1);
  @$pb.TagNumber(2)
  set objectKey($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasObjectKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearObjectKey() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.int get businessProfileId => $_getIZ(2);
  @$pb.TagNumber(3)
  set businessProfileId($core.int value) => $_setSignedInt32(2, value);
  @$pb.TagNumber(3)
  $core.bool hasBusinessProfileId() => $_has(2);
  @$pb.TagNumber(3)
  void clearBusinessProfileId() => $_clearField(3);
}

class GetActiveBusinessProfileRequest extends $pb.GeneratedMessage {
  factory GetActiveBusinessProfileRequest() => create();

  GetActiveBusinessProfileRequest._();

  factory GetActiveBusinessProfileRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GetActiveBusinessProfileRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetActiveBusinessProfileRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'grpc.business_profile'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetActiveBusinessProfileRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetActiveBusinessProfileRequest copyWith(
          void Function(GetActiveBusinessProfileRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GetActiveBusinessProfileRequest))
          as GetActiveBusinessProfileRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetActiveBusinessProfileRequest create() =>
      GetActiveBusinessProfileRequest._();
  @$core.override
  GetActiveBusinessProfileRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GetActiveBusinessProfileRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetActiveBusinessProfileRequest>(
          create);
  static GetActiveBusinessProfileRequest? _defaultInstance;
}

class DiscoverBusinessProfilesRequest extends $pb.GeneratedMessage {
  factory DiscoverBusinessProfilesRequest({
    $core.String? query,
    $core.String? businessType,
    $core.double? latitude,
    $core.double? longitude,
    $core.double? radiusKm,
    $core.int? limit,
  }) {
    final result = create();
    if (query != null) result.query = query;
    if (businessType != null) result.businessType = businessType;
    if (latitude != null) result.latitude = latitude;
    if (longitude != null) result.longitude = longitude;
    if (radiusKm != null) result.radiusKm = radiusKm;
    if (limit != null) result.limit = limit;
    return result;
  }

  DiscoverBusinessProfilesRequest._();

  factory DiscoverBusinessProfilesRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DiscoverBusinessProfilesRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DiscoverBusinessProfilesRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'grpc.business_profile'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'query')
    ..aOS(2, _omitFieldNames ? '' : 'businessType')
    ..aD(3, _omitFieldNames ? '' : 'latitude')
    ..aD(4, _omitFieldNames ? '' : 'longitude')
    ..aD(5, _omitFieldNames ? '' : 'radiusKm')
    ..aI(6, _omitFieldNames ? '' : 'limit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DiscoverBusinessProfilesRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DiscoverBusinessProfilesRequest copyWith(
          void Function(DiscoverBusinessProfilesRequest) updates) =>
      super.copyWith(
              (message) => updates(message as DiscoverBusinessProfilesRequest))
          as DiscoverBusinessProfilesRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DiscoverBusinessProfilesRequest create() =>
      DiscoverBusinessProfilesRequest._();
  @$core.override
  DiscoverBusinessProfilesRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DiscoverBusinessProfilesRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DiscoverBusinessProfilesRequest>(
          create);
  static DiscoverBusinessProfilesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get query => $_getSZ(0);
  @$pb.TagNumber(1)
  set query($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasQuery() => $_has(0);
  @$pb.TagNumber(1)
  void clearQuery() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get businessType => $_getSZ(1);
  @$pb.TagNumber(2)
  set businessType($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasBusinessType() => $_has(1);
  @$pb.TagNumber(2)
  void clearBusinessType() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.double get latitude => $_getN(2);
  @$pb.TagNumber(3)
  set latitude($core.double value) => $_setDouble(2, value);
  @$pb.TagNumber(3)
  $core.bool hasLatitude() => $_has(2);
  @$pb.TagNumber(3)
  void clearLatitude() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.double get longitude => $_getN(3);
  @$pb.TagNumber(4)
  set longitude($core.double value) => $_setDouble(3, value);
  @$pb.TagNumber(4)
  $core.bool hasLongitude() => $_has(3);
  @$pb.TagNumber(4)
  void clearLongitude() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.double get radiusKm => $_getN(4);
  @$pb.TagNumber(5)
  set radiusKm($core.double value) => $_setDouble(4, value);
  @$pb.TagNumber(5)
  $core.bool hasRadiusKm() => $_has(4);
  @$pb.TagNumber(5)
  void clearRadiusKm() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.int get limit => $_getIZ(5);
  @$pb.TagNumber(6)
  set limit($core.int value) => $_setSignedInt32(5, value);
  @$pb.TagNumber(6)
  $core.bool hasLimit() => $_has(5);
  @$pb.TagNumber(6)
  void clearLimit() => $_clearField(6);
}

class DeleteBusinessProfileRequest extends $pb.GeneratedMessage {
  factory DeleteBusinessProfileRequest({
    $core.int? id,
  }) {
    final result = create();
    if (id != null) result.id = id;
    return result;
  }

  DeleteBusinessProfileRequest._();

  factory DeleteBusinessProfileRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DeleteBusinessProfileRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteBusinessProfileRequest',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'grpc.business_profile'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteBusinessProfileRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteBusinessProfileRequest copyWith(
          void Function(DeleteBusinessProfileRequest) updates) =>
      super.copyWith(
              (message) => updates(message as DeleteBusinessProfileRequest))
          as DeleteBusinessProfileRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DeleteBusinessProfileRequest create() =>
      DeleteBusinessProfileRequest._();
  @$core.override
  DeleteBusinessProfileRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DeleteBusinessProfileRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteBusinessProfileRequest>(create);
  static DeleteBusinessProfileRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

class DeleteBusinessProfileResponse extends $pb.GeneratedMessage {
  factory DeleteBusinessProfileResponse() => create();

  DeleteBusinessProfileResponse._();

  factory DeleteBusinessProfileResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DeleteBusinessProfileResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeleteBusinessProfileResponse',
      package: const $pb.PackageName(
          _omitMessageNames ? '' : 'grpc.business_profile'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteBusinessProfileResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeleteBusinessProfileResponse copyWith(
          void Function(DeleteBusinessProfileResponse) updates) =>
      super.copyWith(
              (message) => updates(message as DeleteBusinessProfileResponse))
          as DeleteBusinessProfileResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DeleteBusinessProfileResponse create() =>
      DeleteBusinessProfileResponse._();
  @$core.override
  DeleteBusinessProfileResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DeleteBusinessProfileResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeleteBusinessProfileResponse>(create);
  static DeleteBusinessProfileResponse? _defaultInstance;
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
