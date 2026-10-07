// This is a generated file - do not edit.
//
// Generated from timeline/post.proto.

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

import 'post.pb.dart' as $0;

export 'post.pb.dart';

@$pb.GrpcServiceName('grpc.timeline.PostService')
class PostServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  PostServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.Post> createPost(
    $0.CreatePostRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createPost, request, options: options);
  }

  $grpc.ResponseFuture<$0.DeletePostResponse> deletePost(
    $0.DeletePostRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deletePost, request, options: options);
  }

  $grpc.ResponseFuture<$0.Post> addComment(
    $0.AddCommentRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$addComment, request, options: options);
  }

  $grpc.ResponseFuture<$0.Post> addReaction(
    $0.AddReactionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$addReaction, request, options: options);
  }

  $grpc.ResponseFuture<$0.Post> removeReaction(
    $0.RemoveReactionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$removeReaction, request, options: options);
  }

  // method descriptors

  static final _$createPost = $grpc.ClientMethod<$0.CreatePostRequest, $0.Post>(
      '/grpc.timeline.PostService/CreatePost',
      ($0.CreatePostRequest value) => value.writeToBuffer(),
      $0.Post.fromBuffer);
  static final _$deletePost =
      $grpc.ClientMethod<$0.DeletePostRequest, $0.DeletePostResponse>(
          '/grpc.timeline.PostService/DeletePost',
          ($0.DeletePostRequest value) => value.writeToBuffer(),
          $0.DeletePostResponse.fromBuffer);
  static final _$addComment = $grpc.ClientMethod<$0.AddCommentRequest, $0.Post>(
      '/grpc.timeline.PostService/AddComment',
      ($0.AddCommentRequest value) => value.writeToBuffer(),
      $0.Post.fromBuffer);
  static final _$addReaction =
      $grpc.ClientMethod<$0.AddReactionRequest, $0.Post>(
          '/grpc.timeline.PostService/AddReaction',
          ($0.AddReactionRequest value) => value.writeToBuffer(),
          $0.Post.fromBuffer);
  static final _$removeReaction =
      $grpc.ClientMethod<$0.RemoveReactionRequest, $0.Post>(
          '/grpc.timeline.PostService/RemoveReaction',
          ($0.RemoveReactionRequest value) => value.writeToBuffer(),
          $0.Post.fromBuffer);
}

@$pb.GrpcServiceName('grpc.timeline.PostService')
abstract class PostServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.timeline.PostService';

  PostServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.CreatePostRequest, $0.Post>(
        'CreatePost',
        createPost_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.CreatePostRequest.fromBuffer(value),
        ($0.Post value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.DeletePostRequest, $0.DeletePostResponse>(
        'DeletePost',
        deletePost_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.DeletePostRequest.fromBuffer(value),
        ($0.DeletePostResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AddCommentRequest, $0.Post>(
        'AddComment',
        addComment_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.AddCommentRequest.fromBuffer(value),
        ($0.Post value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AddReactionRequest, $0.Post>(
        'AddReaction',
        addReaction_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AddReactionRequest.fromBuffer(value),
        ($0.Post value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RemoveReactionRequest, $0.Post>(
        'RemoveReaction',
        removeReaction_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.RemoveReactionRequest.fromBuffer(value),
        ($0.Post value) => value.writeToBuffer()));
  }

  $async.Future<$0.Post> createPost_Pre($grpc.ServiceCall $call,
      $async.Future<$0.CreatePostRequest> $request) async {
    return createPost($call, await $request);
  }

  $async.Future<$0.Post> createPost(
      $grpc.ServiceCall call, $0.CreatePostRequest request);

  $async.Future<$0.DeletePostResponse> deletePost_Pre($grpc.ServiceCall $call,
      $async.Future<$0.DeletePostRequest> $request) async {
    return deletePost($call, await $request);
  }

  $async.Future<$0.DeletePostResponse> deletePost(
      $grpc.ServiceCall call, $0.DeletePostRequest request);

  $async.Future<$0.Post> addComment_Pre($grpc.ServiceCall $call,
      $async.Future<$0.AddCommentRequest> $request) async {
    return addComment($call, await $request);
  }

  $async.Future<$0.Post> addComment(
      $grpc.ServiceCall call, $0.AddCommentRequest request);

  $async.Future<$0.Post> addReaction_Pre($grpc.ServiceCall $call,
      $async.Future<$0.AddReactionRequest> $request) async {
    return addReaction($call, await $request);
  }

  $async.Future<$0.Post> addReaction(
      $grpc.ServiceCall call, $0.AddReactionRequest request);

  $async.Future<$0.Post> removeReaction_Pre($grpc.ServiceCall $call,
      $async.Future<$0.RemoveReactionRequest> $request) async {
    return removeReaction($call, await $request);
  }

  $async.Future<$0.Post> removeReaction(
      $grpc.ServiceCall call, $0.RemoveReactionRequest request);
}
