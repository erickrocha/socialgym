import 'package:flutter_test/flutter_test.dart';
import 'package:lapidation_mobile/models/evolution_check_in.dart';
import 'package:lapidation_mobile/models/person.dart';
import 'package:lapidation_mobile/utils/evolution_body_progress_mapper.dart';

void main() {
  group('computeVitruvianProgress', () {
    test('uses profile baseline with first-checkin fallback', () {
      final personInfo = PersonInfo(
        id: 1,
        personId: 1,
        weight: 80,
        height: 180,
        uuid: 'pinfo-1',
        createdAt: DateTime(2026, 1, 1),
        updatedAt: DateTime(2026, 1, 1),
      );

      final checkins = [
        EvolutionCheckIn(
          uuid: 'a',
          personUuid: 'person-1',
          createdAt: DateTime(2026, 1, 1),
          note: 'start',
          visibility: 'Private',
          composition: EvolutionComposition(weight: 81, bodyFatPct: 20),
          circumferences: EvolutionCircumferences(chest: 100, waist: 90),
        ),
        EvolutionCheckIn(
          uuid: 'b',
          personUuid: 'person-1',
          createdAt: DateTime(2026, 2, 1),
          note: 'later',
          visibility: 'Private',
          composition: EvolutionComposition(weight: 78, bodyFatPct: 18),
          circumferences: EvolutionCircumferences(chest: 104, waist: 86),
        ),
      ];

      final result = computeVitruvianProgress(
        checkins: checkins,
        personInfo: personInfo,
      );

      expect(result.hasRenderableData, isTrue);
      expect(result.baselineSource, BaselineSource.profileThenCheckinFallback);
      expect(result.progressByRegion[VitruvianRegion.chest], isNotNull);
      expect(
        result.progressByRegion[VitruvianRegion.waist]!.intensity,
        greaterThan(0),
      );
    });

    test('interpolates when latest check-in misses fields', () {
      final checkins = [
        EvolutionCheckIn(
          uuid: 'a',
          personUuid: 'person-1',
          createdAt: DateTime(2026, 1, 1),
          note: 'start',
          visibility: 'Private',
          composition: EvolutionComposition(weight: 80),
          circumferences: EvolutionCircumferences(waist: 92),
        ),
        EvolutionCheckIn(
          uuid: 'b',
          personUuid: 'person-1',
          createdAt: DateTime(2026, 2, 1),
          note: 'middle',
          visibility: 'Private',
          composition: EvolutionComposition(weight: 78),
          circumferences: EvolutionCircumferences(waist: 88),
        ),
        EvolutionCheckIn(
          uuid: 'c',
          personUuid: 'person-1',
          createdAt: DateTime(2026, 3, 1),
          note: 'missing latest waist',
          visibility: 'Private',
          composition: EvolutionComposition(weight: 77),
          circumferences: EvolutionCircumferences(),
        ),
      ];

      final result = computeVitruvianProgress(checkins: checkins);

      expect(result.hasRenderableData, isTrue);
      expect(result.usedInterpolation, isTrue);
      expect(result.progressByRegion[VitruvianRegion.waist], isNotNull);
      expect(
        result.progressByRegion[VitruvianRegion.waist]!.interpolated,
        isTrue,
      );
    });
  });
}
