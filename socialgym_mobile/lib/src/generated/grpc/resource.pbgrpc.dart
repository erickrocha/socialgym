// This is a generated file - do not edit.
//
// Generated from resource.proto.

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

import 'resource.pb.dart' as $0;

export 'resource.pb.dart';

@$pb.GrpcServiceName('grpc.resource.ResourceService')
class ResourceServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  ResourceServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ResourceResponse> getResource(
    $0.ResourceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getResource, request, options: options);
  }

  // method descriptors

  static final _$getResource =
      $grpc.ClientMethod<$0.ResourceRequest, $0.ResourceResponse>(
          '/grpc.resource.ResourceService/GetResource',
          ($0.ResourceRequest value) => value.writeToBuffer(),
          $0.ResourceResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.resource.ResourceService')
abstract class ResourceServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.resource.ResourceService';

  ResourceServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ResourceRequest, $0.ResourceResponse>(
        'GetResource',
        getResource_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.ResourceRequest.fromBuffer(value),
        ($0.ResourceResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.ResourceResponse> getResource_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ResourceRequest> $request) async {
    return getResource($call, await $request);
  }

  $async.Future<$0.ResourceResponse> getResource(
      $grpc.ServiceCall call, $0.ResourceRequest request);
}
