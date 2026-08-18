import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/exercise.dart';
import '../../models/workout.dart';
import '../../providers/auth_provider.dart';
import '../../providers/workout_provider.dart';
import '../../widgets/visibility_dropdown_field.dart';

enum WorkoutFormMode { save, startSession }

class WorkoutFormDialog extends StatefulWidget {
  final int ownerId;
  final List<Exercise> initialExercises;
  final String ownerUuid;
  final WorkoutFormMode mode;

  const WorkoutFormDialog({
    super.key,
    required this.ownerId,
    required this.ownerUuid,
    this.initialExercises = const [],
    this.mode = WorkoutFormMode.save,
  });

  @override
  State<WorkoutFormDialog> createState() => _WorkoutFormDialogState();
}

class _WorkoutFormDialogState extends State<WorkoutFormDialog> {
  final _formKey = GlobalKey<FormState>();
  final _nameController = TextEditingController();
  final _descriptionController = TextEditingController();
  final _nameFocus = FocusNode();
  final _descriptionFocus = FocusNode();
  String _difficulty = 'soft';
  String _visibility = 'Private';
  final Set<String> _selectedMuscleGroups = {};

  @override
  void dispose() {
    _nameController.dispose();
    _descriptionController.dispose();
    _nameFocus.dispose();
    _descriptionFocus.dispose();
    super.dispose();
  }

  Future<void> _handleSaveWorkout() async {
    if (!_formKey.currentState!.validate()) return;

    final authProvider = context.read<AuthProvider>();
    final workoutProvider = context.read<WorkoutProvider>();
    final token = authProvider.auth?.accessToken ?? '';

    final workoutData = {
      'name': _nameController.text.trim(),
      'description': _descriptionController.text.trim(),
      'difficulty': _difficulty,
      'muscleGroup': _selectedMuscleGroups.join('|'),
      'visibility': _visibility,
      'ownerId': widget.ownerId,
      'ownerUuid': widget.ownerUuid,
      'exercises': widget.initialExercises.map((e) => e.toJson()).toList(),
    };

    final success = await workoutProvider.addWorkout(workoutData, token);
    if (success && mounted) {
      Navigator.of(context).pop(true);
    }
  }

