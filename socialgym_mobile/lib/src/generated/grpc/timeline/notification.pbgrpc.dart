// This is a generated file - do not edit.
//
// Generated from timeline/notification.proto.

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

import 'notification.pb.dart' as $0;

export 'notification.pb.dart';

/// Notifications of the person in the token; there is no owner field.
@$pb.GrpcServiceName('grpc.timeline.NotificationService')
class NotificationServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  NotificationServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.ListNotificationsResponse> listNotifications(
    $0.ListNotificationsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listNotifications, request, options: options);
  }

  $grpc.ResponseFuture<$0.MarkNotificationReadResponse> markNotificationRead(
    $0.MarkNotificationReadRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$markNotificationRead, request, options: options);
  }

  // method descriptors

  static final _$listNotifications = $grpc.ClientMethod<
          $0.ListNotificationsRequest, $0.ListNotificationsResponse>(
      '/grpc.timeline.NotificationService/ListNotifications',
      ($0.ListNotificationsRequest value) => value.writeToBuffer(),
      $0.ListNotificationsResponse.fromBuffer);
  static final _$markNotificationRead = $grpc.ClientMethod<
          $0.MarkNotificationReadRequest, $0.MarkNotificationReadResponse>(
      '/grpc.timeline.NotificationService/MarkNotificationRead',
      ($0.MarkNotificationReadRequest value) => value.writeToBuffer(),
      $0.MarkNotificationReadResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.timeline.NotificationService')
abstract class NotificationServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.timeline.NotificationService';

  NotificationServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ListNotificationsRequest,
            $0.ListNotificationsResponse>(
        'ListNotifications',
        listNotifications_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListNotificationsRequest.fromBuffer(value),
        ($0.ListNotificationsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.MarkNotificationReadRequest,
            $0.MarkNotificationReadResponse>(
        'MarkNotificationRead',
        markNotificationRead_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.MarkNotificationReadRequest.fromBuffer(value),
        ($0.MarkNotificationReadResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.ListNotificationsResponse> listNotifications_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListNotificationsRequest> $request) async {
    return listNotifications($call, await $request);
  }

  $async.Future<$0.ListNotificationsResponse> listNotifications(
      $grpc.ServiceCall call, $0.ListNotificationsRequest request);

  $async.Future<$0.MarkNotificationReadResponse> markNotificationRead_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.MarkNotificationReadRequest> $request) async {
    return markNotificationRead($call, await $request);
  }

  $async.Future<$0.MarkNotificationReadResponse> markNotificationRead(
      $grpc.ServiceCall call, $0.MarkNotificationReadRequest request);
}
