import 'package:fixnum/fixnum.dart';
import 'package:grpc/grpc.dart' as grpc;
import 'package:lapidation_mobile/commons/exercise_mapper.dart';
import 'package:lapidation_mobile/config/api_config.dart';
import 'package:lapidation_mobile/models/exercise.dart';
import 'package:lapidation_mobile/services/base_service.dart';
import 'package:lapidation_mobile/services/grpc/grpc_channel_factory.dart';
import 'package:lapidation_mobile/src/generated/grpc/exercise.pbgrpc.dart'
    as $exercise;

import '../../models/paginated_exercise_response.dart';

class GrpcExerciseService {
  GrpcExerciseService._();

  static $exercise.ExerciseServiceClient? _client;

  static $exercise.ExerciseServiceClient _ensureClient() {
    if (_client != null) return _client!;
    final channel = GrpcChannelFactory.channelFor(
      host: ApiConfig.grpcHost,
      port: ApiConfig.grpcPort,
      authority: ApiConfig.grpcAuthority,
    );
    _client = $exercise.ExerciseServiceClient(
      channel,
      interceptors: GrpcChannelFactory.interceptors,
    );
    return _client!;
  }

  static Future<Exercise> getExercise({required int id}) async {
    try {
      final response = await _ensureClient().getExercise(
        $exercise.ExerciseRequest()..id = id,
        options: grpc.CallOptions(timeout: ApiConfig.timeout));
      return ExerciseMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to load exercise');
    }
  }

  static Future<Exercise> getExerciseByUuid({required String uuid}) async {
    try {
      final response = await _ensureClient().getExercise(
        $exercise.ExerciseRequest()..uuid = uuid,
        options: grpc.CallOptions(timeout: ApiConfig.timeout),);
      return ExerciseMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to load exercise');
    }
  }

  static Future<Exercise> addExercise({required Exercise exercise}) async {
    try {
      final response = await _ensureClient().addExercise(
        ExerciseMapper().toProto(exercise),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),);
      return ExerciseMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to create exercise');
    }
  }

  static Future<Exercise> updateExercise({required Exercise exercise}) async {
    try {
      final response = await _ensureClient().updateExercise(
        ExerciseMapper().toProto(exercise),
        options: grpc.CallOptions(timeout: ApiConfig.timeout),);
      return ExerciseMapper().fromProto(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to update exercise');
    }
  }

  static Future<PaginatedExerciseResponse> getPaginatedExercises({
    required String ownerUuid,
    required String category,
    required String visibility,
    required List<String> publicOwners,
    required int pageNumber,
    required int pageSize,
    required String sortBy
  }) async {
    try {
      final response = await _ensureClient().getExercises(
        $exercise.ExerciseParams()
          ..ownerUuid = ownerUuid
          ..category = category
          ..visibility = visibility
          ..owners.addAll(publicOwners)
          ..pageNumber = Int64(pageNumber)
          ..pageSize = Int64(pageSize)
          ..sortBy = sortBy,
        options: grpc.CallOptions(timeout: ApiConfig.timeout),);
      return fromProtoPaginated(response);
    } on grpc.GrpcError catch (e) {
      throw BaseService.handleGrpcError(e, 'Failed to load exercises');
    }
  }

  static PaginatedExerciseResponse fromProtoPaginated($exercise.PaginatedExercise proto) {
    final exercises = proto.content.map((e) => ExerciseMapper().fromProto(e)).toList();
    return PaginatedExerciseResponse(
      content: exercises,
      totalCount: proto.totalCount.toInt(),
      pageNumber: proto.pageNumber.toInt(),
      pageSize: proto.pageSize.toInt(),
      hasNextPage: proto.hasNextPage,
    );
  }
}
