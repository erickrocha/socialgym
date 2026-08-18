// This is a generated file - do not edit.
//
// Generated from settings.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'settings.pb.dart' as $0;

export 'settings.pb.dart';

@$pb.GrpcServiceName('grpc.settings.SettingsService')
class SettingsServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  SettingsServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.Setting> getById(
    $0.SettingIdRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getById, request, options: options);
  }

  $grpc.ResponseFuture<$0.Setting> persistSettings(
    $0.Setting request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$persistSettings, request, options: options);
  }

  $grpc.ResponseFuture<$0.Setting> getByUuid(
    $0.SettingIdRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getByUuid, request, options: options);
  }

  $grpc.ResponseFuture<$0.Setting> getByOwnerIds(
    $0.SettingOwnerIdRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getByOwnerIds, request, options: options);
  }

  // method descriptors

  static final _$getById = $grpc.ClientMethod<$0.SettingIdRequest, $0.Setting>(
      '/grpc.settings.SettingsService/GetById',
      ($0.SettingIdRequest value) => value.writeToBuffer(),
      $0.Setting.fromBuffer);
  static final _$persistSettings = $grpc.ClientMethod<$0.Setting, $0.Setting>(
      '/grpc.settings.SettingsService/PersistSettings',
      ($0.Setting value) => value.writeToBuffer(),
      $0.Setting.fromBuffer);
  static final _$getByUuid =
      $grpc.ClientMethod<$0.SettingIdRequest, $0.Setting>(
          '/grpc.settings.SettingsService/GetByUuid',
          ($0.SettingIdRequest value) => value.writeToBuffer(),
          $0.Setting.fromBuffer);
  static final _$getByOwnerIds =
      $grpc.ClientMethod<$0.SettingOwnerIdRequest, $0.Setting>(
          '/grpc.settings.SettingsService/GetByOwnerIds',
          ($0.SettingOwnerIdRequest value) => value.writeToBuffer(),
          $0.Setting.fromBuffer);
}

@$pb.GrpcServiceName('grpc.settings.SettingsService')
abstract class SettingsServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.settings.SettingsService';

  SettingsServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.SettingIdRequest, $0.Setting>(
        'GetById',
        getById_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.SettingIdRequest.fromBuffer(value),
        ($0.Setting value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.Setting, $0.Setting>(
        'PersistSettings',
        persistSettings_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.Setting.fromBuffer(value),
        ($0.Setting value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SettingIdRequest, $0.Setting>(
        'GetByUuid',
        getByUuid_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.SettingIdRequest.fromBuffer(value),
        ($0.Setting value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SettingOwnerIdRequest, $0.Setting>(
        'GetByOwnerIds',
        getByOwnerIds_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SettingOwnerIdRequest.fromBuffer(value),
        ($0.Setting value) => value.writeToBuffer()));
  }

  $async.Future<$0.Setting> getById_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SettingIdRequest> $request) async {
    return getById($call, await $request);
  }

  $async.Future<$0.Setting> getById(
      $grpc.ServiceCall call, $0.SettingIdRequest request);

  $async.Future<$0.Setting> persistSettings_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.Setting> $request) async {
    return persistSettings($call, await $request);
  }

  $async.Future<$0.Setting> persistSettings(
      $grpc.ServiceCall call, $0.Setting request);

  $async.Future<$0.Setting> getByUuid_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SettingIdRequest> $request) async {
    return getByUuid($call, await $request);
  }

  $async.Future<$0.Setting> getByUuid(
      $grpc.ServiceCall call, $0.SettingIdRequest request);

  $async.Future<$0.Setting> getByOwnerIds_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SettingOwnerIdRequest> $request) async {
    return getByOwnerIds($call, await $request);
  }

  $async.Future<$0.Setting> getByOwnerIds(
      $grpc.ServiceCall call, $0.SettingOwnerIdRequest request);
}
