// This is a generated file - do not edit.
//
// Generated from person.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'business_profile.pb.dart' as $4;
import 'person_address.pb.dart' as $2;
import 'person_info.pb.dart' as $1;
import 'user.pb.dart' as $3;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class ConsentStatusRequest extends $pb.GeneratedMessage {
  factory ConsentStatusRequest({
    $core.String? document,
  }) {
    final result = create();
    if (document != null) result.document = document;
    return result;
  }

  ConsentStatusRequest._();

  factory ConsentStatusRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ConsentStatusRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ConsentStatusRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'document')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConsentStatusRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConsentStatusRequest copyWith(void Function(ConsentStatusRequest) updates) =>
      super.copyWith((message) => updates(message as ConsentStatusRequest))
          as ConsentStatusRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConsentStatusRequest create() => ConsentStatusRequest._();
  @$core.override
  ConsentStatusRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ConsentStatusRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ConsentStatusRequest>(create);
  static ConsentStatusRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get document => $_getSZ(0);
  @$pb.TagNumber(1)
  set document($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDocument() => $_has(0);
  @$pb.TagNumber(1)
  void clearDocument() => $_clearField(1);
}

class ConsentStatusResponse extends $pb.GeneratedMessage {
  factory ConsentStatusResponse({
    $core.bool? active,
    $core.String? version,
  }) {
    final result = create();
    if (active != null) result.active = active;
    if (version != null) result.version = version;
    return result;
  }

  ConsentStatusResponse._();