  void _handleStartSession() {
    if (!_formKey.currentState!.validate()) return;

    final workout = Workout(
      name: _nameController.text.trim(),
      description: _descriptionController.text.trim(),
      difficulty: _difficulty,
      muscleGroup: _selectedMuscleGroups.join('|'),
      visibility: _visibility,
      ownerId: widget.ownerId,
      ownerUuid: widget.ownerUuid,
      exercises: List.from(widget.initialExercises),
    );

    Navigator.of(context).pop(workout);
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final workoutProvider = context.watch<WorkoutProvider>();
    final isSaveMode = widget.mode == WorkoutFormMode.save;
    final dialogTitle = isSaveMode ? l10n.workoutAddNew : l10n.workoutStartSessionTitle;

    return Dialog(
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      insetPadding: const EdgeInsets.all(16),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 500, maxHeight: 600),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            // Header
            Container(
              padding: const EdgeInsets.all(16),
              decoration: const BoxDecoration(
                gradient: AppColors.gradient3,
                borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
              ),
              child: Row(
                children: [
                  Expanded(
                    child: Text(
                      dialogTitle,
                      style: const TextStyle(
                        color: Colors.white,
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                  IconButton(
                    icon: const Icon(Icons.close, color: Colors.white),
                    onPressed: () => Navigator.of(context).pop(),
                  ),
                ],
              ),
            ),

            // Form
            Flexible(
              child: SingleChildScrollView(
                padding: const EdgeInsets.all(16),
                child: Form(
                  key: _formKey,
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      // Error
                      if (isSaveMode && workoutProvider.error != null) ...[
                        Container(
                          padding: const EdgeInsets.all(12),
                          decoration: BoxDecoration(
                            color: AppColors.danger.withAlpha(25),
                            borderRadius: BorderRadius.circular(8),
                            border: Border.all(color: AppColors.danger.withAlpha(76)),
                          ),
                          child: Text(
                            workoutProvider.error!,
                            style: const TextStyle(color: AppColors.danger, fontSize: 13),
                            textAlign: TextAlign.center,
                          ),
                        ),
                        const SizedBox(height: 12),
                      ],

                      // Name
                      TextFormField(
                        controller: _nameController,
                        focusNode: _nameFocus,
                        textInputAction: TextInputAction.next,
                        decoration: _inputDecoration(
                          l10n.workoutFormName,
                          l10n.workoutFormNamePlaceholder,
                        ),
                        onFieldSubmitted: (_) {
                          FocusScope.of(context).requestFocus(_descriptionFocus);
                        },
                        validator: (v) =>
                            v == null || v.trim().isEmpty ? l10n.validationRequired : null,
                      ),
                      const SizedBox(height: 12),

                      // Description
                      TextFormField(
                        controller: _descriptionController,
                        focusNode: _descriptionFocus,
                        textInputAction: TextInputAction.done,
                        decoration: _inputDecoration(
                          l10n.workoutFormDescription,
                          l10n.workoutFormDescriptionPlaceholder,
                        ),
                        maxLines: 2,
                        onFieldSubmitted: (_) {
                          if (isSaveMode) {
                            if (!workoutProvider.loading) {
                              _handleSaveWorkout();
                            }
                          } else {
                            _handleStartSession();
                          }
                        },
                      ),
                      const SizedBox(height: 12),

                      // Difficulty & Visibility row
                      Row(
                        children: [
                          Expanded(
                            child: DropdownButtonFormField<String>(
                              initialValue: _difficulty,
                              decoration: _inputDecoration(l10n.workoutDifficulty, ''),
                              items: [
                                DropdownMenuItem(value: 'soft', child: Text(l10n.difficultySoft)),
                                DropdownMenuItem(value: 'easy', child: Text(l10n.difficultyEasy)),
                                DropdownMenuItem(
                                  value: 'medium',
                                  child: Text(l10n.difficultyMedium),
                                ),
                                DropdownMenuItem(value: 'hard', child: Text(l10n.difficultyHard)),
                                DropdownMenuItem(
                                  value: 'strong',
                                  child: Text(l10n.difficultyStrong),
                                ),
                              ],
                              onChanged: (v) => setState(() => _difficulty = v ?? 'soft'),
                            ),
                          ),
                          const SizedBox(width: 12),
                          Expanded(
                            child: VisibilityDropdownField(
                              value: _visibility,
                              decoration: _inputDecoration(l10n.workoutVisibility, ''),
                              onChanged: (v) => setState(() => _visibility = v ?? 'Private'),
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 12),

                      // Muscle Groups
                      Text(
                        l10n.workoutMuscleGroups,
                        style: const TextStyle(
                          fontSize: 13,
                          fontWeight: FontWeight.w600,
                          color: Color(0xFF555555),
                        ),
                      ),
                      const SizedBox(height: 4),
                      Wrap(spacing: 8, runSpacing: 4, children: _buildMuscleGroupChips(l10n)),
                    ],
                  ),
                ),
              ),
            ),

            // Actions
            Padding(
              padding: const EdgeInsets.all(16),
              child: Row(
                children: [
                  Expanded(
                    child: OutlinedButton(
                      onPressed: () => Navigator.of(context).pop(),
                      style: OutlinedButton.styleFrom(
                        foregroundColor: AppColors.primary,
                        side: const BorderSide(color: AppColors.primary),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                        padding: const EdgeInsets.symmetric(vertical: 12),
                      ),
                      child: Text(l10n.buttonCancel),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: ElevatedButton(
                      onPressed: isSaveMode
                          ? (workoutProvider.loading ? null : _handleSaveWorkout)
                          : _handleStartSession,
                      style: ElevatedButton.styleFrom(
                        backgroundColor: isSaveMode ? AppColors.primary : AppColors.success,
                        foregroundColor: Colors.white,
                        disabledBackgroundColor: AppColors.primaryDisabled,
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                        padding: const EdgeInsets.symmetric(vertical: 12),
                      ),
                      child: isSaveMode
                          ? (workoutProvider.loading
                                ? const SizedBox(
                                    width: 20,
                                    height: 20,
                                    child: CircularProgressIndicator(
                                      color: Colors.white,
                                      strokeWidth: 2,
                                    ),
                                  )
                                : Text(l10n.buttonConfirm))
                          : Text(l10n.buttonStartSession),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  List<Widget> _buildMuscleGroupChips(AppLocalizations l10n) {
    final groups = {
      'chest': l10n.muscleGroupChest,
      'legs': l10n.muscleGroupLegs,
      'back': l10n.muscleGroupBack,
      'core': l10n.muscleGroupCore,
      'full_body': l10n.muscleGroupFullBody,
      'shoulders': l10n.muscleGroupShoulders,
      'arms': l10n.muscleGroupArms,
    };

    return groups.entries.map((entry) {
      final isSelected = _selectedMuscleGroups.contains(entry.key);
      return FilterChip(
        label: Text(entry.value),
        selected: isSelected,
        onSelected: (selected) {
          setState(() {
            if (selected) {
              _selectedMuscleGroups.add(entry.key);
            } else {
              _selectedMuscleGroups.remove(entry.key);
            }
          });
        },
        selectedColor: AppColors.primary.withAlpha(40),
        checkmarkColor: AppColors.primary,
        labelStyle: TextStyle(
          fontSize: 12,
          color: isSelected ? AppColors.primary : const Color(0xFF555555),
        ),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      );
    }).toList();
  }

  InputDecoration _inputDecoration(String label, String hint) {
    return InputDecoration(
      labelText: label,
      hintText: hint.isNotEmpty ? hint : null,
      contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: AppColors.primary),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: AppColors.primary),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: AppColors.primaryHover, width: 2),
      ),
      isDense: true,
    );
  }
}
