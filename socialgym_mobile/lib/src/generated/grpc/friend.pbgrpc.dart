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

  $grpc.ResponseFuture<$0.SearchFriendsResponse> searchFriends(
    $0.SearchFriendsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$searchFriends, request, options: options);
  }

  /// Mutations mirror the REST endpoints under /workout/api/friends. The caller
  /// (sender / accepter / canceller / remover) is always taken from the auth
  /// token, never the body — exactly as the REST controller uses
  /// current_user.person_id.
  $grpc.ResponseFuture<$0.Friend> sendFriendRequest(
    $0.FriendRequestRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$sendFriendRequest, request, options: options);
  }

  $grpc.ResponseFuture<$0.Friend> acceptFriendRequest(
    $0.FriendRequestRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$acceptFriendRequest, request, options: options);
  }

  $grpc.ResponseFuture<$0.Friend> denyFriendRequest(
    $0.FriendRequestRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$denyFriendRequest, request, options: options);
  }

  $grpc.ResponseFuture<$0.Friend> cancelFriendRequest(
    $0.FriendRequestRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$cancelFriendRequest, request, options: options);
  }

  $grpc.ResponseFuture<$0.RemoveFriendResponse> removeFriend(
    $0.FriendRequestRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$removeFriend, request, options: options);
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
  static final _$searchFriends =
      $grpc.ClientMethod<$0.SearchFriendsRequest, $0.SearchFriendsResponse>(
          '/grpc.friend.FriendService/SearchFriends',
          ($0.SearchFriendsRequest value) => value.writeToBuffer(),
          $0.SearchFriendsResponse.fromBuffer);
  static final _$sendFriendRequest =
      $grpc.ClientMethod<$0.FriendRequestRequest, $0.Friend>(
          '/grpc.friend.FriendService/SendFriendRequest',
          ($0.FriendRequestRequest value) => value.writeToBuffer(),
          $0.Friend.fromBuffer);
  static final _$acceptFriendRequest =
      $grpc.ClientMethod<$0.FriendRequestRequest, $0.Friend>(
          '/grpc.friend.FriendService/AcceptFriendRequest',
          ($0.FriendRequestRequest value) => value.writeToBuffer(),
          $0.Friend.fromBuffer);
  static final _$denyFriendRequest =
      $grpc.ClientMethod<$0.FriendRequestRequest, $0.Friend>(
          '/grpc.friend.FriendService/DenyFriendRequest',
          ($0.FriendRequestRequest value) => value.writeToBuffer(),
          $0.Friend.fromBuffer);
  static final _$cancelFriendRequest =
      $grpc.ClientMethod<$0.FriendRequestRequest, $0.Friend>(
          '/grpc.friend.FriendService/CancelFriendRequest',
          ($0.FriendRequestRequest value) => value.writeToBuffer(),
          $0.Friend.fromBuffer);
  static final _$removeFriend =
      $grpc.ClientMethod<$0.FriendRequestRequest, $0.RemoveFriendResponse>(
          '/grpc.friend.FriendService/RemoveFriend',
          ($0.FriendRequestRequest value) => value.writeToBuffer(),
          $0.RemoveFriendResponse.fromBuffer);
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
    $addMethod(
        $grpc.ServiceMethod<$0.SearchFriendsRequest, $0.SearchFriendsResponse>(
            'SearchFriends',
            searchFriends_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.SearchFriendsRequest.fromBuffer(value),
            ($0.SearchFriendsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.FriendRequestRequest, $0.Friend>(
        'SendFriendRequest',
        sendFriendRequest_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.FriendRequestRequest.fromBuffer(value),
        ($0.Friend value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.FriendRequestRequest, $0.Friend>(
        'AcceptFriendRequest',
        acceptFriendRequest_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.FriendRequestRequest.fromBuffer(value),
        ($0.Friend value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.FriendRequestRequest, $0.Friend>(
        'DenyFriendRequest',
        denyFriendRequest_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.FriendRequestRequest.fromBuffer(value),
        ($0.Friend value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.FriendRequestRequest, $0.Friend>(
        'CancelFriendRequest',
        cancelFriendRequest_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.FriendRequestRequest.fromBuffer(value),
        ($0.Friend value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.FriendRequestRequest, $0.RemoveFriendResponse>(
            'RemoveFriend',
            removeFriend_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.FriendRequestRequest.fromBuffer(value),
            ($0.RemoveFriendResponse value) => value.writeToBuffer()));
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

  $async.Future<$0.SearchFriendsResponse> searchFriends_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.SearchFriendsRequest> $request) async {
    return searchFriends($call, await $request);
  }

  $async.Future<$0.SearchFriendsResponse> searchFriends(
      $grpc.ServiceCall call, $0.SearchFriendsRequest request);

  $async.Future<$0.Friend> sendFriendRequest_Pre($grpc.ServiceCall $call,
      $async.Future<$0.FriendRequestRequest> $request) async {
    return sendFriendRequest($call, await $request);
  }

  $async.Future<$0.Friend> sendFriendRequest(
      $grpc.ServiceCall call, $0.FriendRequestRequest request);

  $async.Future<$0.Friend> acceptFriendRequest_Pre($grpc.ServiceCall $call,
      $async.Future<$0.FriendRequestRequest> $request) async {
    return acceptFriendRequest($call, await $request);
  }

  $async.Future<$0.Friend> acceptFriendRequest(
      $grpc.ServiceCall call, $0.FriendRequestRequest request);

  $async.Future<$0.Friend> denyFriendRequest_Pre($grpc.ServiceCall $call,
      $async.Future<$0.FriendRequestRequest> $request) async {
    return denyFriendRequest($call, await $request);
  }

  $async.Future<$0.Friend> denyFriendRequest(
      $grpc.ServiceCall call, $0.FriendRequestRequest request);

  $async.Future<$0.Friend> cancelFriendRequest_Pre($grpc.ServiceCall $call,
      $async.Future<$0.FriendRequestRequest> $request) async {
    return cancelFriendRequest($call, await $request);
  }

  $async.Future<$0.Friend> cancelFriendRequest(
      $grpc.ServiceCall call, $0.FriendRequestRequest request);

  $async.Future<$0.RemoveFriendResponse> removeFriend_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.FriendRequestRequest> $request) async {
    return removeFriend($call, await $request);
  }

  $async.Future<$0.RemoveFriendResponse> removeFriend(
      $grpc.ServiceCall call, $0.FriendRequestRequest request);
}
