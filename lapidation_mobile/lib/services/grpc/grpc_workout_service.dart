import 'package:grpc/grpc.dart' as grpc;
import 'package:lapidation_mobile/commons/exercise_mapper.dart';
import 'package:lapidation_mobile/commons/workout_mapper.dart';
import 'package:lapidation_mobile/models/exercise.dart';
import 'package:lapidation_mobile/models/workout.dart';
import 'package:lapidation_mobile/services/base_service.dart';
import 'package:lapidation_mobile/services/grpc/grpc_channel_factory.dart';
import 'package:lapidation_mobile/src/generated/grpc/workout.pbgrpc.dart'
    as $workout;

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
      channel,
      interceptors: GrpcChannelFactory.interceptors,
    );
    return _client!;
  }

  /// Drops the cached client. Call after the underlying channel has been
  /// shut down (e.g. on sign-out) so the next call rebuilds against a
  /// fresh channel instead of reusing one that is closing/closed.
  static Future<void> shutdown() async {
    _client = null;
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

  static Future<List<Workout>> getWorkoutsByOwnerUuid({
    required String ownerUuid,
  }) async {
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

  static Future<Workout> createWorkout({
    required Workout workout,
    String? targetPersonUuid,
  }) async {
    try {
      final mapper = WorkoutMapper();
      final proto = $workout.Workout()
        ..mergeFromMessage(mapper.toProto(workout));
      if (targetPersonUuid != null && targetPersonUuid.isNotEmpty) {
        proto.targetPersonUuid = targetPersonUuid;
      }
      final response = await _ensureClient().addWorkout(
        proto,
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

  static Future<void> deleteWorkout({required String uuid}) async {
    try {
      await _ensureClient().deleteWorkout(
        $workout.WorkoutRequest()..uuid = uuid,
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to delete workout');
    }
  }

  static Future<Workout> addExercisesToWorkout({
    required String workoutUuid,
    required List<Exercise> exercises,
  }) async {
    try {
      final response = await _ensureClient().addExercisesToWorkout(
        $workout.WorkoutExercisesRequest()
          ..workoutUuid = workoutUuid
          ..exercises.addAll(exercises.map((e) => ExerciseMapper().toProto(e))),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),
      );
      return WorkoutMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(
        e,
        'Failed to add exercises to workout',
      );
    }
  }
}
