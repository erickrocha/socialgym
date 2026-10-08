// This is a generated file - do not edit.
//
// Generated from media.proto.

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

import 'media.pb.dart' as $0;

export 'media.pb.dart';

/// Upload links for post media (C-010): the gRPC twin of the REST /media/upload route. The file itself
/// never travels over gRPC; the client sends it over HTTPS to the returned pre-signed link.
@$pb.GrpcServiceName('grpc.media.MediaService')
class MediaServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  MediaServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.MediaUploadResponse> getPostMediaUploadUrl(
    $0.MediaUploadRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getPostMediaUploadUrl, request, options: options);
  }

  // method descriptors

  static final _$getPostMediaUploadUrl =
      $grpc.ClientMethod<$0.MediaUploadRequest, $0.MediaUploadResponse>(
          '/grpc.media.MediaService/GetPostMediaUploadUrl',
          ($0.MediaUploadRequest value) => value.writeToBuffer(),
          $0.MediaUploadResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.media.MediaService')
abstract class MediaServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.media.MediaService';

  MediaServiceBase() {
    $addMethod(
        $grpc.ServiceMethod<$0.MediaUploadRequest, $0.MediaUploadResponse>(
            'GetPostMediaUploadUrl',
            getPostMediaUploadUrl_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.MediaUploadRequest.fromBuffer(value),
            ($0.MediaUploadResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.MediaUploadResponse> getPostMediaUploadUrl_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.MediaUploadRequest> $request) async {
    return getPostMediaUploadUrl($call, await $request);
  }

  $async.Future<$0.MediaUploadResponse> getPostMediaUploadUrl(
      $grpc.ServiceCall call, $0.MediaUploadRequest request);
}