  factory ConsentStatusResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ConsentStatusResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ConsentStatusResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'active')
    ..aOS(2, _omitFieldNames ? '' : 'version')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConsentStatusResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ConsentStatusResponse copyWith(
          void Function(ConsentStatusResponse) updates) =>
      super.copyWith((message) => updates(message as ConsentStatusResponse))
          as ConsentStatusResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ConsentStatusResponse create() => ConsentStatusResponse._();
  @$core.override
  ConsentStatusResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ConsentStatusResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ConsentStatusResponse>(create);
  static ConsentStatusResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get active => $_getBF(0);
  @$pb.TagNumber(1)
  set active($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasActive() => $_has(0);
  @$pb.TagNumber(1)
  void clearActive() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get version => $_getSZ(1);
  @$pb.TagNumber(2)
  set version($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasVersion() => $_has(1);
  @$pb.TagNumber(2)
  void clearVersion() => $_clearField(2);
}

class RoleStatusRequest extends $pb.GeneratedMessage {
  factory RoleStatusRequest({
    $core.String? role,
  }) {
    final result = create();
    if (role != null) result.role = role;
    return result;
  }

  RoleStatusRequest._();

  factory RoleStatusRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RoleStatusRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RoleStatusRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'role')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoleStatusRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoleStatusRequest copyWith(void Function(RoleStatusRequest) updates) =>
      super.copyWith((message) => updates(message as RoleStatusRequest))
          as RoleStatusRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoleStatusRequest create() => RoleStatusRequest._();
  @$core.override
  RoleStatusRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RoleStatusRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RoleStatusRequest>(create);
  static RoleStatusRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get role => $_getSZ(0);
  @$pb.TagNumber(1)
  set role($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRole() => $_has(0);
  @$pb.TagNumber(1)
  void clearRole() => $_clearField(1);
}

class RoleStatusResponse extends $pb.GeneratedMessage {
  factory RoleStatusResponse({
    $core.bool? active,
  }) {
    final result = create();
    if (active != null) result.active = active;
    return result;
  }

  RoleStatusResponse._();

  factory RoleStatusResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RoleStatusResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RoleStatusResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'active')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoleStatusResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RoleStatusResponse copyWith(void Function(RoleStatusResponse) updates) =>
      super.copyWith((message) => updates(message as RoleStatusResponse))
          as RoleStatusResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RoleStatusResponse create() => RoleStatusResponse._();
  @$core.override
  RoleStatusResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RoleStatusResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RoleStatusResponse>(create);
  static RoleStatusResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get active => $_getBF(0);
  @$pb.TagNumber(1)
  set active($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasActive() => $_has(0);
  @$pb.TagNumber(1)
  void clearActive() => $_clearField(1);
}

class GetMeRequest extends $pb.GeneratedMessage {
  factory GetMeRequest() => create();

  GetMeRequest._();

  factory GetMeRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GetMeRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetMeRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetMeRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GetMeRequest copyWith(void Function(GetMeRequest) updates) =>
      super.copyWith((message) => updates(message as GetMeRequest))
          as GetMeRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetMeRequest create() => GetMeRequest._();
  @$core.override
  GetMeRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GetMeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetMeRequest>(create);
  static GetMeRequest? _defaultInstance;
}

class RemovePersonAddressRequest extends $pb.GeneratedMessage {
  factory RemovePersonAddressRequest({
    $core.int? id,
    $core.String? uuid,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    return result;
  }

  RemovePersonAddressRequest._();

  factory RemovePersonAddressRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RemovePersonAddressRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemovePersonAddressRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemovePersonAddressRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemovePersonAddressRequest copyWith(
          void Function(RemovePersonAddressRequest) updates) =>
      super.copyWith(
              (message) => updates(message as RemovePersonAddressRequest))
          as RemovePersonAddressRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemovePersonAddressRequest create() => RemovePersonAddressRequest._();
  @$core.override
  RemovePersonAddressRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RemovePersonAddressRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemovePersonAddressRequest>(create);
  static RemovePersonAddressRequest? _defaultInstance;

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

class RemovePersonAddressResponse extends $pb.GeneratedMessage {
  factory RemovePersonAddressResponse({
    $core.bool? success,
  }) {
    final result = create();
    if (success != null) result.success = success;
    return result;
  }

  RemovePersonAddressResponse._();

  factory RemovePersonAddressResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RemovePersonAddressResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemovePersonAddressResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemovePersonAddressResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemovePersonAddressResponse copyWith(
          void Function(RemovePersonAddressResponse) updates) =>
      super.copyWith(
              (message) => updates(message as RemovePersonAddressResponse))
          as RemovePersonAddressResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemovePersonAddressResponse create() =>
      RemovePersonAddressResponse._();
  @$core.override
  RemovePersonAddressResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RemovePersonAddressResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemovePersonAddressResponse>(create);
  static RemovePersonAddressResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);
}

class PersonImageUploadRequest extends $pb.GeneratedMessage {
  factory PersonImageUploadRequest({
    $core.String? imageType,
    $core.String? format,
  }) {
    final result = create();
    if (imageType != null) result.imageType = imageType;
    if (format != null) result.format = format;
    return result;
  }

  PersonImageUploadRequest._();

  factory PersonImageUploadRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PersonImageUploadRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PersonImageUploadRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'imageType')
    ..aOS(2, _omitFieldNames ? '' : 'format')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonImageUploadRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonImageUploadRequest copyWith(
          void Function(PersonImageUploadRequest) updates) =>
      super.copyWith((message) => updates(message as PersonImageUploadRequest))
          as PersonImageUploadRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PersonImageUploadRequest create() => PersonImageUploadRequest._();
  @$core.override
  PersonImageUploadRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PersonImageUploadRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PersonImageUploadRequest>(create);
  static PersonImageUploadRequest? _defaultInstance;

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

class PersonImageUploadResponse extends $pb.GeneratedMessage {
  factory PersonImageUploadResponse({
    $core.String? url,
    $core.String? objectKey,
    $core.int? personId,
  }) {
    final result = create();
    if (url != null) result.url = url;
    if (objectKey != null) result.objectKey = objectKey;
    if (personId != null) result.personId = personId;
    return result;
  }

  PersonImageUploadResponse._();

  factory PersonImageUploadResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PersonImageUploadResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PersonImageUploadResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'url')
    ..aOS(2, _omitFieldNames ? '' : 'objectKey')
    ..aI(3, _omitFieldNames ? '' : 'personId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonImageUploadResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonImageUploadResponse copyWith(
          void Function(PersonImageUploadResponse) updates) =>
      super.copyWith((message) => updates(message as PersonImageUploadResponse))
          as PersonImageUploadResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PersonImageUploadResponse create() => PersonImageUploadResponse._();
  @$core.override
  PersonImageUploadResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PersonImageUploadResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PersonImageUploadResponse>(create);
  static PersonImageUploadResponse? _defaultInstance;

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
  $core.int get personId => $_getIZ(2);
  @$pb.TagNumber(3)
  set personId($core.int value) => $_setSignedInt32(2, value);
  @$pb.TagNumber(3)
  $core.bool hasPersonId() => $_has(2);
  @$pb.TagNumber(3)
  void clearPersonId() => $_clearField(3);
}

class PersonImageRequest extends $pb.GeneratedMessage {
  factory PersonImageRequest({
    $core.String? imageType,
  }) {
    final result = create();
    if (imageType != null) result.imageType = imageType;
    return result;
  }

  PersonImageRequest._();

  factory PersonImageRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PersonImageRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PersonImageRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'imageType')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonImageRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonImageRequest copyWith(void Function(PersonImageRequest) updates) =>
      super.copyWith((message) => updates(message as PersonImageRequest))
          as PersonImageRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PersonImageRequest create() => PersonImageRequest._();
  @$core.override
  PersonImageRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PersonImageRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PersonImageRequest>(create);
  static PersonImageRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get imageType => $_getSZ(0);
  @$pb.TagNumber(1)
  set imageType($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasImageType() => $_has(0);
  @$pb.TagNumber(1)
  void clearImageType() => $_clearField(1);
}

class DeletePersonImageResponse extends $pb.GeneratedMessage {
  factory DeletePersonImageResponse({
    $core.bool? success,
  }) {
    final result = create();
    if (success != null) result.success = success;
    return result;
  }

  DeletePersonImageResponse._();

  factory DeletePersonImageResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory DeletePersonImageResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeletePersonImageResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeletePersonImageResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  DeletePersonImageResponse copyWith(
          void Function(DeletePersonImageResponse) updates) =>
      super.copyWith((message) => updates(message as DeletePersonImageResponse))
          as DeletePersonImageResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DeletePersonImageResponse create() => DeletePersonImageResponse._();
  @$core.override
  DeletePersonImageResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static DeletePersonImageResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeletePersonImageResponse>(create);
  static DeletePersonImageResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool value) => $_setBool(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => $_clearField(1);
}

enum PersonParams_ParamIdentifier { id, uuid, notSet }

class PersonParams extends $pb.GeneratedMessage {
  factory PersonParams({
    $core.int? id,
    $core.String? uuid,
    $core.String? query,
    $core.int? limit,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    if (query != null) result.query = query;
    if (limit != null) result.limit = limit;
    return result;
  }

  PersonParams._();

  factory PersonParams.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PersonParams.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, PersonParams_ParamIdentifier>
      _PersonParams_ParamIdentifierByTag = {
    1: PersonParams_ParamIdentifier.id,
    2: PersonParams_ParamIdentifier.uuid,
    0: PersonParams_ParamIdentifier.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PersonParams',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..aOS(3, _omitFieldNames ? '' : 'query')
    ..aI(4, _omitFieldNames ? '' : 'limit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonParams clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonParams copyWith(void Function(PersonParams) updates) =>
      super.copyWith((message) => updates(message as PersonParams))
          as PersonParams;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PersonParams create() => PersonParams._();
  @$core.override
  PersonParams createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PersonParams getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PersonParams>(create);
  static PersonParams? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  PersonParams_ParamIdentifier whichParamIdentifier() =>
      _PersonParams_ParamIdentifierByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearParamIdentifier() => $_clearField($_whichOneof(0));

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
  $core.String get query => $_getSZ(2);
  @$pb.TagNumber(3)
  set query($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasQuery() => $_has(2);
  @$pb.TagNumber(3)
  void clearQuery() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.int get limit => $_getIZ(3);
  @$pb.TagNumber(4)
  set limit($core.int value) => $_setSignedInt32(3, value);
  @$pb.TagNumber(4)
  $core.bool hasLimit() => $_has(3);
  @$pb.TagNumber(4)
  void clearLimit() => $_clearField(4);
}

enum PersonIdRequest_Identifier { id, uuid, notSet }

class PersonIdRequest extends $pb.GeneratedMessage {
  factory PersonIdRequest({
    $core.int? id,
    $core.String? uuid,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    return result;
  }

  PersonIdRequest._();

  factory PersonIdRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PersonIdRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, PersonIdRequest_Identifier>
      _PersonIdRequest_IdentifierByTag = {
    1: PersonIdRequest_Identifier.id,
    2: PersonIdRequest_Identifier.uuid,
    0: PersonIdRequest_Identifier.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PersonIdRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonIdRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonIdRequest copyWith(void Function(PersonIdRequest) updates) =>
      super.copyWith((message) => updates(message as PersonIdRequest))
          as PersonIdRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PersonIdRequest create() => PersonIdRequest._();
  @$core.override
  PersonIdRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PersonIdRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PersonIdRequest>(create);
  static PersonIdRequest? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  PersonIdRequest_Identifier whichIdentifier() =>
      _PersonIdRequest_IdentifierByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearIdentifier() => $_clearField($_whichOneof(0));

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

class SearchMentionableFriendsRequest extends $pb.GeneratedMessage {
  factory SearchMentionableFriendsRequest({
    $core.int? personId,
    $core.String? query,
    $core.int? limit,
  }) {
    final result = create();
    if (personId != null) result.personId = personId;
    if (query != null) result.query = query;
    if (limit != null) result.limit = limit;
    return result;
  }

  SearchMentionableFriendsRequest._();

  factory SearchMentionableFriendsRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SearchMentionableFriendsRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SearchMentionableFriendsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'personId')
    ..aOS(2, _omitFieldNames ? '' : 'query')
    ..aI(3, _omitFieldNames ? '' : 'limit')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SearchMentionableFriendsRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SearchMentionableFriendsRequest copyWith(
          void Function(SearchMentionableFriendsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as SearchMentionableFriendsRequest))
          as SearchMentionableFriendsRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SearchMentionableFriendsRequest create() =>
      SearchMentionableFriendsRequest._();
  @$core.override
  SearchMentionableFriendsRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SearchMentionableFriendsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SearchMentionableFriendsRequest>(
          create);
  static SearchMentionableFriendsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get personId => $_getIZ(0);
  @$pb.TagNumber(1)
  set personId($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasPersonId() => $_has(0);
  @$pb.TagNumber(1)
  void clearPersonId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get query => $_getSZ(1);
  @$pb.TagNumber(2)
  set query($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasQuery() => $_has(1);
  @$pb.TagNumber(2)
  void clearQuery() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.int get limit => $_getIZ(2);
  @$pb.TagNumber(3)
  set limit($core.int value) => $_setSignedInt32(2, value);
  @$pb.TagNumber(3)
  $core.bool hasLimit() => $_has(2);
  @$pb.TagNumber(3)
  void clearLimit() => $_clearField(3);
}

class PeopleResponse extends $pb.GeneratedMessage {
  factory PeopleResponse({
    $core.Iterable<Person>? people,
  }) {
    final result = create();
    if (people != null) result.people.addAll(people);
    return result;
  }

  PeopleResponse._();

  factory PeopleResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PeopleResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PeopleResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..pPM<Person>(1, _omitFieldNames ? '' : 'people', subBuilder: Person.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PeopleResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PeopleResponse copyWith(void Function(PeopleResponse) updates) =>
      super.copyWith((message) => updates(message as PeopleResponse))
          as PeopleResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PeopleResponse create() => PeopleResponse._();
  @$core.override
  PeopleResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PeopleResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PeopleResponse>(create);
  static PeopleResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Person> get people => $_getList(0);
}

class PersonResponse extends $pb.GeneratedMessage {
  factory PersonResponse({
    Person? person,
  }) {
    final result = create();
    if (person != null) result.person = person;
    return result;
  }

  PersonResponse._();

  factory PersonResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PersonResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PersonResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..aOM<Person>(1, _omitFieldNames ? '' : 'person', subBuilder: Person.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PersonResponse copyWith(void Function(PersonResponse) updates) =>
      super.copyWith((message) => updates(message as PersonResponse))
          as PersonResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PersonResponse create() => PersonResponse._();
  @$core.override
  PersonResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PersonResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PersonResponse>(create);
  static PersonResponse? _defaultInstance;

  @$pb.TagNumber(1)
  Person get person => $_getN(0);
  @$pb.TagNumber(1)
  set person(Person value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasPerson() => $_has(0);
  @$pb.TagNumber(1)
  void clearPerson() => $_clearField(1);
  @$pb.TagNumber(1)
  Person ensurePerson() => $_ensure(0);
}

class Person extends $pb.GeneratedMessage {
  factory Person({
    $core.int? id,
    $core.String? uuid,
    $core.String? firstname,
    $core.String? surname,
    $core.String? dateOfBirth,
    $core.String? gender,
    $core.String? objectKey,
    $core.String? avatar,
    $core.String? cover,
    $3.User? user,
    $1.PersonInfo? personInfo,
    $core.Iterable<$2.PersonAddress>? addresses,
    $core.String? createdAt,
    $core.String? updatedAt,
    $core.Iterable<$4.BusinessProfile>? businessProfiles,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    if (firstname != null) result.firstname = firstname;
    if (surname != null) result.surname = surname;
    if (dateOfBirth != null) result.dateOfBirth = dateOfBirth;
    if (gender != null) result.gender = gender;
    if (objectKey != null) result.objectKey = objectKey;
    if (avatar != null) result.avatar = avatar;
    if (cover != null) result.cover = cover;
    if (user != null) result.user = user;
    if (personInfo != null) result.personInfo = personInfo;
    if (addresses != null) result.addresses.addAll(addresses);
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    if (businessProfiles != null)
      result.businessProfiles.addAll(businessProfiles);
    return result;
  }

  Person._();

  factory Person.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Person.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Person',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.person'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..aOS(3, _omitFieldNames ? '' : 'firstname')
    ..aOS(4, _omitFieldNames ? '' : 'surname')
    ..aOS(5, _omitFieldNames ? '' : 'dateOfBirth')
    ..aOS(6, _omitFieldNames ? '' : 'gender')
    ..aOS(7, _omitFieldNames ? '' : 'objectKey')
    ..aOS(8, _omitFieldNames ? '' : 'avatar')
    ..aOS(9, _omitFieldNames ? '' : 'cover')
    ..aOM<$3.User>(10, _omitFieldNames ? '' : 'user',
        subBuilder: $3.User.create)
    ..aOM<$1.PersonInfo>(11, _omitFieldNames ? '' : 'personInfo',
        subBuilder: $1.PersonInfo.create)
    ..pPM<$2.PersonAddress>(12, _omitFieldNames ? '' : 'addresses',
        subBuilder: $2.PersonAddress.create)
    ..aOS(13, _omitFieldNames ? '' : 'createdAt')
    ..aOS(14, _omitFieldNames ? '' : 'updatedAt')
    ..pPM<$4.BusinessProfile>(15, _omitFieldNames ? '' : 'businessProfiles',
        subBuilder: $4.BusinessProfile.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Person clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Person copyWith(void Function(Person) updates) =>
      super.copyWith((message) => updates(message as Person)) as Person;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Person create() => Person._();
  @$core.override
  Person createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Person getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Person>(create);
  static Person? _defaultInstance;

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
  $core.String get firstname => $_getSZ(2);
  @$pb.TagNumber(3)
  set firstname($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasFirstname() => $_has(2);
  @$pb.TagNumber(3)
  void clearFirstname() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get surname => $_getSZ(3);
  @$pb.TagNumber(4)
  set surname($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasSurname() => $_has(3);
  @$pb.TagNumber(4)
  void clearSurname() => $_clearField(4);

  @$pb.TagNumber(5)
  $core.String get dateOfBirth => $_getSZ(4);
  @$pb.TagNumber(5)
  set dateOfBirth($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasDateOfBirth() => $_has(4);
  @$pb.TagNumber(5)
  void clearDateOfBirth() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get gender => $_getSZ(5);
  @$pb.TagNumber(6)
  set gender($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasGender() => $_has(5);
  @$pb.TagNumber(6)
  void clearGender() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.String get objectKey => $_getSZ(6);
  @$pb.TagNumber(7)
  set objectKey($core.String value) => $_setString(6, value);
  @$pb.TagNumber(7)
  $core.bool hasObjectKey() => $_has(6);
  @$pb.TagNumber(7)
  void clearObjectKey() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get avatar => $_getSZ(7);
  @$pb.TagNumber(8)
  set avatar($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasAvatar() => $_has(7);
  @$pb.TagNumber(8)
  void clearAvatar() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get cover => $_getSZ(8);
  @$pb.TagNumber(9)
  set cover($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasCover() => $_has(8);
  @$pb.TagNumber(9)
  void clearCover() => $_clearField(9);

  @$pb.TagNumber(10)
  $3.User get user => $_getN(9);
  @$pb.TagNumber(10)
  set user($3.User value) => $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasUser() => $_has(9);
  @$pb.TagNumber(10)
  void clearUser() => $_clearField(10);
  @$pb.TagNumber(10)
  $3.User ensureUser() => $_ensure(9);

  @$pb.TagNumber(11)
  $1.PersonInfo get personInfo => $_getN(10);
  @$pb.TagNumber(11)
  set personInfo($1.PersonInfo value) => $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasPersonInfo() => $_has(10);
  @$pb.TagNumber(11)
  void clearPersonInfo() => $_clearField(11);
  @$pb.TagNumber(11)
  $1.PersonInfo ensurePersonInfo() => $_ensure(10);

  @$pb.TagNumber(12)
  $pb.PbList<$2.PersonAddress> get addresses => $_getList(11);

  @$pb.TagNumber(13)
  $core.String get createdAt => $_getSZ(12);
  @$pb.TagNumber(13)
  set createdAt($core.String value) => $_setString(12, value);
  @$pb.TagNumber(13)
  $core.bool hasCreatedAt() => $_has(12);
  @$pb.TagNumber(13)
  void clearCreatedAt() => $_clearField(13);

  @$pb.TagNumber(14)
  $core.String get updatedAt => $_getSZ(13);
  @$pb.TagNumber(14)
  set updatedAt($core.String value) => $_setString(13, value);
  @$pb.TagNumber(14)
  $core.bool hasUpdatedAt() => $_has(13);
  @$pb.TagNumber(14)
  void clearUpdatedAt() => $_clearField(14);

  @$pb.TagNumber(15)
  $pb.PbList<$4.BusinessProfile> get businessProfiles => $_getList(14);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
