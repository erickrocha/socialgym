import 'package:lapidation_mobile/commons/exercise_mapper.dart';
import 'package:lapidation_mobile/models/workout.dart' as domain;
import 'package:lapidation_mobile/src/generated/grpc/workout.pbgrpc.dart'
    as $workout;

import 'mapper.dart';

class WorkoutMapper implements Mapper<domain.Workout, $workout.Workout> {
  @override
  $workout.Workout toProto(domain.Workout domain) {
    return $workout.Workout(
      id: domain.id,
      uuid: domain.uuid,
      ownerId: domain.ownerId,
      ownerUuid: domain.ownerUuid,
      name: domain.name,
      description: domain.description,
      difficulty: _difficultyToProto(domain.difficulty),
      muscleGroup: domain.muscleGroup,
      visibility: domain.visibility,
      createdAt: domain.createdAt?.toIso8601String(),
      updatedAt: domain.updatedAt?.toIso8601String(),
    );
  }

  @override
  domain.Workout fromProto($workout.Workout proto) {
    return domain.Workout(
      id: proto.id,
      uuid: proto.uuid,
      ownerId: proto.ownerId,
      ownerUuid: proto.ownerUuid,
      name: proto.name,
      description: proto.description,
      difficulty: _difficultyFromProto(proto.difficulty),
      createdAt: DateTime.parse(proto.createdAt),
      updatedAt: DateTime.parse(proto.updatedAt),
      exercises: ExerciseMapper().fromProtoList(proto.exercises),
      muscleGroup: proto.muscleGroup,
      visibility: proto.visibility,
    );
  }

  @override
  List<domain.Workout> fromProtoList(List<$workout.Workout> protoList) {
    return protoList.map((proto) => fromProto(proto)).toList();
  }

  @override
  List<$workout.Workout> toProtoList(List<domain.Workout> domainList) {
    return domainList.map((domain) => toProto(domain)).toList();
  }

  String _difficultyToProto(String difficulty) {
    switch (difficulty.toLowerCase()) {
      case 'easy':
        return 'Easy';
      case 'medium':
        return 'Medium';
      case 'hard':
        return 'Hard';
      case 'strong':
        return 'Strong';
      case 'soft':
      default:
        return 'Soft';
    }
  }

  String _difficultyFromProto(String difficulty) {
    switch (difficulty.toLowerCase()) {
      case 'easy':
      case 'medium':
      case 'hard':
      case 'strong':
        return difficulty.toLowerCase();
      case 'soft':
      default:
        return 'soft';
    }
  }
}
