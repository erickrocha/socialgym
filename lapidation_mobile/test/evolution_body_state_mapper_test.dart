import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/models/evolution_check_in.dart';
import 'package:lapidation_mobile/models/person.dart';
import 'package:lapidation_mobile/utils/evolution_body_state_mapper.dart';

EvolutionCheckIn _checkIn({
  required DateTime createdAt,
  double? weight,
  double? bodyFatPct,
  double? muscleMassPct,
}) {
  return EvolutionCheckIn(
    uuid: 'u',
    personUuid: 'p',
    createdAt: createdAt,
    note: '',
    visibility: 'Private',
    composition: EvolutionComposition(
      weight: weight,
      bodyFatPct: bodyFatPct,
      muscleMassPct: muscleMassPct,
    ),
  );
}

PersonInfo _personInfo({double? weight, double? height}) {
  return PersonInfo(
    id: 1,
    personId: 1,
    weight: weight,
    height: height,
    uuid: 'pi',
    createdAt: DateTime(2026, 1, 1),
    updatedAt: DateTime(2026, 1, 1),
  );
}

void main() {
  group('computeVitruvianBodyState - defaults', () {
    test('empty checkins returns medium/medium, scale 1.0, no composition data', () {
      final state = computeVitruvianBodyState(checkins: const [], gender: 'male');

      expect(state.fatBucket, BodyLevel.medium);
      expect(state.muscleBucket, BodyLevel.medium);
      expect(state.scale, 1.0);
      expect(state.hasCompositionData, isFalse);
    });

    test('null gender defaults to male and reports hasGenderData false', () {
      final state = computeVitruvianBodyState(checkins: const [], gender: null);

      expect(state.gender, 'male');
      expect(state.hasGenderData, isFalse);
    });

    test('empty string gender defaults to male and reports hasGenderData false', () {
      final state = computeVitruvianBodyState(checkins: const [], gender: '  ');

      expect(state.gender, 'male');
      expect(state.hasGenderData, isFalse);
    });

    test('gender is case-insensitively normalized', () {
      final state = computeVitruvianBodyState(checkins: const [], gender: 'FEMALE');

      expect(state.gender, 'female');
      expect(state.hasGenderData, isTrue);
    });
  });

  group('computeVitruvianBodyState - fat bucket thresholds', () {
    test('male: 14.9% is low, 15% is medium, 25% is medium, 25.1% is high', () {
      DateTime d(int day) => DateTime(2026, 1, day);

      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d(1), bodyFatPct: 14.9)],
          gender: 'male',
        ).fatBucket,
        BodyLevel.low,
      );
      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d(1), bodyFatPct: 15)],
          gender: 'male',
        ).fatBucket,
        BodyLevel.medium,
      );
      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d(1), bodyFatPct: 25)],
          gender: 'male',
        ).fatBucket,
        BodyLevel.medium,
      );
      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d(1), bodyFatPct: 25.1)],
          gender: 'male',
        ).fatBucket,
        BodyLevel.high,
      );
    });

    test('female: 21.9% is low, 22% is medium, 32% is medium, 32.1% is high', () {
      final d = DateTime(2026, 1, 1);

      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d, bodyFatPct: 21.9)],
          gender: 'female',
        ).fatBucket,
        BodyLevel.low,
      );
      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d, bodyFatPct: 22)],
          gender: 'female',
        ).fatBucket,
        BodyLevel.medium,
      );
      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d, bodyFatPct: 32)],
          gender: 'female',
        ).fatBucket,
        BodyLevel.medium,
      );
      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d, bodyFatPct: 32.1)],
          gender: 'female',
        ).fatBucket,
        BodyLevel.high,
      );
    });

    test('missing bodyFatPct on latest checkin defaults to medium and hasCompositionData false', () {
      final state = computeVitruvianBodyState(
        checkins: [_checkIn(createdAt: DateTime(2026, 1, 1), muscleMassPct: 30)],
        gender: 'male',
      );

      expect(state.fatBucket, BodyLevel.medium);
      expect(state.hasCompositionData, isFalse);
    });
  });

  group('computeVitruvianBodyState - muscle bucket thresholds', () {
    test('male: 32.9% is low, 33% is medium, 38% is medium, 38.1% is high', () {
      final d = DateTime(2026, 1, 1);

      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d, muscleMassPct: 32.9)],
          gender: 'male',
        ).muscleBucket,
        BodyLevel.low,
      );
      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d, muscleMassPct: 33)],
          gender: 'male',
        ).muscleBucket,
        BodyLevel.medium,
      );
      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d, muscleMassPct: 38)],
          gender: 'male',
        ).muscleBucket,
        BodyLevel.medium,
      );
      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d, muscleMassPct: 38.1)],
          gender: 'male',
        ).muscleBucket,
        BodyLevel.high,
      );
    });

    test('female: 23.9% is low, 24% is medium, 30% is medium, 30.1% is high', () {
      final d = DateTime(2026, 1, 1);

      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d, muscleMassPct: 23.9)],
          gender: 'female',
        ).muscleBucket,
        BodyLevel.low,
      );
      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d, muscleMassPct: 30)],
          gender: 'female',
        ).muscleBucket,
        BodyLevel.medium,
      );
      expect(
        computeVitruvianBodyState(
          checkins: [_checkIn(createdAt: d, muscleMassPct: 30.1)],
          gender: 'female',
        ).muscleBucket,
        BodyLevel.high,
      );
    });
  });

  group('computeVitruvianBodyState - scale', () {
    test('BMI at or below 18.5 clamps scale to 0.9', () {
      final state = computeVitruvianBodyState(
        checkins: [_checkIn(createdAt: DateTime(2026, 1, 1), weight: 50)],
        personInfo: _personInfo(height: 180),
        gender: 'male',
      );
      // BMI = 50 / 1.8^2 = 15.43 -> below 18.5
      expect(state.scale, 0.9);
    });

    test('BMI at or above 32 clamps scale to 1.15', () {
      final state = computeVitruvianBodyState(
        checkins: [_checkIn(createdAt: DateTime(2026, 1, 1), weight: 110)],
        personInfo: _personInfo(height: 170),
        gender: 'male',
      );
      // BMI = 110 / 1.7^2 = 38.06 -> above 32
      expect(state.scale, 1.15);
    });

    test('BMI at the midpoint (25.25) maps to scale midpoint (1.025)', () {
      final state = computeVitruvianBodyState(
        checkins: [_checkIn(createdAt: DateTime(2026, 1, 1), weight: 76.5225)],
        personInfo: _personInfo(height: 174),
        gender: 'male',
      );
      // height 1.74m^2 = 3.0276; BMI = 76.5225 / 3.0276 = 25.28 (~midpoint of 18.5-32)
      expect(state.scale, closeTo(1.025, 0.01));
    });

    test('missing weight defaults scale to 1.0', () {
      final state = computeVitruvianBodyState(
        checkins: [_checkIn(createdAt: DateTime(2026, 1, 1))],
        personInfo: _personInfo(height: 180),
        gender: 'male',
      );
      expect(state.scale, 1.0);
    });

    test('missing height defaults scale to 1.0', () {
      final state = computeVitruvianBodyState(
        checkins: [_checkIn(createdAt: DateTime(2026, 1, 1), weight: 80)],
        personInfo: _personInfo(height: null),
        gender: 'male',
      );
      expect(state.scale, 1.0);
    });

    test('falls back to personInfo.weight when latest checkin has no weight', () {
      final withCheckinWeight = computeVitruvianBodyState(
        checkins: [_checkIn(createdAt: DateTime(2026, 1, 1), weight: 50)],
        personInfo: _personInfo(weight: 110, height: 170),
        gender: 'male',
      );
      final withFallbackWeight = computeVitruvianBodyState(
        checkins: [_checkIn(createdAt: DateTime(2026, 1, 1))],
        personInfo: _personInfo(weight: 110, height: 170),
        gender: 'male',
      );

      expect(withCheckinWeight.scale, 0.9); // uses checkin's 50kg -> low BMI
      expect(withFallbackWeight.scale, 1.15); // uses personInfo's 110kg -> high BMI
    });
  });

  group('computeVitruvianBodyState - latest checkin selection', () {
    test('uses the most recent checkin by createdAt regardless of list order', () {
      final state = computeVitruvianBodyState(
        checkins: [
          _checkIn(createdAt: DateTime(2026, 1, 10), bodyFatPct: 10),
          _checkIn(createdAt: DateTime(2026, 3, 1), bodyFatPct: 30),
          _checkIn(createdAt: DateTime(2026, 2, 1), bodyFatPct: 20),
        ],
        gender: 'male',
      );

      expect(state.fatBucket, BodyLevel.high); // from the March (latest) checkin's 30%
    });
  });

  group('VitruvianBodyState.assetPath', () {
    test('builds the expected asset path', () {
      final state = computeVitruvianBodyState(
        checkins: [
          _checkIn(createdAt: DateTime(2026, 1, 1), bodyFatPct: 10, muscleMassPct: 40),
        ],
        gender: 'female',
      );

      expect(state.assetPath, 'assets/vitruvian3d/female_low_high.glb');
    });
  });
}
