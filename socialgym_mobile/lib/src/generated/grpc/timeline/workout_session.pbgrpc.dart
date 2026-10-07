// This is a generated file - do not edit.
//
// Generated from timeline/workout_session.proto.

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

import 'workout_session.pb.dart' as $0;

export 'workout_session.pb.dart';

/// Mirrors /timeline/api/workout-sessions. A session belongs to the person in the token, whatever
/// the message says; another person's session is NOT_FOUND, like a missing one.
@$pb.GrpcServiceName('grpc.timeline.WorkoutSessionService')
class WorkoutSessionServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  WorkoutSessionServiceClient(super.channel,
      {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.WorkoutSession> createWorkoutSession(
    $0.CreateWorkoutSessionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$createWorkoutSession, request, options: options);
  }

  $grpc.ResponseFuture<$0.WorkoutSession> getWorkoutSession(
    $0.GetWorkoutSessionRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getWorkoutSession, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListWorkoutSessionsResponse> listWorkoutSessions(
    $0.ListWorkoutSessionsRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$listWorkoutSessions, request, options: options);
  }

  // method descriptors

  static final _$createWorkoutSession =
      $grpc.ClientMethod<$0.CreateWorkoutSessionRequest, $0.WorkoutSession>(
          '/grpc.timeline.WorkoutSessionService/CreateWorkoutSession',
          ($0.CreateWorkoutSessionRequest value) => value.writeToBuffer(),
          $0.WorkoutSession.fromBuffer);
  static final _$getWorkoutSession =
      $grpc.ClientMethod<$0.GetWorkoutSessionRequest, $0.WorkoutSession>(
          '/grpc.timeline.WorkoutSessionService/GetWorkoutSession',
          ($0.GetWorkoutSessionRequest value) => value.writeToBuffer(),
          $0.WorkoutSession.fromBuffer);
  static final _$listWorkoutSessions = $grpc.ClientMethod<
          $0.ListWorkoutSessionsRequest, $0.ListWorkoutSessionsResponse>(
      '/grpc.timeline.WorkoutSessionService/ListWorkoutSessions',
      ($0.ListWorkoutSessionsRequest value) => value.writeToBuffer(),
      $0.ListWorkoutSessionsResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.timeline.WorkoutSessionService')
abstract class WorkoutSessionServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.timeline.WorkoutSessionService';

  WorkoutSessionServiceBase() {
    $addMethod(
        $grpc.ServiceMethod<$0.CreateWorkoutSessionRequest, $0.WorkoutSession>(
            'CreateWorkoutSession',
            createWorkoutSession_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.CreateWorkoutSessionRequest.fromBuffer(value),
            ($0.WorkoutSession value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.GetWorkoutSessionRequest, $0.WorkoutSession>(
            'GetWorkoutSession',
            getWorkoutSession_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.GetWorkoutSessionRequest.fromBuffer(value),
            ($0.WorkoutSession value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ListWorkoutSessionsRequest,
            $0.ListWorkoutSessionsResponse>(
        'ListWorkoutSessions',
        listWorkoutSessions_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.ListWorkoutSessionsRequest.fromBuffer(value),
        ($0.ListWorkoutSessionsResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.WorkoutSession> createWorkoutSession_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.CreateWorkoutSessionRequest> $request) async {
    return createWorkoutSession($call, await $request);
  }

  $async.Future<$0.WorkoutSession> createWorkoutSession(
      $grpc.ServiceCall call, $0.CreateWorkoutSessionRequest request);

  $async.Future<$0.WorkoutSession> getWorkoutSession_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.GetWorkoutSessionRequest> $request) async {
    return getWorkoutSession($call, await $request);
  }

  $async.Future<$0.WorkoutSession> getWorkoutSession(
      $grpc.ServiceCall call, $0.GetWorkoutSessionRequest request);

  $async.Future<$0.ListWorkoutSessionsResponse> listWorkoutSessions_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.ListWorkoutSessionsRequest> $request) async {
    return listWorkoutSessions($call, await $request);
  }

  $async.Future<$0.ListWorkoutSessionsResponse> listWorkoutSessions(
      $grpc.ServiceCall call, $0.ListWorkoutSessionsRequest request);
}
