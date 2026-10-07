// This is a generated file - do not edit.
//
// Generated from timeline/evolution.proto.

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

import 'evolution.pb.dart' as $0;

export 'evolution.pb.dart';

/// Requires current health_data consent to add; listing is limited to the caller's own check-ins.
@$pb.GrpcServiceName('grpc.timeline.EvolutionCheckInService')
class EvolutionCheckInServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  EvolutionCheckInServiceClient(super.channel,
      {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.EvolutionCheckIn> addEvolutionCheckIn(
    $0.AddEvolutionCheckInRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$addEvolutionCheckIn, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListEvolutionCheckInsResponse> listEvolutionCheckIns(
    $0.ListEvolutionCheckInsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listEvolutionCheckIns, request, options: options);
  }

  // method descriptors

  static final _$addEvolutionCheckIn =
      $grpc.ClientMethod<$0.AddEvolutionCheckInRequest, $0.EvolutionCheckIn>(
          '/grpc.timeline.EvolutionCheckInService/AddEvolutionCheckIn',
          ($0.AddEvolutionCheckInRequest value) => value.writeToBuffer(),
          $0.EvolutionCheckIn.fromBuffer);
  static final _$listEvolutionCheckIns = $grpc.ClientMethod<
          $0.ListEvolutionCheckInsRequest, $0.ListEvolutionCheckInsResponse>(
      '/grpc.timeline.EvolutionCheckInService/ListEvolutionCheckIns',
      ($0.ListEvolutionCheckInsRequest value) => value.writeToBuffer(),
      $0.ListEvolutionCheckInsResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.timeline.EvolutionCheckInService')
abstract class EvolutionCheckInServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.timeline.EvolutionCheckInService';

  EvolutionCheckInServiceBase() {
    $addMethod(
        $grpc.ServiceMethod<$0.AddEvolutionCheckInRequest, $0.EvolutionCheckIn>(
            'AddEvolutionCheckIn',
            addEvolutionCheckIn_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.AddEvolutionCheckInRequest.fromBuffer(value),
            ($0.EvolutionCheckIn value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListEvolutionCheckInsRequest,
            $0.ListEvolutionCheckInsResponse>(
        'ListEvolutionCheckIns',
        listEvolutionCheckIns_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListEvolutionCheckInsRequest.fromBuffer(value),
        ($0.ListEvolutionCheckInsResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.EvolutionCheckIn> addEvolutionCheckIn_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AddEvolutionCheckInRequest> $request) async {
    return addEvolutionCheckIn($call, await $request);
  }

  $async.Future<$0.EvolutionCheckIn> addEvolutionCheckIn(
      $grpc.ServiceCall call, $0.AddEvolutionCheckInRequest request);

  $async.Future<$0.ListEvolutionCheckInsResponse> listEvolutionCheckIns_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListEvolutionCheckInsRequest> $request) async {
    return listEvolutionCheckIns($call, await $request);
  }

  $async.Future<$0.ListEvolutionCheckInsResponse> listEvolutionCheckIns(
      $grpc.ServiceCall call, $0.ListEvolutionCheckInsRequest request);
}
