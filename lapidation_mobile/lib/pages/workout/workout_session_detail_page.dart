import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/exercise.dart';
import '../../models/workout.dart';
import '../../models/workout_session.dart';
import '../../providers/person_provider.dart';
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

    // Group sets by exercise uuid for display, preserving order
    final Map<String, List<Map<String, dynamic>>> grouped =
        <String, List<Map<String, dynamic>>>{};
    for (final set in session.executedSets) {
      final uuid = set['uuid'] as String? ?? '';
      grouped.putIfAbsent(uuid, () => []).add(set);
    }

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
                          _WhiteChip(
                            icon: Icons.timer_outlined,
                            label: session.formattedDuration,
                          ),
                          _WhiteChip(
                            icon: Icons.repeat,
                            label: '${session.totalSets} sets',
                          ),
                          _WhiteChip(
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
                  ...grouped.entries.map(
                    (e) => _ExerciseCard(
                      exerciseName:
                          e.value.first['exerciseName'] as String? ?? '',
                      category: e.value.first['category'] as String? ?? 'Force',
                      sets: e.value,
                      l10n: l10n,
                    ),
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

// ---------------------------------------------------------------------------
// Exercise card — shows all sets in a table
// ---------------------------------------------------------------------------

class _ExerciseCard extends StatelessWidget {
  final String exerciseName;
  final String category;
  final List<Map<String, dynamic>> sets;
  final AppLocalizations l10n;

  const _ExerciseCard({
    required this.exerciseName,
    required this.category,
    required this.sets,
    required this.l10n,
  });

  @override
  Widget build(BuildContext context) {
    final isCardio = category.toLowerCase() == 'cardio';
    final businessType = context
        .read<PersonProvider>()
        .activeBusinessProfile
        ?.businessType;

    return Container(
      margin: const EdgeInsets.only(bottom: 12),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withAlpha(12),
            blurRadius: 4,
            offset: const Offset(0, 2),
          ),
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
              borderRadius: const BorderRadius.vertical(
                top: Radius.circular(12),
              ),
            ),
            child: Row(
              children: [
                Icon(
                  Icons.fitness_center,
                  size: 16,
                  color: AppColors.primaryFor(businessType),
                ),
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
                Container(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 8,
                    vertical: 2,
                  ),
                  decoration: BoxDecoration(
                    color: AppColors.primaryFor(businessType).withAlpha(30),
                    borderRadius: BorderRadius.circular(10),
                  ),
                  child: Text(
                    category,
                    style: TextStyle(
                      fontSize: 11,
                      color: AppColors.primaryFor(businessType),
                    ),
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
// White chip for gradient header
// ---------------------------------------------------------------------------

class _WhiteChip extends StatelessWidget {
  final IconData icon;
  final String label;

  const _WhiteChip({required this.icon, required this.label});

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
          Text(
            label,
            style: const TextStyle(fontSize: 12, color: Colors.white),
          ),
        ],
      ),
    );
  }
}
