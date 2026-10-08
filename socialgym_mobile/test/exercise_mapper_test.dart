import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/commons/exercise_mapper.dart';
import 'package:socialgym_mobile/src/generated/grpc/exercise.pb.dart' as $exercise;

void main() {
  test('the id and the description survive the trip through the proto message', () {
    final proto = $exercise.Exercise(
      id: 42,
      uuid: 'u1',
      name: 'Squat',
      ownerId: 7,
      ownerUuid: 'o1',
      ownerName: 'Ana',
      description: 'deep',
      sets: 3,
      category: 'Force',
      repsOrDuration: 10,
      visibility: 'Private',
      createdAt: '2026-10-01T10:00:00',
      updatedAt: '2026-10-01T10:00:00',
    );
    final domain = ExerciseMapper().fromProto(proto);
    expect((domain.id, domain.description), (42, 'deep'));
    final back = ExerciseMapper().toProto(domain);
    expect((back.id, back.description), (42, 'deep'));
  });

  test('an unsaved exercise has no id and goes out as 0', () {
    final domain = ExerciseMapper().fromProto($exercise.Exercise(name: 'New', createdAt: '2026-10-01T10:00:00', updatedAt: '2026-10-01T10:00:00'));
    expect(domain.id, isNull);
    expect(ExerciseMapper().toProto(domain).id, 0);
  });
}
