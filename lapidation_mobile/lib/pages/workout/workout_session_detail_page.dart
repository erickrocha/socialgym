import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/exercise.dart';
import '../../models/workout.dart';
import '../../models/workout_session.dart';
import '../../providers/person_provider.dart';
import '../../widgets/workout/completed_sets_grouped_view.dart';
import 'workout_execution_page.dart';

class WorkoutSessionDetailPage extends StatelessWidget {
  final WorkoutSession session;

  const WorkoutSessionDetailPage({super.key, required this.session});

  /// Reconstructs a [Workout] from the session's executedSets so the user
  /// can "start again" with the same exercises and last-used weights/reps.
  Workout _buildWorkoutFromSession({
    required int ownerId,
    required String ownerUuid,
  }) {
    // Group sets by exercise uuid preserving insertion order
    final Map<String, List<Map<String, dynamic>>> grouped =
        <String, List<Map<String, dynamic>>>{};
    for (final set in session.executedSets) {
      final uuid = set['uuid'] as String? ?? '';
      grouped.putIfAbsent(uuid, () => []).add(set);
    }

    final now = DateTime.now();
    final exercises = grouped.entries.map((entry) {
      final sets = entry.value;
      final first = sets.first;
      // Use the last set's weight/reps as the pre-filled default
      final last = sets.last;
      int authorId = (last['ownerId'] as num?)?.toInt() ?? 0;
      String authorUuid = (last['ownerUuid'] as String?) ?? '';
      return Exercise(
        name: first['exerciseName'] as String? ?? '',
        ownerId: authorId,
        ownerName: first['ownerName'] as String? ?? '',
        ownerUuid: authorUuid,
        sets: sets.length,
        category: first['category'] as String? ?? 'Force',
        repsOrDuration: (last['repsOrDuration'] as num?)?.toInt() ?? 0,
        uuid: entry.key,
        visibility: first['visibility'] as String? ?? 'Private',
        weight: (last['weight'] as num?)?.toDouble() ?? 0.0,
        createdAt: now,
        updatedAt: now,
      );
    }).toList();

    return Workout(
      name: session.workoutName,
      exercises: exercises,
      ownerId: ownerId,
      ownerUuid: ownerUuid,
    );
  }

  String _fmtDate(DateTime? d) {
    if (d == null) return '';
    return '${d.day.toString().padLeft(2, '0')}/'
        '${d.month.toString().padLeft(2, '0')}/'
        '${d.year}';
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final date = session.completedAtDate;
    final personProvider = context.read<PersonProvider>();
    final ownerId = personProvider.activeAuthorId;
    final ownerUuid = personProvider.activeAuthorUuid;
    final businessType = personProvider.activeBusinessProfile?.businessType;

    return Scaffold(
      backgroundColor: AppColors.background,
      body: Column(
        children: [
          // ── Gradient Header ──────────────────────────────────────────────
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
                          icon: const Icon(
                            Icons.arrow_back,
                            color: Colors.white,
                          ),
                          onPressed: () => Navigator.of(context).pop(),
                        ),
                        Expanded(
                          child: Text(
                            session.workoutName,
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
                        '${session.dayOfWeek}  •  ${_fmtDate(date)}',
                        style: TextStyle(
                          color: Colors.white.withAlpha(200),
                          fontSize: 13,
                        ),
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
                            icon: Icons.timer_outlined,
                            label: session.formattedDuration,
                          ),
                          WhiteChip(
                            icon: Icons.repeat,
                            label: '${session.totalSets} sets',
                          ),
                          WhiteChip(
                            icon: Icons.fitness_center,
                            label:
                                '${session.totalVolume.toStringAsFixed(3)} kg',
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),

          // ── Exercise list ────────────────────────────────────────────────
          Expanded(
            child: SingleChildScrollView(
              padding: const EdgeInsets.fromLTRB(16, 20, 16, 100),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    l10n.sessionDetailExercises,
                    style: const TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.bold,
                      color: Color(0xFF333333),
                    ),
                  ),
                  const SizedBox(height: 12),
                  CompletedSetsGroupedView(
                    sets: session.executedSets,
                    l10n: l10n,
                  ),
                ],
              ),
            ),
          ),
        ],
      ),

      // ── Start Again FAB ──────────────────────────────────────────────────
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () {
          final workout = _buildWorkoutFromSession(
            ownerId: ownerId,
            ownerUuid: ownerUuid,
          );
          Navigator.of(context).push(
            MaterialPageRoute(
              builder: (_) => WorkoutExecutionPage(workout: workout),
            ),
          );
        },
        backgroundColor: AppColors.primaryFor(businessType),
        foregroundColor: Colors.white,
        icon: const Icon(Icons.replay),
        label: Text(l10n.sessionDetailStartAgain),
      ),
    );
  }
}
