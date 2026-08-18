// This is a generated file - do not edit.
//
// Generated from settings.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class SettingIdRequest extends $pb.GeneratedMessage {
  factory SettingIdRequest({
    $core.int? id,
    $core.String? uuid,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    return result;
  }

  SettingIdRequest._();

  factory SettingIdRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SettingIdRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SettingIdRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.settings'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingIdRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingIdRequest copyWith(void Function(SettingIdRequest) updates) =>
      super.copyWith((message) => updates(message as SettingIdRequest))
          as SettingIdRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SettingIdRequest create() => SettingIdRequest._();
  @$core.override
  SettingIdRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SettingIdRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SettingIdRequest>(create);
  static SettingIdRequest? _defaultInstance;

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

class SettingOwnerIdRequest extends $pb.GeneratedMessage {
  factory SettingOwnerIdRequest({
    $core.int? ownerId,
    $core.String? ownerUuid,
  }) {
    final result = create();
    if (ownerId != null) result.ownerId = ownerId;
    if (ownerUuid != null) result.ownerUuid = ownerUuid;
    return result;
  }

  SettingOwnerIdRequest._();

  factory SettingOwnerIdRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SettingOwnerIdRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SettingOwnerIdRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.settings'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'ownerId')
    ..aOS(2, _omitFieldNames ? '' : 'ownerUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingOwnerIdRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingOwnerIdRequest copyWith(
          void Function(SettingOwnerIdRequest) updates) =>
      super.copyWith((message) => updates(message as SettingOwnerIdRequest))
          as SettingOwnerIdRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SettingOwnerIdRequest create() => SettingOwnerIdRequest._();
  @$core.override
  SettingOwnerIdRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SettingOwnerIdRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SettingOwnerIdRequest>(create);
  static SettingOwnerIdRequest? _defaultInstance;

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

class SettingsResponse extends $pb.GeneratedMessage {
  factory SettingsResponse({
    $core.Iterable<Setting>? settings,
  }) {
    final result = create();
    if (settings != null) result.settings.addAll(settings);
    return result;
  }

  SettingsResponse._();

  factory SettingsResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SettingsResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SettingsResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.settings'),
      createEmptyInstance: create)
    ..pPM<Setting>(1, _omitFieldNames ? '' : 'settings',
        subBuilder: Setting.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingsResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SettingsResponse copyWith(void Function(SettingsResponse) updates) =>
      super.copyWith((message) => updates(message as SettingsResponse))
          as SettingsResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SettingsResponse create() => SettingsResponse._();
  @$core.override
  SettingsResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SettingsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SettingsResponse>(create);
  static SettingsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $pb.PbList<Setting> get settings => $_getList(0);
}

class Setting extends $pb.GeneratedMessage {
  factory Setting({
    $core.int? id,
    $core.String? uuid,
    $core.int? ownerId,
    $core.String? ownerUuid,
    $core.String? language,
    $core.String? theme,
    $core.bool? notificationsEnabled,
    $core.String? contextMenuPosition,
    $core.String? homePage,
    $core.String? createdAt,
    $core.String? updatedAt,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (uuid != null) result.uuid = uuid;
    if (ownerId != null) result.ownerId = ownerId;
    if (ownerUuid != null) result.ownerUuid = ownerUuid;
    if (language != null) result.language = language;
    if (theme != null) result.theme = theme;
    if (notificationsEnabled != null)
      result.notificationsEnabled = notificationsEnabled;
    if (contextMenuPosition != null)
      result.contextMenuPosition = contextMenuPosition;
    if (homePage != null) result.homePage = homePage;
    if (createdAt != null) result.createdAt = createdAt;
    if (updatedAt != null) result.updatedAt = updatedAt;
    return result;
  }

  Setting._();

  factory Setting.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Setting.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Setting',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.settings'),
      createEmptyInstance: create)
    ..aI(1, _omitFieldNames ? '' : 'id')
    ..aOS(2, _omitFieldNames ? '' : 'uuid')
    ..aI(3, _omitFieldNames ? '' : 'ownerId')
    ..aOS(4, _omitFieldNames ? '' : 'ownerUuid')
    ..aOS(5, _omitFieldNames ? '' : 'language')
    ..aOS(6, _omitFieldNames ? '' : 'theme')
    ..aOB(7, _omitFieldNames ? '' : 'notificationsEnabled')
    ..aOS(8, _omitFieldNames ? '' : 'contextMenuPosition')
    ..aOS(9, _omitFieldNames ? '' : 'homePage')
    ..aOS(10, _omitFieldNames ? '' : 'createdAt')
    ..aOS(11, _omitFieldNames ? '' : 'updatedAt')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Setting clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Setting copyWith(void Function(Setting) updates) =>
      super.copyWith((message) => updates(message as Setting)) as Setting;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Setting create() => Setting._();
  @$core.override
  Setting createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Setting getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Setting>(create);
  static Setting? _defaultInstance;

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
  $core.String get language => $_getSZ(4);
  @$pb.TagNumber(5)
  set language($core.String value) => $_setString(4, value);
  @$pb.TagNumber(5)
  $core.bool hasLanguage() => $_has(4);
  @$pb.TagNumber(5)
  void clearLanguage() => $_clearField(5);

  @$pb.TagNumber(6)
  $core.String get theme => $_getSZ(5);
  @$pb.TagNumber(6)
  set theme($core.String value) => $_setString(5, value);
  @$pb.TagNumber(6)
  $core.bool hasTheme() => $_has(5);
  @$pb.TagNumber(6)
  void clearTheme() => $_clearField(6);

  @$pb.TagNumber(7)
  $core.bool get notificationsEnabled => $_getBF(6);
  @$pb.TagNumber(7)
  set notificationsEnabled($core.bool value) => $_setBool(6, value);
  @$pb.TagNumber(7)
  $core.bool hasNotificationsEnabled() => $_has(6);
  @$pb.TagNumber(7)
  void clearNotificationsEnabled() => $_clearField(7);

  @$pb.TagNumber(8)
  $core.String get contextMenuPosition => $_getSZ(7);
  @$pb.TagNumber(8)
  set contextMenuPosition($core.String value) => $_setString(7, value);
  @$pb.TagNumber(8)
  $core.bool hasContextMenuPosition() => $_has(7);
  @$pb.TagNumber(8)
  void clearContextMenuPosition() => $_clearField(8);

  @$pb.TagNumber(9)
  $core.String get homePage => $_getSZ(8);
  @$pb.TagNumber(9)
  set homePage($core.String value) => $_setString(8, value);
  @$pb.TagNumber(9)
  $core.bool hasHomePage() => $_has(8);
  @$pb.TagNumber(9)
  void clearHomePage() => $_clearField(9);

  @$pb.TagNumber(10)
  $core.String get createdAt => $_getSZ(9);
  @$pb.TagNumber(10)
  set createdAt($core.String value) => $_setString(9, value);
  @$pb.TagNumber(10)
  $core.bool hasCreatedAt() => $_has(9);
  @$pb.TagNumber(10)
  void clearCreatedAt() => $_clearField(10);

  @$pb.TagNumber(11)
  $core.String get updatedAt => $_getSZ(10);
  @$pb.TagNumber(11)
  set updatedAt($core.String value) => $_setString(10, value);
  @$pb.TagNumber(11)
  $core.bool hasUpdatedAt() => $_has(10);
  @$pb.TagNumber(11)
  void clearUpdatedAt() => $_clearField(11);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
