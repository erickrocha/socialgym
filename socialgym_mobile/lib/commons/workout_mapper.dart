import 'package:socialgym_mobile/commons/exercise_mapper.dart';
import 'package:socialgym_mobile/models/workout.dart' as domain;
import 'package:socialgym_mobile/src/generated/grpc/workout.pbgrpc.dart'
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
      createdAt: DateTime.parse(proto.createdAt),
      updatedAt: DateTime.parse(proto.updatedAt),
      exercises: ExerciseMapper().fromProtoList(proto.exercises),
      muscleGroup: proto.muscleGroup,
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


}
