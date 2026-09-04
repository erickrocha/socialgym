// This is a generated file - do not edit.
//
// Generated from workout.proto.

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
import 'package:protobuf/well_known_types/google/protobuf/empty.pb.dart' as $1;

import 'workout.pb.dart' as $0;

export 'workout.pb.dart';

@$pb.GrpcServiceName('grpc.workout.WorkoutService')
class WorkoutServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  WorkoutServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.Workout> getWorkout(
    $0.WorkoutRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getWorkout, request, options: options);
  }

  $grpc.ResponseFuture<$0.WorkoutResponse> getWorkoutsByOwner(
    $0.WorkoutListRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getWorkoutsByOwner, request, options: options);
  }

  $grpc.ResponseFuture<$0.Workout> addWorkout(
    $0.Workout request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$addWorkout, request, options: options);
  }

  $grpc.ResponseFuture<$0.Workout> updateWorkout(
    $0.Workout request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateWorkout, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteWorkout(
    $0.WorkoutRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteWorkout, request, options: options);
  }

  $grpc.ResponseFuture<$0.Workout> addExercisesToWorkout(
    $0.WorkoutExercisesRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$addExercisesToWorkout, request, options: options);
  }

  /// Team-member assignment lifecycle. Accept/Reject are for the assigned
  /// person; Cancel is for the assigning business profile.
  $grpc.ResponseFuture<$0.Workout> acceptWorkout(
    $0.WorkoutRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$acceptWorkout, request, options: options);
  }

  $grpc.ResponseFuture<$0.Workout> rejectWorkout(
    $0.WorkoutRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$rejectWorkout, request, options: options);
  }

  $grpc.ResponseFuture<$0.Workout> cancelWorkout(
    $0.WorkoutRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$cancelWorkout, request, options: options);
  }

  /// Workouts a business profile has assigned to its team members (every
  /// status). Only the acting profile itself may list them.
  $grpc.ResponseFuture<$0.WorkoutResponse> getWorkoutsAssignedByProfile(
    $0.AssignedWorkoutListRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getWorkoutsAssignedByProfile, request,
        options: options);
  }

  // method descriptors

  static final _$getWorkout = $grpc.ClientMethod<$0.WorkoutRequest, $0.Workout>(
      '/grpc.workout.WorkoutService/GetWorkout',
      ($0.WorkoutRequest value) => value.writeToBuffer(),
      $0.Workout.fromBuffer);
  static final _$getWorkoutsByOwner =
      $grpc.ClientMethod<$0.WorkoutListRequest, $0.WorkoutResponse>(
          '/grpc.workout.WorkoutService/GetWorkoutsByOwner',
          ($0.WorkoutListRequest value) => value.writeToBuffer(),
          $0.WorkoutResponse.fromBuffer);
  static final _$addWorkout = $grpc.ClientMethod<$0.Workout, $0.Workout>(
      '/grpc.workout.WorkoutService/AddWorkout',
      ($0.Workout value) => value.writeToBuffer(),
      $0.Workout.fromBuffer);
  static final _$updateWorkout = $grpc.ClientMethod<$0.Workout, $0.Workout>(
      '/grpc.workout.WorkoutService/UpdateWorkout',
      ($0.Workout value) => value.writeToBuffer(),
      $0.Workout.fromBuffer);
  static final _$deleteWorkout =
      $grpc.ClientMethod<$0.WorkoutRequest, $1.Empty>(
          '/grpc.workout.WorkoutService/DeleteWorkout',
          ($0.WorkoutRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
  static final _$addExercisesToWorkout =
      $grpc.ClientMethod<$0.WorkoutExercisesRequest, $0.Workout>(
          '/grpc.workout.WorkoutService/AddExercisesToWorkout',
          ($0.WorkoutExercisesRequest value) => value.writeToBuffer(),
          $0.Workout.fromBuffer);
  static final _$acceptWorkout =
      $grpc.ClientMethod<$0.WorkoutRequest, $0.Workout>(
          '/grpc.workout.WorkoutService/AcceptWorkout',
          ($0.WorkoutRequest value) => value.writeToBuffer(),
          $0.Workout.fromBuffer);
  static final _$rejectWorkout =
      $grpc.ClientMethod<$0.WorkoutRequest, $0.Workout>(
          '/grpc.workout.WorkoutService/RejectWorkout',
          ($0.WorkoutRequest value) => value.writeToBuffer(),
          $0.Workout.fromBuffer);
  static final _$cancelWorkout =
      $grpc.ClientMethod<$0.WorkoutRequest, $0.Workout>(
          '/grpc.workout.WorkoutService/CancelWorkout',
          ($0.WorkoutRequest value) => value.writeToBuffer(),
          $0.Workout.fromBuffer);
  static final _$getWorkoutsAssignedByProfile =
      $grpc.ClientMethod<$0.AssignedWorkoutListRequest, $0.WorkoutResponse>(
          '/grpc.workout.WorkoutService/GetWorkoutsAssignedByProfile',
          ($0.AssignedWorkoutListRequest value) => value.writeToBuffer(),
          $0.WorkoutResponse.fromBuffer);
}

@$pb.GrpcServiceName('grpc.workout.WorkoutService')
abstract class WorkoutServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.workout.WorkoutService';

  WorkoutServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.WorkoutRequest, $0.Workout>(
        'GetWorkout',
        getWorkout_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.WorkoutRequest.fromBuffer(value),
        ($0.Workout value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.WorkoutListRequest, $0.WorkoutResponse>(
        'GetWorkoutsByOwner',
        getWorkoutsByOwner_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.WorkoutListRequest.fromBuffer(value),
        ($0.WorkoutResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.Workout, $0.Workout>(
        'AddWorkout',
        addWorkout_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.Workout.fromBuffer(value),
        ($0.Workout value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.Workout, $0.Workout>(
        'UpdateWorkout',
        updateWorkout_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.Workout.fromBuffer(value),
        ($0.Workout value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.WorkoutRequest, $1.Empty>(
        'DeleteWorkout',
        deleteWorkout_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.WorkoutRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.WorkoutExercisesRequest, $0.Workout>(
        'AddExercisesToWorkout',
        addExercisesToWorkout_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.WorkoutExercisesRequest.fromBuffer(value),
        ($0.Workout value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.WorkoutRequest, $0.Workout>(
        'AcceptWorkout',
        acceptWorkout_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.WorkoutRequest.fromBuffer(value),
        ($0.Workout value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.WorkoutRequest, $0.Workout>(
        'RejectWorkout',
        rejectWorkout_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.WorkoutRequest.fromBuffer(value),
        ($0.Workout value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.WorkoutRequest, $0.Workout>(
        'CancelWorkout',
        cancelWorkout_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.WorkoutRequest.fromBuffer(value),
        ($0.Workout value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.AssignedWorkoutListRequest, $0.WorkoutResponse>(
            'GetWorkoutsAssignedByProfile',
            getWorkoutsAssignedByProfile_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.AssignedWorkoutListRequest.fromBuffer(value),
            ($0.WorkoutResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.Workout> getWorkout_Pre($grpc.ServiceCall $call,
      $async.Future<$0.WorkoutRequest> $request) async {
    return getWorkout($call, await $request);
  }

  $async.Future<$0.Workout> getWorkout(
      $grpc.ServiceCall call, $0.WorkoutRequest request);

  $async.Future<$0.WorkoutResponse> getWorkoutsByOwner_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.WorkoutListRequest> $request) async {
    return getWorkoutsByOwner($call, await $request);
  }

  $async.Future<$0.WorkoutResponse> getWorkoutsByOwner(
      $grpc.ServiceCall call, $0.WorkoutListRequest request);

  $async.Future<$0.Workout> addWorkout_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.Workout> $request) async {
    return addWorkout($call, await $request);
  }

  $async.Future<$0.Workout> addWorkout(
      $grpc.ServiceCall call, $0.Workout request);

  $async.Future<$0.Workout> updateWorkout_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.Workout> $request) async {
    return updateWorkout($call, await $request);
  }

  $async.Future<$0.Workout> updateWorkout(
      $grpc.ServiceCall call, $0.Workout request);

  $async.Future<$1.Empty> deleteWorkout_Pre($grpc.ServiceCall $call,
      $async.Future<$0.WorkoutRequest> $request) async {
    return deleteWorkout($call, await $request);
  }

  $async.Future<$1.Empty> deleteWorkout(
      $grpc.ServiceCall call, $0.WorkoutRequest request);

  $async.Future<$0.Workout> addExercisesToWorkout_Pre($grpc.ServiceCall $call,
      $async.Future<$0.WorkoutExercisesRequest> $request) async {
    return addExercisesToWorkout($call, await $request);
  }

  $async.Future<$0.Workout> addExercisesToWorkout(
      $grpc.ServiceCall call, $0.WorkoutExercisesRequest request);

  $async.Future<$0.Workout> acceptWorkout_Pre($grpc.ServiceCall $call,
      $async.Future<$0.WorkoutRequest> $request) async {
    return acceptWorkout($call, await $request);
  }

  $async.Future<$0.Workout> acceptWorkout(
      $grpc.ServiceCall call, $0.WorkoutRequest request);

  $async.Future<$0.Workout> rejectWorkout_Pre($grpc.ServiceCall $call,
      $async.Future<$0.WorkoutRequest> $request) async {
    return rejectWorkout($call, await $request);
  }

  $async.Future<$0.Workout> rejectWorkout(
      $grpc.ServiceCall call, $0.WorkoutRequest request);

  $async.Future<$0.Workout> cancelWorkout_Pre($grpc.ServiceCall $call,
      $async.Future<$0.WorkoutRequest> $request) async {
    return cancelWorkout($call, await $request);
  }

  $async.Future<$0.Workout> cancelWorkout(
      $grpc.ServiceCall call, $0.WorkoutRequest request);

  $async.Future<$0.WorkoutResponse> getWorkoutsAssignedByProfile_Pre(
      $grpc.ServiceCall $call,
      $async.Future<$0.AssignedWorkoutListRequest> $request) async {
    return getWorkoutsAssignedByProfile($call, await $request);
  }

  $async.Future<$0.WorkoutResponse> getWorkoutsAssignedByProfile(
      $grpc.ServiceCall call, $0.AssignedWorkoutListRequest request);
}
