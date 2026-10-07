// This is a generated file - do not edit.
//
// Generated from timeline/chat.proto.

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

import 'chat.pb.dart' as $0;

export 'chat.pb.dart';

/// Mirrors /timeline/api/chat (REST) and /timeline/api/chat/ws (WebSocket). The acting person is
/// always the one in the access token. Dates are ISO-8601 strings, as in the REST JSON.
@$pb.GrpcServiceName('grpc.timeline.ChatService')
class ChatServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  ChatServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListConversationsResponse> listConversations(
    $0.ListConversationsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listConversations, request, options: options);
  }

  /// Online subset of the requested people, limited to friends and people who share a
  /// conversation with the caller; anyone else is omitted exactly like an offline person.
  $grpc.ResponseFuture<$0.GetPresenceResponse> getPresence(
    $0.GetPresenceRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getPresence, request, options: options);
  }

  $grpc.ResponseFuture<$0.Conversation> createDirectConversation(
    $0.CreateDirectConversationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createDirectConversation, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.Conversation> createBusinessTeamGroup(
    $0.CreateBusinessTeamGroupRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createBusinessTeamGroup, request,
        options: options);
  }

  $grpc.ResponseFuture<$0.Conversation> createBusinessDirectConversation(
    $0.CreateBusinessDirectConversationRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createBusinessDirectConversation, request,
        options: options);
  }

  /// since_epoch_ms > 0 returns the messages newer than it (reconnect replay); otherwise a page.
  $grpc.ResponseFuture<$0.ListMessagesResponse> listMessages(
    $0.ListMessagesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listMessages, request, options: options);
  }

  $grpc.ResponseFuture<$0.Message> sendMessage(
    $0.SendMessageRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$sendMessage, request, options: options);
  }

  $grpc.ResponseFuture<$0.MarkConversationReadResponse> markConversationRead(
    $0.MarkConversationReadRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$markConversationRead, request, options: options);
  }

  /// Real-time stream replacing the WebSocket for the mobile app. Opening requires current Terms
  /// and Privacy consent (PERMISSION_DENIED otherwise); the server ends the stream when the access
  /// token expires (UNAUTHENTICATED) and the client reopens it with a fresh token.
  $grpc.ResponseStream<$0.ServerFrame> openStream(
    $async.Stream<$0.ClientFrame> request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(_$openStream, request, options: options);
  }

  // method descriptors

  static final _$listConversations = $grpc.ClientMethod<
          $0.ListConversationsRequest, $0.ListConversationsResponse>(
      '/grpc.timeline.ChatService/ListConversations',
      ($0.ListConversationsRequest value) => value.writeToBuffer(),
      $0.ListConversationsResponse.fromBuffer);
  static final _$getPresence =
      $grpc.ClientMethod<$0.GetPresenceRequest, $0.GetPresenceResponse>(
          '/grpc.timeline.ChatService/GetPresence',
          ($0.GetPresenceRequest value) => value.writeToBuffer(),
          $0.GetPresenceResponse.fromBuffer);
  static final _$createDirectConversation =
      $grpc.ClientMethod<$0.CreateDirectConversationRequest, $0.Conversation>(
          '/grpc.timeline.ChatService/CreateDirectConversation',
          ($0.CreateDirectConversationRequest value) => value.writeToBuffer(),
          $0.Conversation.fromBuffer);
  static final _$createBusinessTeamGroup =
      $grpc.ClientMethod<$0.CreateBusinessTeamGroupRequest, $0.Conversation>(
          '/grpc.timeline.ChatService/CreateBusinessTeamGroup',
          ($0.CreateBusinessTeamGroupRequest value) => value.writeToBuffer(),
          $0.Conversation.fromBuffer);
  static final _$createBusinessDirectConversation = $grpc.ClientMethod<
          $0.CreateBusinessDirectConversationRequest, $0.Conversation>(
      '/grpc.timeline.ChatService/CreateBusinessDirectConversation',
      ($0.CreateBusinessDirectConversationRequest value) =>
          value.writeToBuffer(),
      $0.Conversation.fromBuffer);
  static final _$listMessages =
      $grpc.ClientMethod<$0.ListMessagesRequest, $0.ListMessagesResponse>(
          '/grpc.timeline.ChatService/ListMessages',
          ($0.ListMessagesRequest value) => value.writeToBuffer(),
          $0.ListMessagesResponse.fromBuffer);
  static final _$sendMessage =
      $grpc.ClientMethod<$0.SendMessageRequest, $0.Message>(
          '/grpc.timeline.ChatService/SendMessage',
          ($0.SendMessageRequest value) => value.writeToBuffer(),
          $0.Message.fromBuffer);
  static final _$markConversationRead = $grpc.ClientMethod<
          $0.MarkConversationReadRequest, $0.MarkConversationReadResponse>(
      '/grpc.timeline.ChatService/MarkConversationRead',
      ($0.MarkConversationReadRequest value) => value.writeToBuffer(),
      $0.MarkConversationReadResponse.fromBuffer);
  static final _$openStream =
      $grpc.ClientMethod<$0.ClientFrame, $0.ServerFrame>(
          '/grpc.timeline.ChatService/OpenStream',
          ($0.ClientFrame value) => value.writeToBuffer(),
          $0.ServerFrame.fromBuffer);
}

