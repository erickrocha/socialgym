import 'package:grpc/grpc.dart' as grpc;
import 'package:lapidation_mobile/commons/exercise_mapper.dart';
import 'package:lapidation_mobile/commons/workout_mapper.dart';
import 'package:lapidation_mobile/models/exercise.dart';
import 'package:lapidation_mobile/models/workout.dart';
import 'package:lapidation_mobile/services/grpc/grpc_channel_factory.dart';
import 'package:lapidation_mobile/src/generated/grpc/workout.pbgrpc.dart' as $workout;

import '../../config/api_config.dart';

class GrpcWorkoutService {
  GrpcWorkoutService._();

  static $workout.WorkoutServiceClient? _client;

  static Future<Workout> getWorkoutsById({required int id}) async {
    final response = await _ensureClient().getWorkout(
      $workout.WorkoutRequest()..id = id,
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return WorkoutMapper().fromProto(response);
  }

  static $workout.WorkoutServiceClient _ensureClient() {
    if (_client != null) return _client!;
    final channel = GrpcChannelFactory.channelFor(
      host: ApiConfig.grpcHost,
      port: ApiConfig.grpcPort,
      authority: ApiConfig.grpcAuthority,
    );
    _client = $workout.WorkoutServiceClient(
        channel, interceptors: GrpcChannelFactory.interceptors);
    return _client!;
  }

  static Future<Workout> getWorkoutByUuid({required String uuid}) async {
    final response = await _ensureClient().getWorkout(
      $workout.WorkoutRequest()..uuid = uuid,
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return WorkoutMapper().fromProto(response);
  }

  static Future<List<Workout>> getWorkoutsByOwnerUuid(
      {required String ownerUuid}) async {
    final response = await _ensureClient().getWorkoutsByOwner(
      $workout.WorkoutListRequest()..ownerUuid = ownerUuid,
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return WorkoutMapper().fromProtoList(response.workouts);
  }

  static Future<Workout> createWorkout({required Workout workout}) async {
    final mapper = WorkoutMapper();
    final response = await _ensureClient().addWorkout(
      $workout.Workout()..mergeFromMessage(mapper.toProto(workout)),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return mapper.fromProto(response);
  }

  static Future<Workout> update({required Workout workout}) async {
    final mapper = WorkoutMapper();
    final response = await _ensureClient().updateWorkout(
      $workout.Workout()..mergeFromMessage(mapper.toProto(workout)),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return mapper.fromProto(response);
  }

  static Future<Workout> addExercisesToWorkout({required String workoutUuid, required List<Exercise> exercises}) async {
    final response = await _ensureClient().addExercisesToWorkout(
      $workout.WorkoutExercisesRequest()
        ..workoutUuid = workoutUuid
        ..exercises.addAll(exercises.map((e) => ExerciseMapper().toProto(e))),
      options: grpc.CallOptions(timeout: const Duration(seconds: 5)),
    );
    return WorkoutMapper().fromProto(response);
  }
}