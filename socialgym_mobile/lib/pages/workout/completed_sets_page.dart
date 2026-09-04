import 'package:flutter/material.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../widgets/workout/completed_sets_grouped_view.dart';

/// Full-page, read-only view of every set completed so far in the current
/// workout execution, grouped by exercise. Reached by tapping the
/// "Completed Sets" box on [WorkoutExecutionPage].
class CompletedSetsPage extends StatelessWidget {
  final String workoutName;
  final List<Map<String, dynamic>> sets;

  const CompletedSetsPage({super.key, required this.workoutName, required this.sets});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    final totalVolume = sets.fold<double>(0, (acc, s) {
      if ((s['category'] as String? ?? '').toLowerCase() == 'cardio') {
        return acc;
      }
      final w = (s['weight'] as num?)?.toDouble() ?? 0.0;
      final r = (s['repsOrDuration'] as num?)?.toInt() ?? 0;
      return acc + w * r;
    });

    return Scaffold(
      backgroundColor: AppColors.background,
      body: Column(
        children: [
          // ── Gradient header ────────────────────────────────────────────────
          Container(
            width: double.infinity,
            decoration: const BoxDecoration(gradient: AppColors.gradient3),
            child: SafeArea(
              bottom: false,
              child: Padding(
                padding: const EdgeInsets.fromLTRB(4, 8, 16, 16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        IconButton(
                          icon: const Icon(Icons.arrow_back, color: Colors.white),
                          onPressed: () => Navigator.of(context).pop(),
                        ),
                        Expanded(
                          child: Text(
                            '${l10n.executionCompletedSets} (${sets.length})',
                            style: const TextStyle(
                              color: Colors.white,
                              fontSize: 20,
                              fontWeight: FontWeight.bold,
                            ),
                            overflow: TextOverflow.ellipsis,
                          ),
                        ),
                      ],
                    ),
                    Padding(
                      padding: const EdgeInsets.only(left: 16),
                      child: Text(
                        workoutName,
                        style: TextStyle(color: Colors.white.withAlpha(200), fontSize: 13),
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                    const SizedBox(height: 12),
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 16),
                      child: Wrap(
                        spacing: 10,
                        runSpacing: 6,
                        children: [
                          WhiteChip(
                            icon: Icons.repeat,
                            label: '${sets.length} ${l10n.executionSet}',
                          ),
                          WhiteChip(
                            icon: Icons.fitness_center,
                            label: '${totalVolume.toStringAsFixed(0)} ${l10n.workoutWeightUnit}',
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),

          // ── Grouped list ──────────────────────────────────────────────────
          Expanded(
            child: sets.isEmpty
                ? Center(
                    child: Text(
                      l10n.executionCompletedSets,
                      style: const TextStyle(color: Color(0xFF888888)),
                    ),
                  )
                : SingleChildScrollView(
                    padding: const EdgeInsets.fromLTRB(16, 20, 16, 24),
                    child: CompletedSetsGroupedView(sets: sets, l10n: l10n),
                  ),
          ),
        ],
      ),
    );
  }
}