@$pb.GrpcServiceName('grpc.timeline.ChatService')
abstract class ChatServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.timeline.ChatService';

  ChatServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListConversationsRequest,
            $0.ListConversationsResponse>(
        'ListConversations',
        listConversations_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListConversationsRequest.fromBuffer(value),
        ($0.ListConversationsResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.GetPresenceRequest, $0.GetPresenceResponse>(
            'GetPresence',
            getPresence_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.GetPresenceRequest.fromBuffer(value),
            ($0.GetPresenceResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateDirectConversationRequest,
            $0.Conversation>(
        'CreateDirectConversation',
        createDirectConversation_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateDirectConversationRequest.fromBuffer(value),
        ($0.Conversation value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.CreateBusinessTeamGroupRequest, $0.Conversation>(
            'CreateBusinessTeamGroup',
            createBusinessTeamGroup_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.CreateBusinessTeamGroupRequest.fromBuffer(value),
            ($0.Conversation value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CreateBusinessDirectConversationRequest,
            $0.Conversation>(
        'CreateBusinessDirectConversation',
        createBusinessDirectConversation_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.CreateBusinessDirectConversationRequest.fromBuffer(value),
        ($0.Conversation value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.ListMessagesRequest, $0.ListMessagesResponse>(
            'ListMessages',
            listMessages_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.ListMessagesRequest.fromBuffer(value),
            ($0.ListMessagesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SendMessageRequest, $0.Message>(
        'SendMessage',
        sendMessage_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SendMessageRequest.fromBuffer(value),
        ($0.Message value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MarkConversationReadRequest,
            $0.MarkConversationReadResponse>(
        'MarkConversationRead',
        markConversationRead_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MarkConversationReadRequest.fromBuffer(value),
        ($0.MarkConversationReadResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ClientFrame, $0.ServerFrame>(
        'OpenStream',
        openStream,
        true,
        true,
        ($core.List<$core.int> value) => $0.ClientFrame.fromBuffer(value),
        ($0.ServerFrame value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListConversationsResponse> listConversations_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListConversationsRequest> $request) async {
    return listConversations($call, await $request);
  }

  $async.Future<$0.ListConversationsResponse> listConversations(
      $grpc.ServiceCall call, $0.ListConversationsRequest request);

  $async.Future<$0.GetPresenceResponse> getPresence_Pre($grpc.ServiceCall $call,
      $async.Future<$0.GetPresenceRequest> $request) async {
    return getPresence($call, await $request);
  }

  $async.Future<$0.GetPresenceResponse> getPresence(
      $grpc.ServiceCall call, $0.GetPresenceRequest request);

  $async.Future<$0.Conversation> createDirectConversation_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateDirectConversationRequest> $request) async {
    return createDirectConversation($call, await $request);
  }

  $async.Future<$0.Conversation> createDirectConversation(
      $grpc.ServiceCall call, $0.CreateDirectConversationRequest request);

  $async.Future<$0.Conversation> createBusinessTeamGroup_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateBusinessTeamGroupRequest> $request) async {
    return createBusinessTeamGroup($call, await $request);
  }

  $async.Future<$0.Conversation> createBusinessTeamGroup(
      $grpc.ServiceCall call, $0.CreateBusinessTeamGroupRequest request);

  $async.Future<$0.Conversation> createBusinessDirectConversation_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateBusinessDirectConversationRequest>
          $request) async {
    return createBusinessDirectConversation($call, await $request);
  }

  $async.Future<$0.Conversation> createBusinessDirectConversation(
      $grpc.ServiceCall call,
      $0.CreateBusinessDirectConversationRequest request);

  $async.Future<$0.ListMessagesResponse> listMessages_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListMessagesRequest> $request) async {
    return listMessages($call, await $request);
  }

  $async.Future<$0.ListMessagesResponse> listMessages(
      $grpc.ServiceCall call, $0.ListMessagesRequest request);

  $async.Future<$0.Message> sendMessage_Pre($grpc.ServiceCall $call,
      $async.Future<$0.SendMessageRequest> $request) async {
    return sendMessage($call, await $request);
  }

  $async.Future<$0.Message> sendMessage(
      $grpc.ServiceCall call, $0.SendMessageRequest request);

  $async.Future<$0.MarkConversationReadResponse> markConversationRead_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.MarkConversationReadRequest> $request) async {
    return markConversationRead($call, await $request);
  }

  $async.Future<$0.MarkConversationReadResponse> markConversationRead(
      $grpc.ServiceCall call, $0.MarkConversationReadRequest request);

  $async.Stream<$0.ServerFrame> openStream(
      $grpc.ServiceCall call, $async.Stream<$0.ClientFrame> request);
}
