// This is a generated file - do not edit.
//
// Generated from resource.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'country.pb.dart' as $1;
import 'province.pb.dart' as $3;
import 'settings.pb.dart' as $2;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

enum ResourceRequest_Identifier { userId, ownerUuid, notSet }

class ResourceRequest extends $pb.GeneratedMessage {
  factory ResourceRequest({
    $core.int? userId,
    $core.String? ownerUuid,
  }) {
    final result = create();
    if (userId != null) result.userId = userId;
    if (ownerUuid != null) result.ownerUuid = ownerUuid;
    return result;
  }

  ResourceRequest._();

  factory ResourceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ResourceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ResourceRequest_Identifier>
      _ResourceRequest_IdentifierByTag = {
    1: ResourceRequest_Identifier.userId,
    2: ResourceRequest_Identifier.ownerUuid,
    0: ResourceRequest_Identifier.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ResourceRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.resource'),
      createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aI(1, _omitFieldNames ? '' : 'userId', protoName: 'userId')
    ..aOS(2, _omitFieldNames ? '' : 'ownerUuid', protoName: 'ownerUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ResourceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ResourceRequest copyWith(void Function(ResourceRequest) updates) =>
      super.copyWith((message) => updates(message as ResourceRequest))
          as ResourceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ResourceRequest create() => ResourceRequest._();
  @$core.override
  ResourceRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ResourceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ResourceRequest>(create);
  static ResourceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  ResourceRequest_Identifier whichIdentifier() =>
      _ResourceRequest_IdentifierByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  @$pb.TagNumber(2)
  void clearIdentifier() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.int get userId => $_getIZ(0);
  @$pb.TagNumber(1)
  set userId($core.int value) => $_setSignedInt32(0, value);
  @$pb.TagNumber(1)
  $core.bool hasUserId() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserId() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get ownerUuid => $_getSZ(1);
  @$pb.TagNumber(2)
  set ownerUuid($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasOwnerUuid() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwnerUuid() => $_clearField(2);
}

class ResourceResponse extends $pb.GeneratedMessage {
  factory ResourceResponse({
    $core.Iterable<$1.Country>? countries,
    $2.Setting? setting,
    $core.Iterable<$3.Province>? provinces,
  }) {
    final result = create();
    if (countries != null) result.countries.addAll(countries);
    if (setting != null) result.setting = setting;
    if (provinces != null) result.provinces.addAll(provinces);
    return result;
  }

  ResourceResponse._();

  factory ResourceResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ResourceResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ResourceResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.resource'),
      createEmptyInstance: create)
    ..pPM<$1.Country>(1, _omitFieldNames ? '' : 'countries',
        subBuilder: $1.Country.create)
    ..aOM<$2.Setting>(2, _omitFieldNames ? '' : 'setting',
        subBuilder: $2.Setting.create)
    ..pPM<$3.Province>(3, _omitFieldNames ? '' : 'provinces',
        subBuilder: $3.Province.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ResourceResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ResourceResponse copyWith(void Function(ResourceResponse) updates) =>
      super.copyWith((message) => updates(message as ResourceResponse))
          as ResourceResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ResourceResponse create() => ResourceResponse._();
  @$core.override
  ResourceResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ResourceResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ResourceResponse>(create);
  static ResourceResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<$1.Country> get countries => $_getList(0);

  @$pb.TagNumber(2)
  $2.Setting get setting => $_getN(1);
  @$pb.TagNumber(2)
  set setting($2.Setting value) => $_setField(2, value);
  @$pb.TagNumber(2)
  $core.bool hasSetting() => $_has(1);
  @$pb.TagNumber(2)
  void clearSetting() => $_clearField(2);
  @$pb.TagNumber(2)
  $2.Setting ensureSetting() => $_ensure(1);

  @$pb.TagNumber(3)
  $pb.PbList<$3.Province> get provinces => $_getList(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
