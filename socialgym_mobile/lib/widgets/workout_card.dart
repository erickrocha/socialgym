import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../config/app_colors.dart';
import '../l10n/app_localizations.dart';
import '../models/visibility_option.dart';
import '../models/workout.dart';
import '../providers/person_provider.dart';

class WorkoutCard extends StatelessWidget {
  final Workout workout;
  final bool isSelected;
  final VoidCallback? onTap;

  const WorkoutCard({super.key, required this.workout, this.isSelected = false, this.onTap});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final visibility = VisibilityOption.fromApiValue(workout.visibility);
    final businessType = context.watch<PersonProvider>().activeBusinessProfile?.businessType;

    return GestureDetector(
      onTap: onTap,
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 200),
        decoration: BoxDecoration(
          color: Colors.white,
          borderRadius: BorderRadius.circular(12),
          border: Border.all(
            color: isSelected ? AppColors.primaryFor(businessType) : Colors.transparent,
            width: 2,
          ),
          boxShadow: [
            BoxShadow(
              color: Colors.black.withAlpha(isSelected ? 30 : 15),
              blurRadius: isSelected ? 8 : 4,
              offset: const Offset(0, 2),
            ),
          ],
        ),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header: icon, title, difficulty, visibility
              Row(
                children: [
                  // Muscle group icon
                  Container(
                    width: 40,
                    height: 40,
                    decoration: BoxDecoration(
                      color: AppColors.primaryFor(businessType).withAlpha(25),
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Center(
                      child: Text(
                        _getMuscleGroupEmoji(workout.muscleGroup),
                        style: const TextStyle(fontSize: 20),
                      ),
                    ),
                  ),
                  const SizedBox(width: 12),
                  // Title and difficulty
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          workout.name,
                          style: const TextStyle(
                            fontSize: 16,
                            fontWeight: FontWeight.w600,
                            color: Color(0xFF333333),
                          ),
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                        ),
                        const SizedBox(height: 4),
                        _DifficultyBadge(difficulty: workout.difficulty, l10n: l10n),
                      ],
                    ),
                  ),
                  // Visibility icon
                  Text(visibility.emoji, style: const TextStyle(fontSize: 18)),
                ],
              ),

              // Description
              if (workout.description != null && workout.description!.isNotEmpty) ...[
                const SizedBox(height: 8),
                Text(
                  workout.description!,
                  style: const TextStyle(fontSize: 13, color: Color(0xFF666666)),
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                ),
              ],

              const SizedBox(height: 12),

              // Meta: muscle groups + exercise count
              Wrap(
                spacing: 8,
                runSpacing: 4,
                children: [
                  ...workout.muscleGroups.map(
                    (mg) => _MetaChip(icon: '🎯', label: _getMuscleGroupLabel(mg, l10n)),
                  ),
                  _MetaChip(
                    icon: '📋',
                    label: '${workout.exercises.length} ${l10n.workoutExercises}',
                  ),
                ],
              ),

              // Expand indicator
              if (isSelected) ...[
                const SizedBox(height: 8),
                Center(
                  child: Icon(Icons.keyboard_arrow_down, color: AppColors.primaryFor(businessType)),
                ),
              ],
            ],
          ),
        ),
      ),
    );
  }

  String _getMuscleGroupEmoji(String muscleGroup) {
    final primary = muscleGroup.isNotEmpty ? muscleGroup.split('|').first : '';
    switch (primary.toLowerCase()) {
      case 'chest':
        return '💪';
      case 'legs':
        return '🦵';
      case 'back':
        return '🔙';
      case 'core':
        return '🎯';
      case 'full_body':
        return '🏃';
      case 'shoulders':
        return '🏋️';
      case 'arms':
        return '💪';
      default:
        return '🏋️';
    }
  }

  String _getMuscleGroupLabel(String mg, AppLocalizations l10n) {
    switch (mg.toLowerCase()) {
      case 'chest':
        return l10n.muscleGroupChest;
      case 'legs':
        return l10n.muscleGroupLegs;
      case 'back':
        return l10n.muscleGroupBack;
      case 'core':
        return l10n.muscleGroupCore;
      case 'full_body':
        return l10n.muscleGroupFullBody;
      case 'shoulders':
        return l10n.muscleGroupShoulders;
      case 'arms':
        return l10n.muscleGroupArms;
      case 'glutes':
        return l10n.muscleGroupGlutes;
      default:
        return mg;
    }
  }
}

class _DifficultyBadge extends StatelessWidget {
  final String difficulty;
  final AppLocalizations l10n;

  const _DifficultyBadge({required this.difficulty, required this.l10n});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
      decoration: BoxDecoration(
        color: _getColor().withAlpha(30),
        borderRadius: BorderRadius.circular(10),
        border: Border.all(color: _getColor().withAlpha(100)),
      ),
      child: Text(
        _getLabel(),
        style: TextStyle(fontSize: 11, fontWeight: FontWeight.w600, color: _getColor()),
      ),
    );
  }

  Color _getColor() {
    switch (difficulty.toLowerCase()) {
      case 'soft':
        return const Color(0xFF4CAF50);
      case 'easy':
        return const Color(0xFF8BC34A);
      case 'medium':
        return const Color(0xFFFFC107);
      case 'hard':
        return const Color(0xFFFF9800);
      case 'strong':
        return const Color(0xFFF44336);
      default:
        return Colors.grey;
    }
  }

  String _getLabel() {
    switch (difficulty.toLowerCase()) {
      case 'soft':
        return l10n.difficultySoft;
      case 'easy':
        return l10n.difficultyEasy;
      case 'medium':
        return l10n.difficultyMedium;
      case 'hard':
        return l10n.difficultyHard;
      case 'strong':
        return l10n.difficultyStrong;
      default:
        return difficulty;
    }
  }
}

class _MetaChip extends StatelessWidget {
  final String icon;
  final String label;

  const _MetaChip({required this.icon, required this.label});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: const Color(0xFFF5F5F5),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(icon, style: const TextStyle(fontSize: 12)),
          const SizedBox(width: 4),
          Text(label, style: const TextStyle(fontSize: 12, color: Color(0xFF666666))),
        ],
      ),
    );
  }
}
