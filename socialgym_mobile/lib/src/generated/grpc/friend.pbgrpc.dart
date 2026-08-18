// This is a generated file - do not edit.
//
// Generated from friend.proto.

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

import 'friend.pb.dart' as $0;

export 'friend.pb.dart';

@$pb.GrpcServiceName('grpc.friend.FriendService')
class FriendServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  FriendServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.FriendsResponse> getFriends(
    $0.FriendsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getFriends, request, options: options);
  }

  $grpc.ResponseFuture<$0.FriendPageResponse> getFriendPage(
    $0.FriendPageRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getFriendPage, request, options: options);
  }

  // method descriptors

  static final _$getFriends =
      $grpc.ClientMethod<$0.FriendsRequest, $0.FriendsResponse>(
          '/grpc.friend.FriendService/GetFriends',
          ($0.FriendsRequest value) => value.writeToBuffer(),
          $0.FriendsResponse.fromBuffer);
  static final _$getFriendPage =
      $grpc.ClientMethod<$0.FriendPageRequest, $0.FriendPageResponse>(
          '/grpc.friend.FriendService/GetFriendPage',
          ($0.FriendPageRequest value) => value.writeToBuffer(),
          $0.FriendPageResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.friend.FriendService')
abstract class FriendServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.friend.FriendService';

  FriendServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.FriendsRequest, $0.FriendsResponse>(
        'GetFriends',
        getFriends_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.FriendsRequest.fromBuffer(value),
        ($0.FriendsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.FriendPageRequest, $0.FriendPageResponse>(
        'GetFriendPage',
        getFriendPage_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.FriendPageRequest.fromBuffer(value),
        ($0.FriendPageResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.FriendsResponse> getFriends_Pre($grpc.ServiceCall $call,
      $async.Future<$0.FriendsRequest> $request) async {
    return getFriends($call, await $request);
  }

  $async.Future<$0.FriendsResponse> getFriends(
      $grpc.ServiceCall call, $0.FriendsRequest request);

  $async.Future<$0.FriendPageResponse> getFriendPage_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.FriendPageRequest> $request) async {
    return getFriendPage($call, await $request);
  }

  $async.Future<$0.FriendPageResponse> getFriendPage(
      $grpc.ServiceCall call, $0.FriendPageRequest request);
}
