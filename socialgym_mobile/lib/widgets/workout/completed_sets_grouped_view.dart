import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../providers/person_provider.dart';

// ---------------------------------------------------------------------------
// Grouped "completed sets" view — shared by WorkoutSessionDetailPage (saved
// sessions) and CompletedSetsPage (in-progress workout execution).
// ---------------------------------------------------------------------------

/// Groups a flat list of executed-set maps by their exercise `uuid`
/// (insertion order preserved) and renders one [ExerciseSetsCard] per exercise.
class CompletedSetsGroupedView extends StatelessWidget {
  final List<Map<String, dynamic>> sets;
  final AppLocalizations l10n;

  const CompletedSetsGroupedView({super.key, required this.sets, required this.l10n});

  @override
  Widget build(BuildContext context) {
    final Map<String, List<Map<String, dynamic>>> grouped = <String, List<Map<String, dynamic>>>{};
    for (final set in sets) {
      final uuid = set['uuid'] as String? ?? '';
      grouped.putIfAbsent(uuid, () => []).add(set);
    }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: grouped.entries
          .map(
            (e) => ExerciseSetsCard(
              exerciseName: e.value.first['exerciseName'] as String? ?? '',
              category: e.value.first['category'] as String? ?? 'Force',
              sets: e.value,
              l10n: l10n,
            ),
          )
          .toList(),
    );
  }
}

// ---------------------------------------------------------------------------
// Exercise card — shows all sets for one exercise in a table, with a
// per-exercise total (set count + volume) in the header.
// ---------------------------------------------------------------------------

class ExerciseSetsCard extends StatelessWidget {
  final String exerciseName;
  final String category;
  final List<Map<String, dynamic>> sets;
  final AppLocalizations l10n;

  const ExerciseSetsCard({
    super.key,
    required this.exerciseName,
    required this.category,
    required this.sets,
    required this.l10n,
  });

  @override
  Widget build(BuildContext context) {
    final isCardio = category.toLowerCase() == 'cardio';
    final businessType = context.read<PersonProvider>().activeBusinessProfile?.businessType;

    final volume = sets.fold<double>(0, (acc, s) {
      final w = (s['weight'] as num?)?.toDouble() ?? 0.0;
      final r = (s['repsOrDuration'] as num?)?.toInt() ?? 0;
      return acc + w * r;
    });
    final totalLabel = isCardio
        ? '${sets.length}×'
        : '${sets.length}× · ${volume.toStringAsFixed(0)} ${l10n.workoutWeightUnit}';

    return Container(
      margin: const EdgeInsets.only(bottom: 12),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 2)),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Exercise header row
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
            decoration: BoxDecoration(
              color: AppColors.primaryFor(businessType).withAlpha(15),
              borderRadius: const BorderRadius.vertical(top: Radius.circular(12)),
            ),
            child: Row(
              children: [
                Icon(Icons.fitness_center, size: 16, color: AppColors.primaryFor(businessType)),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    exerciseName,
                    style: TextStyle(
                      fontSize: 14,
                      fontWeight: FontWeight.bold,
                      color: AppColors.primaryFor(businessType),
                    ),
                  ),
                ),
                Text(
                  totalLabel,
                  style: TextStyle(
                    fontSize: 11,
                    fontWeight: FontWeight.w600,
                    color: AppColors.primaryFor(businessType),
                  ),
                ),
              ],
            ),
          ),

          // Column labels
          Padding(
            padding: const EdgeInsets.fromLTRB(14, 8, 14, 4),
            child: Row(
              children: [
                SizedBox(
                  width: 36,
                  child: Text(
                    l10n.executionSet,
                    style: const TextStyle(
                      fontSize: 11,
                      fontWeight: FontWeight.w600,
                      color: Color(0xFF888888),
                    ),
                  ),
                ),
                Expanded(
                  child: Text(
                    isCardio ? l10n.sessionDetailSpeed : l10n.workoutWeight,
                    style: const TextStyle(
                      fontSize: 11,
                      fontWeight: FontWeight.w600,
                      color: Color(0xFF888888),
                    ),
                    textAlign: TextAlign.center,
                  ),
                ),
                Expanded(
                  child: Text(
                    isCardio ? l10n.workoutDuration : l10n.workoutReps,
                    style: const TextStyle(
                      fontSize: 11,
                      fontWeight: FontWeight.w600,
                      color: Color(0xFF888888),
                    ),
                    textAlign: TextAlign.center,
                  ),
                ),
              ],
            ),
          ),
          const Divider(height: 1),

          // Set rows
          ...sets.map((set) {
            final setNum = set['setNumber'] ?? 0;
            final weight = (set['weight'] as num?)?.toDouble() ?? 0.0;
            final reps = (set['repsOrDuration'] as num?)?.toInt() ?? 0;
            return Padding(
              padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
              child: Row(
                children: [
                  SizedBox(
                    width: 36,
                    child: Container(
                      width: 24,
                      height: 24,
                      decoration: BoxDecoration(
                        color: AppColors.primaryFor(businessType),
                        shape: BoxShape.circle,
                      ),
                      child: Center(
                        child: Text(
                          '$setNum',
                          style: const TextStyle(
                            color: Colors.white,
                            fontSize: 11,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                    ),
                  ),
                  Expanded(
                    child: Text(
                      isCardio
                          ? weight.toStringAsFixed(1)
                          : '${weight.toStringAsFixed(weight == weight.roundToDouble() ? 0 : 1)} kg',
                      style: const TextStyle(
                        fontSize: 14,
                        fontWeight: FontWeight.w600,
                        color: Color(0xFF333333),
                      ),
                      textAlign: TextAlign.center,
                    ),
                  ),
                  Expanded(
                    child: Text(
                      '$reps',
                      style: const TextStyle(
                        fontSize: 14,
                        fontWeight: FontWeight.w600,
                        color: Color(0xFF333333),
                      ),
                      textAlign: TextAlign.center,
                    ),
                  ),
                ],
              ),
            );
          }),
          const SizedBox(height: 4),
        ],
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// White chip for gradient headers
// ---------------------------------------------------------------------------

class WhiteChip extends StatelessWidget {
  final IconData icon;
  final String label;

  const WhiteChip({super.key, required this.icon, required this.label});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
      decoration: BoxDecoration(
        color: Colors.white.withAlpha(40),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.white.withAlpha(80)),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, size: 12, color: Colors.white),
          const SizedBox(width: 4),
          Text(label, style: const TextStyle(fontSize: 12, color: Colors.white)),
        ],
      ),
    );
  }
}
