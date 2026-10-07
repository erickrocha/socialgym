// This is a generated file - do not edit.
//
// Generated from timeline/push_device.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

class RegisterPushDeviceRequest extends $pb.GeneratedMessage {
  factory RegisterPushDeviceRequest({
    $core.String? deviceUuid,
    $core.String? platform,
    $core.String? registrationToken,
  }) {
    final result = create();
    if (deviceUuid != null) result.deviceUuid = deviceUuid;
    if (platform != null) result.platform = platform;
    if (registrationToken != null) result.registrationToken = registrationToken;
    return result;
  }

  RegisterPushDeviceRequest._();

  factory RegisterPushDeviceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RegisterPushDeviceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RegisterPushDeviceRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'deviceUuid')
    ..aOS(2, _omitFieldNames ? '' : 'platform')
    ..aOS(3, _omitFieldNames ? '' : 'registrationToken')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RegisterPushDeviceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RegisterPushDeviceRequest copyWith(
          void Function(RegisterPushDeviceRequest) updates) =>
      super.copyWith((message) => updates(message as RegisterPushDeviceRequest))
          as RegisterPushDeviceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RegisterPushDeviceRequest create() => RegisterPushDeviceRequest._();
  @$core.override
  RegisterPushDeviceRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RegisterPushDeviceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RegisterPushDeviceRequest>(create);
  static RegisterPushDeviceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get deviceUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set deviceUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDeviceUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearDeviceUuid() => $_clearField(1);

  /// android or ios.
  @$pb.TagNumber(2)
  $core.String get platform => $_getSZ(1);
  @$pb.TagNumber(2)
  set platform($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasPlatform() => $_has(1);
  @$pb.TagNumber(2)
  void clearPlatform() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get registrationToken => $_getSZ(2);
  @$pb.TagNumber(3)
  set registrationToken($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasRegistrationToken() => $_has(2);
  @$pb.TagNumber(3)
  void clearRegistrationToken() => $_clearField(3);
}

class RegisterPushDeviceResponse extends $pb.GeneratedMessage {
  factory RegisterPushDeviceResponse() => create();

  RegisterPushDeviceResponse._();

  factory RegisterPushDeviceResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RegisterPushDeviceResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RegisterPushDeviceResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RegisterPushDeviceResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RegisterPushDeviceResponse copyWith(
          void Function(RegisterPushDeviceResponse) updates) =>
      super.copyWith(
              (message) => updates(message as RegisterPushDeviceResponse))
          as RegisterPushDeviceResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RegisterPushDeviceResponse create() => RegisterPushDeviceResponse._();
  @$core.override
  RegisterPushDeviceResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RegisterPushDeviceResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RegisterPushDeviceResponse>(create);
  static RegisterPushDeviceResponse? _defaultInstance;
}

class RemovePushDeviceRequest extends $pb.GeneratedMessage {
  factory RemovePushDeviceRequest({
    $core.String? deviceUuid,
  }) {
    final result = create();
    if (deviceUuid != null) result.deviceUuid = deviceUuid;
    return result;
  }

  RemovePushDeviceRequest._();

  factory RemovePushDeviceRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RemovePushDeviceRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemovePushDeviceRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'deviceUuid')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemovePushDeviceRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemovePushDeviceRequest copyWith(
          void Function(RemovePushDeviceRequest) updates) =>
      super.copyWith((message) => updates(message as RemovePushDeviceRequest))
          as RemovePushDeviceRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemovePushDeviceRequest create() => RemovePushDeviceRequest._();
  @$core.override
  RemovePushDeviceRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RemovePushDeviceRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemovePushDeviceRequest>(create);
  static RemovePushDeviceRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get deviceUuid => $_getSZ(0);
  @$pb.TagNumber(1)
  set deviceUuid($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasDeviceUuid() => $_has(0);
  @$pb.TagNumber(1)
  void clearDeviceUuid() => $_clearField(1);
}

class RemovePushDeviceResponse extends $pb.GeneratedMessage {
  factory RemovePushDeviceResponse() => create();

  RemovePushDeviceResponse._();

  factory RemovePushDeviceResponse.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory RemovePushDeviceResponse.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'RemovePushDeviceResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'grpc.timeline'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemovePushDeviceResponse clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  RemovePushDeviceResponse copyWith(
          void Function(RemovePushDeviceResponse) updates) =>
      super.copyWith((message) => updates(message as RemovePushDeviceResponse))
          as RemovePushDeviceResponse;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RemovePushDeviceResponse create() => RemovePushDeviceResponse._();
  @$core.override
  RemovePushDeviceResponse createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static RemovePushDeviceResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RemovePushDeviceResponse>(create);
  static RemovePushDeviceResponse? _defaultInstance;
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
