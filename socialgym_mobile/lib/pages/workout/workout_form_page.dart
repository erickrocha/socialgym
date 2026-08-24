import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../models/person.dart';
import '../../models/workout.dart';
import '../../providers/person_provider.dart';
import '../../providers/workout_provider.dart';
import '../../widgets/difficulty_dropdown_field.dart';
import '../../widgets/team/team_member_picker_field.dart';
import '../../widgets/visibility_dropdown_field.dart';

/// Full-page "create a new workout" flow (replaces the old modal for this
/// entry point). Editing an existing workout, and the specialized
/// save-from-selection/quick-start flows on the Exercises page, still use
/// [WorkoutFormDialog].
class WorkoutFormPage extends StatefulWidget {
  final int ownerId;
  final String ownerUuid;

  const WorkoutFormPage({super.key, required this.ownerId, required this.ownerUuid});

  @override
  State<WorkoutFormPage> createState() => _WorkoutFormPageState();
}

class _WorkoutFormPageState extends State<WorkoutFormPage> {
  final _formKey = GlobalKey<FormState>();
  final _nameController = TextEditingController();
  final _descriptionController = TextEditingController();
  final _nameFocus = FocusNode();
  final _descriptionFocus = FocusNode();
  String _difficulty = 'soft';
  String _visibility = 'Private';
  final Set<String> _selectedMuscleGroups = {};
  Person? _assignTo;

  @override
  void dispose() {
    _nameController.dispose();
    _descriptionController.dispose();
    _nameFocus.dispose();
    _descriptionFocus.dispose();
    super.dispose();
  }

  Future<void> _handleSave() async {
    if (!_formKey.currentState!.validate()) return;

    final workoutProvider = context.read<WorkoutProvider>();
    final workoutData = Workout(
      ownerId: widget.ownerId,
      ownerUuid: widget.ownerUuid,
      name: _nameController.text.trim(),
      description: _descriptionController.text.trim(),
      difficulty: _difficulty,
      muscleGroup: _selectedMuscleGroups.join('|'),
      visibility: _visibility,
      exercises: const [],
    );

    final success = await workoutProvider.addWorkout(
      workoutData,
      targetPersonUuid: _assignTo?.uuid,
    );
    if (success && mounted) {
      Navigator.of(context).pop(true);
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final workoutProvider = context.watch<WorkoutProvider>();
    final personProvider = context.watch<PersonProvider>();
    final businessProfileId = personProvider.activeBusinessProfile?.id;

    return Scaffold(
      appBar: AppBar(
        title: Text(l10n.workoutAddNew),
        backgroundColor: AppColors.primary,
        foregroundColor: Colors.white,
      ),
      body: SafeArea(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(16),
          child: Form(
            key: _formKey,
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                if (workoutProvider.error != null) ...[
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
                    if (!workoutProvider.loading) _handleSave();
                  },
                ),
                const SizedBox(height: 12),

                Row(
                  children: [
                    Expanded(
                      child: DifficultyDropdownField(
                        value: _difficulty,
                        decoration: _inputDecoration(l10n.workoutDifficulty, ''),
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

                if (personProvider.isProfessional && businessProfileId != null) ...[
                  TeamMemberPickerField(
                    businessProfileId: businessProfileId,
                    selected: _assignTo,
                    onChanged: (person) => setState(() => _assignTo = person),
                    decoration: _inputDecoration(l10n.workoutAssignToTeamMember, ''),
                  ),
                  const SizedBox(height: 12),
                ],

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
                const SizedBox(height: 24),

                ElevatedButton(
                  onPressed: workoutProvider.loading ? null : _handleSave,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: AppColors.primary,
                    foregroundColor: Colors.white,
                    disabledBackgroundColor: AppColors.primaryDisabled,
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                    padding: const EdgeInsets.symmetric(vertical: 14),
                  ),
                  child: workoutProvider.loading
                      ? const SizedBox(
                          width: 20,
                          height: 20,
                          child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2),
                        )
                      : Text(l10n.buttonSave),
                ),
              ],
            ),
          ),
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
