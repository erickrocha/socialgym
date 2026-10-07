// This is a generated file - do not edit.
//
// Generated from timeline/push_device.proto.

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

import 'push_device.pb.dart' as $0;

export 'push_device.pb.dart';

@$pb.GrpcServiceName('grpc.timeline.PushDeviceService')
class PushDeviceServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  PushDeviceServiceClient(super.channel, {super.options, super.interceptors});

  /// The registration token is accepted here and never returned by any RPC.
  $grpc.ResponseFuture<$0.RegisterPushDeviceResponse> registerPushDevice(
    $0.RegisterPushDeviceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$registerPushDevice, request, options: options);
  }

  $grpc.ResponseFuture<$0.RemovePushDeviceResponse> removePushDevice(
    $0.RemovePushDeviceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$removePushDevice, request, options: options);
  }

  // method descriptors

  static final _$registerPushDevice = $grpc.ClientMethod<
          $0.RegisterPushDeviceRequest, $0.RegisterPushDeviceResponse>(
      '/grpc.timeline.PushDeviceService/RegisterPushDevice',
      ($0.RegisterPushDeviceRequest value) => value.writeToBuffer(),
      $0.RegisterPushDeviceResponse.fromBuffer);
  static final _$removePushDevice = $grpc.ClientMethod<
          $0.RemovePushDeviceRequest, $0.RemovePushDeviceResponse>(
      '/grpc.timeline.PushDeviceService/RemovePushDevice',
      ($0.RemovePushDeviceRequest value) => value.writeToBuffer(),
      $0.RemovePushDeviceResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.timeline.PushDeviceService')
abstract class PushDeviceServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.timeline.PushDeviceService';

  PushDeviceServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.RegisterPushDeviceRequest,
            $0.RegisterPushDeviceResponse>(
        'RegisterPushDevice',
        registerPushDevice_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RegisterPushDeviceRequest.fromBuffer(value),
        ($0.RegisterPushDeviceResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemovePushDeviceRequest,
            $0.RemovePushDeviceResponse>(
        'RemovePushDevice',
        removePushDevice_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemovePushDeviceRequest.fromBuffer(value),
        ($0.RemovePushDeviceResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.RegisterPushDeviceResponse> registerPushDevice_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RegisterPushDeviceRequest> $request) async {
    return registerPushDevice($call, await $request);
  }

  $async.Future<$0.RegisterPushDeviceResponse> registerPushDevice(
      $grpc.ServiceCall call, $0.RegisterPushDeviceRequest request);

  $async.Future<$0.RemovePushDeviceResponse> removePushDevice_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.RemovePushDeviceRequest> $request) async {
    return removePushDevice($call, await $request);
  }

  $async.Future<$0.RemovePushDeviceResponse> removePushDevice(
      $grpc.ServiceCall call, $0.RemovePushDeviceRequest request);
}
