// This is a generated file - do not edit.
//
// Generated from timeline/feed.proto.

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

import 'feed.pb.dart' as $0;

export 'feed.pb.dart';

@$pb.GrpcServiceName('grpc.timeline.FeedService')
class FeedServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  FeedServiceClient(super.channel, {super.options, super.interceptors});

  /// The caller's feed: own posts and accepted friends' posts, 20 per zero-based page.
  $grpc.ResponseFuture<$0.FeedResponse> getFeed(
    $0.GetFeedRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getFeed, request, options: options);
  }

  /// The posts of a Business Profile. Any other uuid is NOT_FOUND.
  $grpc.ResponseFuture<$0.FeedResponse> getFeedByAuthor(
    $0.GetFeedByAuthorRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getFeedByAuthor, request, options: options);
  }

  // method descriptors

  static final _$getFeed =
      $grpc.ClientMethod<$0.GetFeedRequest, $0.FeedResponse>(
          '/grpc.timeline.FeedService/GetFeed',
          ($0.GetFeedRequest value) => value.writeToBuffer(),
          $0.FeedResponse.fromBuffer);
  static final _$getFeedByAuthor =
      $grpc.ClientMethod<$0.GetFeedByAuthorRequest, $0.FeedResponse>(
          '/grpc.timeline.FeedService/GetFeedByAuthor',
          ($0.GetFeedByAuthorRequest value) => value.writeToBuffer(),
          $0.FeedResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.timeline.FeedService')
abstract class FeedServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.timeline.FeedService';

  FeedServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.GetFeedRequest, $0.FeedResponse>(
        'GetFeed',
        getFeed_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.GetFeedRequest.fromBuffer(value),
        ($0.FeedResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetFeedByAuthorRequest, $0.FeedResponse>(
        'GetFeedByAuthor',
        getFeedByAuthor_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetFeedByAuthorRequest.fromBuffer(value),
        ($0.FeedResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.FeedResponse> getFeed_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetFeedRequest> $request) async {
    return getFeed($call, await $request);
  }

  $async.Future<$0.FeedResponse> getFeed(
      $grpc.ServiceCall call, $0.GetFeedRequest request);

  $async.Future<$0.FeedResponse> getFeedByAuthor_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetFeedByAuthorRequest> $request) async {
    return getFeedByAuthor($call, await $request);
  }

  $async.Future<$0.FeedResponse> getFeedByAuthor(
      $grpc.ServiceCall call, $0.GetFeedByAuthorRequest request);
}
