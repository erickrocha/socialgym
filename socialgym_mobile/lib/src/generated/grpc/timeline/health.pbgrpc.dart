// This is a generated file - do not edit.
//
// Generated from timeline/health.proto.

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

import 'health.pb.dart' as $0;

export 'health.pb.dart';

/// Authenticated liveness probe of the timeline gRPC server: it proves the gateway route, TLS
/// and token validation work end to end. It touches no data.
@$pb.GrpcServiceName('grpc.timeline.HealthService')
class HealthServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  HealthServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.HealthResponse> check(
    $0.HealthRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$check, request, options: options);
  }

  // method descriptors

  static final _$check =
      $grpc.ClientMethod<$0.HealthRequest, $0.HealthResponse>(
          '/grpc.timeline.HealthService/Check',
          ($0.HealthRequest value) => value.writeToBuffer(),
          $0.HealthResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.timeline.HealthService')
abstract class HealthServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.timeline.HealthService';

  HealthServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.HealthRequest, $0.HealthResponse>(
        'Check',
        check_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.HealthRequest.fromBuffer(value),
        ($0.HealthResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.HealthResponse> check_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.HealthRequest> $request) async {
    return check($call, await $request);
  }

  $async.Future<$0.HealthResponse> check(
      $grpc.ServiceCall call, $0.HealthRequest request);
}
