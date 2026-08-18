

import 'package:socialgym_mobile/commons/mapper.dart';

import '../models/exercise.dart';
import '../src/generated/grpc/exercise.pb.dart' as $exercise;

class ExerciseMapper implements Mapper<Exercise, $exercise.Exercise> {
  @override
  Exercise fromProto($exercise.Exercise proto) {
    return Exercise(
      name: proto.name,
      ownerId: proto.ownerId,
      ownerUuid: proto.ownerUuid,
      ownerName: proto.ownerName,
      sets: proto.sets,
      category: proto.category,
      repsOrDuration: proto.repsOrDuration,
      uuid: proto.uuid,
      visibility: proto.visibility,
      createdAt: DateTime.parse(proto.createdAt),
      updatedAt: DateTime.parse(proto.updatedAt),
    );
  }

  @override
  List<Exercise> fromProtoList(List<$exercise.Exercise> protoList) {
    return protoList.map((proto) => fromProto(proto)).toList();
  }

  @override
  $exercise.Exercise toProto(Exercise domain) {
    return $exercise.Exercise(
      name: domain.name,
      ownerId: domain.ownerId,
      ownerUuid: domain.ownerUuid,
      ownerName: domain.ownerName,
      sets: domain.sets,
      category: domain.category,
      repsOrDuration: domain.repsOrDuration,
      uuid: domain.uuid,
      visibility: domain.visibility,
      createdAt: domain.createdAt?.toIso8601String(),
      updatedAt: domain.updatedAt?.toIso8601String(),
    );
  }

  @override
  List<$exercise.Exercise> toProtoList(List<Exercise> domainList) {
    return domainList.map((domain) => toProto(domain)).toList();
  }


}