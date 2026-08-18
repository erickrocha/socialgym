import 'package:flutter/material.dart';

import '../config/app_colors.dart';
import '../l10n/app_localizations.dart';
import '../models/exercise.dart';
import '../models/visibility_option.dart';

class ExerciseListWidget extends StatefulWidget {
  final List<Exercise> exercises;
  final String workoutName;
  final VoidCallback? onAddExercise;
  final void Function(int oldIndex, int newIndex)? onReorder;
  final void Function(Exercise exercise, int index)? onDelete;

  const ExerciseListWidget({
    super.key,
    required this.exercises,
    required this.workoutName,
    this.onAddExercise,
    this.onReorder,
    this.onDelete,
  });

  @override
  State<ExerciseListWidget> createState() => _ExerciseListWidgetState();
}

class _ExerciseListWidgetState extends State<ExerciseListWidget> {
  late List<Exercise> _exercises;

  @override
  void initState() {
    super.initState();
    _exercises = List.from(widget.exercises);
  }

  @override
  void didUpdateWidget(ExerciseListWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.exercises != widget.exercises) {
      _exercises = List.from(widget.exercises);
    }
  }

  void _onReorderItem(int oldIndex, int newIndex) {
    setState(() {
      if (newIndex > oldIndex) {
        newIndex -= 1;
      }
      final exercise = _exercises.removeAt(oldIndex);
      _exercises.insert(newIndex, exercise);
    });
    widget.onReorder?.call(oldIndex, newIndex);
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    if (_exercises.isEmpty) {
      return _buildEmpty(l10n);
    }

    return Container(
      margin: const EdgeInsets.only(top: 8),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: AppColors.primary.withAlpha(40)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header
          Row(
            children: [
              Expanded(
                child: Text(
                  '${l10n.workoutExercisesFor} ${widget.workoutName}',
                  style: const TextStyle(
                    fontSize: 15,
                    fontWeight: FontWeight.w600,
                    color: Color(0xFF333333),
                  ),
                ),
              ),
              Text(
                '${_exercises.length} ${l10n.workoutExercises}',
                style: const TextStyle(fontSize: 12, color: Color(0xFF999999)),
              ),
              if (widget.onAddExercise != null) ...[
                const SizedBox(width: 8),
                SizedBox(
                  height: 28,
                  child: ElevatedButton.icon(
                    onPressed: widget.onAddExercise,
                    icon: const Icon(Icons.add, size: 14),
                    label: Text(l10n.workoutAddExercise, style: const TextStyle(fontSize: 12)),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: AppColors.primary,
                      foregroundColor: Colors.white,
                      padding: const EdgeInsets.symmetric(horizontal: 8),
                      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(14)),
                    ),
                  ),
                ),
              ],
            ],
          ),

          const SizedBox(height: 12),

          // Exercise items with drag and drop
          ReorderableListView.builder(
            shrinkWrap: true,
            physics: const NeverScrollableScrollPhysics(),
            buildDefaultDragHandles: false,
            itemCount: _exercises.length,
            onReorderItem: _onReorderItem,
            proxyDecorator: (child, index, animation) {
              return AnimatedBuilder(
                animation: animation,
                builder: (context, child) {
                  final elevation = Tween<double>(
                    begin: 0,
                    end: 6,
                  ).animate(CurvedAnimation(parent: animation, curve: Curves.easeInOut)).value;
                  return Material(
                    elevation: elevation,
                    color: Colors.transparent,
                    borderRadius: BorderRadius.circular(8),
                    child: child,
                  );
                },
                child: child,
              );
            },
            itemBuilder: (context, index) {
              final exercise = _exercises[index];
              return _ReorderableExerciseItem(
                key: ValueKey(exercise.id ?? index),
                index: index,
                displayIndex: index + 1,
                exercise: exercise,
                l10n: l10n,
                onDelete: widget.onDelete != null
                    ? () {
                        final deletedExercise = _exercises[index];
                        setState(() {
                          _exercises.removeAt(index);
                        });
                        widget.onDelete?.call(deletedExercise, index);
                      }
                    : null,
              );
            },
          ),
        ],
      ),
    );
  }

  Widget _buildEmpty(AppLocalizations l10n) {
    return Container(
      margin: const EdgeInsets.only(top: 8),
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: AppColors.primary.withAlpha(40)),
      ),
      child: Column(
        children: [
          const Text('🏋️', style: TextStyle(fontSize: 32)),
          const SizedBox(height: 8),
          Text(
            l10n.workoutNoExercises,
            style: const TextStyle(fontSize: 14, color: Color(0xFF666666)),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 4),
          Text(
            l10n.workoutAddExerciseHint,
            style: const TextStyle(fontSize: 12, color: Color(0xFF999999)),
            textAlign: TextAlign.center,
          ),
          if (widget.onAddExercise != null) ...[
            const SizedBox(height: 12),
            ElevatedButton.icon(
              onPressed: widget.onAddExercise,
              icon: const Icon(Icons.add, size: 16),
              label: Text(l10n.workoutAddExercise),
              style: ElevatedButton.styleFrom(
                backgroundColor: AppColors.primary,
                foregroundColor: Colors.white,
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20)),
              ),
            ),
          ],
        ],
      ),
    );
  }
}

