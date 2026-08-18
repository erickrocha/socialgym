# Vitruvian Progress Graph

This module renders a Vitruvian-style body graph using `CustomPainter` and SVG path strings.

## Files

- `vitruvian_progress_card.dart`: UI card wrapper + legend + region chips.
- `vitruvian_progress_painter.dart`: high-performance painter that colors body regions.
- `../../utils/evolution_body_progress_mapper.dart`: transforms `PersonInfo` + `EvolutionCheckIn` history into region intensities.

## Technical decisions

- Baseline policy: profile weight (`PersonInfo`) with first-checkin fallback for missing metrics.
- Comparison policy: absolute metric delta to intensity conversion.
- Missing metric policy: interpolation from historical check-ins.

## Region mapping

- Shoulders: chest + muscle mass
- Chest: chest + muscle mass + body fat
- Arms: biceps (L/R) + muscle mass
- Abdomen: abdomen + body fat + visceral fat
- Waist: waist + body fat + weight
- Hips: hip + weight
- Thighs: thigh (L/R) + weight
- Neck: neck + body fat

## Test

The transformation logic is covered in:

- `test/evolution_body_progress_mapper_test.dart`

