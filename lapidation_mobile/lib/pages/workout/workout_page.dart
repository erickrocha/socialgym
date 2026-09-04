import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:lapidation_mobile/services/exercise_service.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/exercise.dart';
import '../../models/workout.dart';
import '../../providers/auth_provider.dart';
import '../../providers/person_provider.dart';
import '../../providers/workout_provider.dart';
import '../../widgets/exercise_list_widget.dart';
import '../../config/nav_section.dart';
import '../../widgets/main_layout.dart';
import '../../widgets/workout_card.dart';
import 'exercise_form_dialog.dart';
import 'workout_execution_page.dart';
import 'workout_form_dialog.dart';
import 'workout_form_page.dart';

class WorkoutPage extends StatefulWidget {
  const WorkoutPage({super.key});

  @override
  State<WorkoutPage> createState() => _WorkoutPageState();
}

class _WorkoutPageState extends State<WorkoutPage> {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _fetchWorkouts();
    });
  }

  void _fetchWorkouts() {
    final authProvider = context.read<AuthProvider>();
    final personProvider = context.read<PersonProvider>();
    final workoutProvider = context.read<WorkoutProvider>();
    final token = authProvider.auth?.accessToken ?? '';
    final ownerUuid = personProvider.isProfessional
        ? personProvider.activeBusinessProfile?.uuid
        : personProvider.person?.uuid;

    if (ownerUuid != null) {
      workoutProvider.fetchWorkouts(ownerUuid, token);
    }
  }

  void _navigateToAddWorkoutPage() {
    final personProvider = context.read<PersonProvider>();
    final ownerId = personProvider.activeAuthorId;
    final ownerUuid = personProvider.activeAuthorUuid;

    Navigator.of(context)
        .push(
          MaterialPageRoute(
            builder: (_) =>
                WorkoutFormPage(ownerId: ownerId, ownerUuid: ownerUuid),
          ),
        )
        .then((result) {
          if (result == true) {
            _fetchWorkouts();
          }
        });
  }

  void _showAddExerciseDialog(Workout workout) {
    showDialog(
      context: context,
      builder: (_) => ExerciseFormDialog(
        workoutId: workout.id,
        workoutUuid: workout.uuid!,
        workoutVisibility: workout.visibility,
      ),
    ).then((result) {
      if (result == true) {
        _fetchWorkouts();
      }
    });
  }

  void _showEditWorkoutDialog(Workout workout) {
    final personProvider = context.read<PersonProvider>();
    final ownerId = personProvider.activeAuthorId;
    final ownerUuid = personProvider.activeAuthorUuid;

    showDialog(
      context: context,
      builder: (_) => WorkoutFormDialog(
        ownerId: ownerId,
        ownerUuid: ownerUuid,
        initialWorkout: workout,
      ),
    ).then((result) {
      if (result == true) {
        _fetchWorkouts();
      }
    });
  }

  Future<void> _confirmDeleteWorkout(Workout workout) async {
    final l10n = AppLocalizations.of(context)!;
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (dialogContext) => AlertDialog(
        title: Text(l10n.workoutDeleteConfirmTitle),
        content: Text(l10n.workoutDeleteConfirm),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(dialogContext, false),
            child: Text(l10n.buttonCancel),
          ),
          TextButton(
            onPressed: () => Navigator.pop(dialogContext, true),
            child: Text(
              l10n.buttonConfirm,
              style: const TextStyle(color: AppColors.danger),
            ),
          ),
        ],
      ),
    );

    if (confirmed != true || !mounted || workout.uuid == null) return;

    final workoutProvider = context.read<WorkoutProvider>();
    await workoutProvider.deleteWorkout(workout.uuid!);
  }

  void _startWorkout(Workout workout) {
    Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => WorkoutExecutionPage(workout: workout)),
    );
  }

  void _onReorderExercises(Workout workout, int oldIndex, int newIndex) {
    final workoutProvider = context.read<WorkoutProvider>();
    workoutProvider.reorderExercises(workout.id!, oldIndex, newIndex);
  }

  Future<void> _deleteExercise(Exercise exercise, int index) async {
    if (exercise.id == null) return;

    final authProvider = context.read<AuthProvider>();
    final token = authProvider.auth?.accessToken ?? '';

    try {
      await ExerciseService.deleteExercise(exercise.id!, token);
      _fetchWorkouts();
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Failed to delete exercise: $e'),
            backgroundColor: Colors.red,
          ),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final businessType = context
        .watch<PersonProvider>()
        .activeBusinessProfile
        ?.businessType;

    return MainLayout(
      navSection: NavSection.workout,
      currentRoute: '/workouts',
      floatingActionButton: FloatingActionButton(
        onPressed: _navigateToAddWorkoutPage,
        backgroundColor: AppColors.primaryFor(businessType),
        foregroundColor: Colors.white,
        tooltip: l10n.workoutAddNew,
        child: const Icon(Icons.add),
      ),
      body: Consumer<WorkoutProvider>(
        builder: (context, workoutProvider, _) {
          if (workoutProvider.loading && workoutProvider.workouts.isEmpty) {
            return Center(
              child: CircularProgressIndicator(
                color: AppColors.primaryFor(businessType),
              ),
            );
          }

          // Pending / rejected / cancelled assignments live on the Workout
          // Invites screen; the main list is only accepted workouts.
          final workouts = workoutProvider.workouts
              .where((w) => w.status == 'Accepted')
              .toList();

          return Column(
            children: [
              // Error banner
              if (workoutProvider.error != null)
                Container(
                  width: double.infinity,
                  padding: const EdgeInsets.all(12),
                  color: AppColors.danger.withAlpha(25),
                  child: Row(
                    children: [
                      const Icon(
                        Icons.error_outline,
                        color: AppColors.danger,
                        size: 18,
                      ),
                      const SizedBox(width: 8),
                      Expanded(
                        child: Text(
                          workoutProvider.error!,
                          style: const TextStyle(
                            color: AppColors.danger,
                            fontSize: 13,
                          ),
                        ),
                      ),
                      IconButton(
                        icon: const Icon(Icons.close, size: 16),
                        onPressed: () => workoutProvider.clearError(),
                      ),
                    ],
                  ),
                ),

              // Header
              Container(
                padding: const EdgeInsets.symmetric(
                  horizontal: 16,
                  vertical: 16,
                ),
                child: Row(
                  children: [
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            l10n.workoutTitle,
                            style: const TextStyle(
                              fontSize: 24,
                              fontWeight: FontWeight.bold,
                              color: Color(0xFF333333),
                            ),
                          ),
                          const SizedBox(height: 4),
                          Text(
                            l10n.workoutDescription,
                            style: const TextStyle(
                              fontSize: 14,
                              color: Color(0xFF888888),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),

              // Workout list
              Expanded(
                child: workouts.isEmpty
                    ? _buildEmpty(l10n)
                    : _buildWorkoutList(
                        workoutProvider,
                        workouts,
                        l10n,
                        businessType,
                      ),
              ),
            ],
          );
        },
      ),
    );
  }

  Widget _buildEmpty(AppLocalizations l10n) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Text('🏋️', style: TextStyle(fontSize: 48)),
          const SizedBox(height: 16),
          Text(
            l10n.workoutNoWorkouts,
            style: const TextStyle(fontSize: 16, color: Color(0xFF888888)),
          ),
        ],
      ),
    );
  }

  Widget _buildWorkoutList(
    WorkoutProvider workoutProvider,
    List<Workout> workouts,
    AppLocalizations l10n,
    String? businessType,
  ) {
    return RefreshIndicator(
      color: AppColors.primaryFor(businessType),
      onRefresh: () async => _fetchWorkouts(),
      child: ListView.builder(
        padding: const EdgeInsets.symmetric(horizontal: 16),
        itemCount: workouts.length,
        itemBuilder: (context, index) {
          final workout = workouts[index];
          final isSelected = workoutProvider.selectedWorkout?.id == workout.id;

          return Padding(
            padding: const EdgeInsets.only(bottom: 12),
            child: Column(
              children: [
                Dismissible(
                  key: ValueKey('workout_${workout.uuid ?? workout.id}'),
                  direction: DismissDirection.horizontal,
                  confirmDismiss: (direction) async {
                    if (direction == DismissDirection.endToStart) {
                      await _confirmDeleteWorkout(workout);
                    } else {
                      _showEditWorkoutDialog(workout);
                    }
                    return false;
                  },
                  background: Container(
                    decoration: BoxDecoration(
                      color: AppColors.primaryFor(businessType),
                      borderRadius: BorderRadius.circular(12),
                    ),
                    alignment: Alignment.centerLeft,
                    padding: const EdgeInsets.only(left: 20),
                    child: const Icon(
                      Icons.edit,
                      color: Colors.white,
                      size: 24,
                    ),
                  ),
                  secondaryBackground: Container(
                    decoration: BoxDecoration(
                      color: Colors.red,
                      borderRadius: BorderRadius.circular(12),
                    ),
                    alignment: Alignment.centerRight,
                    padding: const EdgeInsets.only(right: 20),
                    child: const Icon(
                      Icons.delete,
                      color: Colors.white,
                      size: 24,
                    ),
                  ),
                  child: WorkoutCard(
                    workout: workout,
                    isSelected: isSelected,
                    onTap: () => workoutProvider.selectWorkout(workout),
                  ),
                ),
                if (isSelected) ...[
                  // Start workout button
                  if (workout.exercises.isNotEmpty)
                    Padding(
                      padding: const EdgeInsets.only(top: 8),
                      child: SizedBox(
                        width: double.infinity,
                        child: ElevatedButton.icon(
                          onPressed: () => _startWorkout(workout),
                          icon: const Text(
                            '🚀',
                            style: TextStyle(fontSize: 18),
                          ),
                          label: Text(
                            l10n.executionStartWorkout,
                            style: const TextStyle(
                              fontSize: 16,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                          style: ElevatedButton.styleFrom(
                            backgroundColor: AppColors.secondary,
                            foregroundColor: Colors.white,
                            shape: RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(12),
                            ),
                            padding: const EdgeInsets.symmetric(vertical: 14),
                          ),
                        ),
                      ),
                    ),

                  // Exercise list
                  ExerciseListWidget(
                    exercises: workout.exercises,
                    workoutName: workout.name,
                    onAddExercise: () => _showAddExerciseDialog(workout),
                    onReorder: (oldIndex, newIndex) =>
                        _onReorderExercises(workout, oldIndex, newIndex),
                    onDelete: _deleteExercise,
                  ),
                ],
              ],
            ),
          );
        },
      ),
    );
  }
}
