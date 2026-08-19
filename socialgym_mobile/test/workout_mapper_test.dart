import 'package:flutter_test/flutter_test.dart';
import 'package:socialgym_mobile/commons/workout_mapper.dart';
import 'package:socialgym_mobile/models/workout.dart' as domain;
import 'package:socialgym_mobile/src/generated/grpc/workout.pb.dart' as proto;

void main() {
  group('WorkoutMapper', () {
    test('serializes lowercase Dart difficulty and muscle groups', () {
      final workout = domain.Workout(
        ownerId: 1,
        ownerUuid: 'owner-uuid',
        name: 'Upper body',
        difficulty: 'hard',
        muscleGroup: 'chest|arms',
      );

      final mapped = WorkoutMapper().toProto(workout);

      expect(mapped.difficulty, 'Hard');
      expect(mapped.muscleGroup, 'chest|arms');
    });

    test('normalizes Rust difficulty when deserializing', () {
      final workout = proto.Workout(
        ownerId: 1,
        ownerUuid: 'owner-uuid',
        name: 'Leg day',
        difficulty: 'Strong',
        muscleGroup: 'legs|core',
        createdAt: '2026-08-19T12:00:00.000Z',
        updatedAt: '2026-08-19T12:00:00.000Z',
      );

      final mapped = WorkoutMapper().fromProto(workout);

      expect(mapped.difficulty, 'strong');
      expect(mapped.muscleGroup, 'legs|core');
    });
  });
}
