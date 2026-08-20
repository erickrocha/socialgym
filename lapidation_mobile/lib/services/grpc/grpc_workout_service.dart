import 'package:grpc/grpc.dart' as grpc;
import 'package:lapidation_mobile/commons/exercise_mapper.dart';
import 'package:lapidation_mobile/commons/workout_mapper.dart';
import 'package:lapidation_mobile/models/exercise.dart';
import 'package:lapidation_mobile/models/workout.dart';
import 'package:lapidation_mobile/services/base_service.dart';
import 'package:lapidation_mobile/services/grpc/grpc_channel_factory.dart';
import 'package:lapidation_mobile/src/generated/grpc/workout.pbgrpc.dart' as $workout;

import '../../config/api_config.dart';

class GrpcWorkoutService {
  GrpcWorkoutService._();

  static $workout.WorkoutServiceClient? _client;

  static Future<Workout> getWorkoutsById({required int id}) async {
    try {
      final response = await _ensureClient().getWorkout(
        $workout.WorkoutRequest()..id = id,
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return WorkoutMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to load workout');
    }
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
    try {
      final response = await _ensureClient().getWorkout(
        $workout.WorkoutRequest()..uuid = uuid,
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return WorkoutMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to load workout');
    }
  }

  static Future<List<Workout>> getWorkoutsByOwnerUuid(
      {required String ownerUuid}) async {
    try {
      final response = await _ensureClient().getWorkoutsByOwner(
        $workout.WorkoutListRequest()..ownerUuid = ownerUuid,
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return WorkoutMapper().fromProtoList(response.workouts);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to load workouts');
    }
  }

  static Future<Workout> createWorkout({required Workout workout}) async {
    try {
      final mapper = WorkoutMapper();
      final response = await _ensureClient().addWorkout(
        $workout.Workout()..mergeFromMessage(mapper.toProto(workout)),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return mapper.fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to create workout');
    }
  }

  static Future<Workout> update({required Workout workout}) async {
    try {
      final mapper = WorkoutMapper();
      final response = await _ensureClient().updateWorkout(
        $workout.Workout()..mergeFromMessage(mapper.toProto(workout)),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return mapper.fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to update workout');
    }
  }

  static Future<Workout> addExercisesToWorkout({required String workoutUuid, required List<Exercise> exercises}) async {
    try {
      final response = await _ensureClient().addExercisesToWorkout(
        $workout.WorkoutExercisesRequest()
          ..workoutUuid = workoutUuid
          ..exercises.addAll(exercises.map((e) => ExerciseMapper().toProto(e))),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return WorkoutMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to add exercises to workout');
    }
  }
}
