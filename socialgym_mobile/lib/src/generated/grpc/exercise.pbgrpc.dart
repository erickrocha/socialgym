// This is a generated file - do not edit.
//
// Generated from exercise.proto.

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

import 'exercise.pb.dart' as $0;

export 'exercise.pb.dart';

@$pb.GrpcServiceName('grpc.exercise.ExerciseService')
class ExerciseServiceClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  ExerciseServiceClient(super.channel, {super.options, super.interceptors});

  $grpc.ResponseFuture<$0.Exercise> getExercise(
    $0.ExerciseRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getExercise, request, options: options);
  }

  $grpc.ResponseFuture<$0.PaginatedExercise> getExercises(
    $0.ExerciseParams request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$getExercises, request, options: options);
  }

  $grpc.ResponseFuture<$0.Exercise> addExercise(
    $0.Exercise request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$addExercise, request, options: options);
  }

  $grpc.ResponseFuture<$0.Exercise> updateExercise(
    $0.Exercise request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$updateExercise, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> deleteExercise(
    $0.ExerciseRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$deleteExercise, request, options: options);
  }

  // method descriptors

  static final _$getExercise =
      $grpc.ClientMethod<$0.ExerciseRequest, $0.Exercise>(
          '/grpc.exercise.ExerciseService/GetExercise',
          ($0.ExerciseRequest value) => value.writeToBuffer(),
          $0.Exercise.fromBuffer);
  static final _$getExercises =
      $grpc.ClientMethod<$0.ExerciseParams, $0.PaginatedExercise>(
          '/grpc.exercise.ExerciseService/GetExercises',
          ($0.ExerciseParams value) => value.writeToBuffer(),
          $0.PaginatedExercise.fromBuffer);
  static final _$addExercise = $grpc.ClientMethod<$0.Exercise, $0.Exercise>(
      '/grpc.exercise.ExerciseService/AddExercise',
      ($0.Exercise value) => value.writeToBuffer(),
      $0.Exercise.fromBuffer);
  static final _$updateExercise = $grpc.ClientMethod<$0.Exercise, $0.Exercise>(
      '/grpc.exercise.ExerciseService/UpdateExercise',
      ($0.Exercise value) => value.writeToBuffer(),
      $0.Exercise.fromBuffer);
  static final _$deleteExercise =
      $grpc.ClientMethod<$0.ExerciseRequest, $1.Empty>(
          '/grpc.exercise.ExerciseService/DeleteExercise',
          ($0.ExerciseRequest value) => value.writeToBuffer(),
          $1.Empty.fromBuffer);
}

@$pb.GrpcServiceName('grpc.exercise.ExerciseService')
abstract class ExerciseServiceBase extends $grpc.Service {
  $core.String get $name => 'grpc.exercise.ExerciseService';

  ExerciseServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ExerciseRequest, $0.Exercise>(
        'GetExercise',
        getExercise_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.ExerciseRequest.fromBuffer(value),
        ($0.Exercise value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ExerciseParams, $0.PaginatedExercise>(
        'GetExercises',
        getExercises_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.ExerciseParams.fromBuffer(value),
        ($0.PaginatedExercise value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.Exercise, $0.Exercise>(
        'AddExercise',
        addExercise_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.Exercise.fromBuffer(value),
        ($0.Exercise value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.Exercise, $0.Exercise>(
        'UpdateExercise',
        updateExercise_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.Exercise.fromBuffer(value),
        ($0.Exercise value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ExerciseRequest, $1.Empty>(
        'DeleteExercise',
        deleteExercise_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.ExerciseRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
  }

  $async.Future<$0.Exercise> getExercise_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ExerciseRequest> $request) async {
    return getExercise($call, await $request);
  }

  $async.Future<$0.Exercise> getExercise(
      $grpc.ServiceCall call, $0.ExerciseRequest request);

  $async.Future<$0.PaginatedExercise> getExercises_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ExerciseParams> $request) async {
    return getExercises($call, await $request);
  }

  $async.Future<$0.PaginatedExercise> getExercises(
      $grpc.ServiceCall call, $0.ExerciseParams request);

  $async.Future<$0.Exercise> addExercise_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.Exercise> $request) async {
    return addExercise($call, await $request);
  }

  $async.Future<$0.Exercise> addExercise(
      $grpc.ServiceCall call, $0.Exercise request);

  $async.Future<$0.Exercise> updateExercise_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.Exercise> $request) async {
    return updateExercise($call, await $request);
  }

  $async.Future<$0.Exercise> updateExercise(
      $grpc.ServiceCall call, $0.Exercise request);

  $async.Future<$1.Empty> deleteExercise_Pre($grpc.ServiceCall $call,
      $async.Future<$0.ExerciseRequest> $request) async {
    return deleteExercise($call, await $request);
  }

  $async.Future<$1.Empty> deleteExercise(
      $grpc.ServiceCall call, $0.ExerciseRequest request);
}