class _ReorderableExerciseItem extends StatelessWidget {
  final int index;
  final int displayIndex;
  final Exercise exercise;
  final AppLocalizations l10n;
  final VoidCallback? onDelete;

  const _ReorderableExerciseItem({
    super.key,
    required this.index,
    required this.displayIndex,
    required this.exercise,
    required this.l10n,
    this.onDelete,
  });

  @override
  Widget build(BuildContext context) {
    final visibility = VisibilityOption.fromApiValue(exercise.visibility);
    final content = Container(
      margin: const EdgeInsets.only(bottom: 8),
      padding: const EdgeInsets.symmetric(vertical: 4),
      decoration: BoxDecoration(color: Colors.white, borderRadius: BorderRadius.circular(8)),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          // Drag handle
          ReorderableDragStartListener(
            index: index,
            child: Container(
              padding: const EdgeInsets.all(8),
              child: const Icon(Icons.drag_handle, color: Color(0xFF999999), size: 20),
            ),
          ),
          // Number badge
          Container(
            width: 28,
            height: 28,
            decoration: BoxDecoration(
              color: AppColors.primary.withAlpha(25),
              borderRadius: BorderRadius.circular(14),
            ),
            child: Center(
              child: Text(
                '$displayIndex',
                style: const TextStyle(
                  fontSize: 13,
                  fontWeight: FontWeight.bold,
                  color: AppColors.primary,
                ),
              ),
            ),
          ),
          const SizedBox(width: 12),
          // Content
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  exercise.name,
                  style: const TextStyle(
                    fontSize: 14,
                    fontWeight: FontWeight.w600,
                    color: Color(0xFF333333),
                  ),
                ),
                // Owner name with icon, category, and visibility - first row
                Padding(
                  padding: const EdgeInsets.only(top: 4.0, bottom: 4.0),
                  child: Row(
                    children: [
                      // Owner with person icon
                      Icon(Icons.person, size: 14, color: AppColors.primary),
                      const SizedBox(width: 4),
                      Flexible(
                        child: Text(
                          exercise.ownerName,
                          style: const TextStyle(fontSize: 11, color: Color(0xFF555555)),
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                        ),
                      ),
                      const SizedBox(width: 12),
                      // Category icon
                      Tooltip(
                        message: exercise.category,
                        child: Icon(
                          _getCategoryIcon(exercise.category),
                          size: 16,
                          color: AppColors.primary,
                        ),
                      ),
                      // Visibility icon aligned to right
                      const Spacer(),
                      Tooltip(
                        message: visibility.label(l10n),
                        child: Icon(visibility.icon, size: 16, color: AppColors.primary),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 4),
                // Details row - second row
                Row(
                  children: [
                    Flexible(
                      child: _DetailChip(label: l10n.workoutSets, value: '${exercise.sets}'),
                    ),
                    const SizedBox(width: 8),
                    Flexible(
                      child: _DetailChip(
                        label: l10n.workoutReps,
                        value: '${exercise.repsOrDuration}',
                      ),
                    ),
                    if (exercise.weight > 0) ...[
                      const SizedBox(width: 8),
                      Flexible(
                        child: _DetailChip(
                          label: l10n.workoutWeight,
                          value: '${exercise.weight} ${l10n.workoutWeightUnit}',
                        ),
                      ),
                    ],
                  ],
                ),
              ],
            ),
          ),
        ],
      ),
    );

    if (onDelete == null) {
      return content;
    }

    return Dismissible(
      key: ValueKey('dismissible_${exercise.id ?? index}'),
      direction: DismissDirection.endToStart,
      confirmDismiss: (direction) async {
        onDelete?.call();
        return false;
      },
      background: Container(
        margin: const EdgeInsets.only(bottom: 8),
        decoration: BoxDecoration(color: Colors.red, borderRadius: BorderRadius.circular(8)),
        alignment: Alignment.centerRight,
        padding: const EdgeInsets.only(right: 20),
        child: const Icon(Icons.delete, color: Colors.white, size: 24),
      ),
      child: content,
    );
  }
}

class _DetailChip extends StatelessWidget {
  final String label;
  final String value;

  const _DetailChip({required this.label, required this.value});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
      decoration: BoxDecoration(
        color: const Color(0xFFF0F0F0),
        borderRadius: BorderRadius.circular(6),
      ),
      child: Text('$label: $value', style: const TextStyle(fontSize: 11, color: Color(0xFF555555))),
    );
  }
}

IconData _getCategoryIcon(String category) {
  switch (category.toLowerCase()) {
    case 'force':
      return Icons.fitness_center;
    case 'cardio':
      return Icons.directions_run;
    default:
      return Icons.category;
  }
}
