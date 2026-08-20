import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:lapidation_mobile/models/exercise.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../providers/person_provider.dart';
import '../../providers/workout_provider.dart';
import '../../widgets/visibility_dropdown_field.dart';

class ExerciseFormDialog extends StatefulWidget {
  final String workoutUuid;
  final int? workoutId;
  final String? workoutVisibility;

  const ExerciseFormDialog({
    super.key,
    this.workoutId,
    required this.workoutUuid,
    this.workoutVisibility,
  });

  @override
  State<ExerciseFormDialog> createState() => _ExerciseFormDialogState();
}

class _ExerciseFormDialogState extends State<ExerciseFormDialog> {
  final _nameController = TextEditingController();
  final _descriptionController = TextEditingController();
  final _setsController = TextEditingController();
  final _repsController = TextEditingController();
  final _nameFocus = FocusNode();
  final _descriptionFocus = FocusNode();
  final _setsFocus = FocusNode();
  final _repsFocus = FocusNode();
  late String _visibility;
  String _category = 'Force';

  final List<Exercise> _exercises = [];

  @override
  void initState() {
    super.initState();
    _visibility = widget.workoutVisibility ?? 'Private';
  }

  @override
  void dispose() {
    _nameController.dispose();
    _descriptionController.dispose();
    _setsController.dispose();
    _repsController.dispose();
    _nameFocus.dispose();
    _descriptionFocus.dispose();
    _setsFocus.dispose();
    _repsFocus.dispose();
    super.dispose();
  }

  void _addExercise() {
    if (_nameController.text.trim().isEmpty ||
        _setsController.text.trim().isEmpty ||
        _repsController.text.trim().isEmpty) {
      return;
    }
    final personProvider = context.read<PersonProvider>();
    final ownerId = personProvider.ownerId;
    final ownerUuid = personProvider.ownerUuid;
    final ownerName = personProvider.ownerName;

    setState(() {
      _exercises.add(Exercise(
        name: _nameController.text.trim(),
        ownerId: ownerId,
        ownerName: ownerName,
        ownerUuid: ownerUuid,
        description: _descriptionController.text.trim(),
        category: _category,
        visibility: _visibility,
        sets: int.tryParse(_setsController.text) ?? 0,
        repsOrDuration: int.tryParse(_repsController.text) ?? 0,
      ));
      _nameController.clear();
      _descriptionController.clear();
      _setsController.clear();
      _repsController.clear();
    });

    // Keep keyboard flow fast for adding multiple exercises.
    FocusScope.of(context).requestFocus(_nameFocus);
  }

  String _getRepOurDuration(AppLocalizations l10n) {
    if (_category == 'Cardio') {
      return l10n.exerciseDurationMin;
    } else {
      return l10n.workoutReps;
    }
  }

  void _removeExercise(int index) {
    setState(() => _exercises.removeAt(index));
  }

  Future<void> _handleSave() async {
    if (_exercises.isEmpty) return;
    final workoutProvider = context.read<WorkoutProvider>();

    final success = await workoutProvider.addExercisesToWorkout(
      widget.workoutUuid,
      _exercises
    );

    if (success && mounted) {
      Navigator.of(context).pop(true);
    }
  }

  bool get _isFormValid =>
      _nameController.text.trim().isEmpty ||
      _setsController.text.trim().isEmpty ||
      _repsController.text.trim().isEmpty;

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final workoutProvider = context.watch<WorkoutProvider>();

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
                      l10n.workoutAddExercise,
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
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    // Error
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
                    Row(
                      children: [
                        Expanded(
                          child: DropdownButtonFormField<String>(
                            initialValue: _category,
                            decoration: _inputDecoration(l10n.exerciseCategory, ''),
                            items: [
                              DropdownMenuItem(value: 'Force', child: Text(l10n.categoryForce)),
                              DropdownMenuItem(value: 'Cardio', child: Text(l10n.categoryCardio)),
                            ],
                            onChanged: (v) => setState(() => _category = v ?? 'Force'),
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

                    // Name
                    TextFormField(
                      controller: _nameController,
                      focusNode: _nameFocus,
                      textInputAction: TextInputAction.next,
                      decoration: _inputDecoration(
                        l10n.workoutExerciseName,
                        l10n.workoutExerciseNamePlaceholder,
                      ),
                      onChanged: (_) => setState(() {}),
                      onFieldSubmitted: (_) {
                        FocusScope.of(context).requestFocus(_descriptionFocus);
                      },
                    ),
                    const SizedBox(height: 10),

                    // Description
                    TextFormField(
                      controller: _descriptionController,
                      focusNode: _descriptionFocus,
                      textInputAction: TextInputAction.next,
                      decoration:
                          _inputDecoration(
                            l10n.workoutExerciseDescription,
                            l10n.workoutExerciseDescriptionPlaceholder,
                          ).copyWith(
                            contentPadding: const EdgeInsets.symmetric(
                              horizontal: 12,
                              vertical: 10,
                            ),
                          ),
                      minLines: 1,
                      maxLines: 1,
                      onFieldSubmitted: (_) {
                        FocusScope.of(context).requestFocus(_setsFocus);
                      },
                    ),
                    const SizedBox(height: 10),

                    // Sets, Reps, Add button row
                    Row(
                      crossAxisAlignment: CrossAxisAlignment.end,
                      children: [
                        Expanded(
                          child: TextFormField(
                            controller: _setsController,
                            focusNode: _setsFocus,
                            keyboardType: TextInputType.number,
                            textInputAction: TextInputAction.next,
                            decoration: _inputDecoration(l10n.workoutSets, ''),
                            onChanged: (_) => setState(() {}),
                            onFieldSubmitted: (_) {
                              FocusScope.of(context).requestFocus(_repsFocus);
                            },
                          ),
                        ),
                        const SizedBox(width: 8),
                        Expanded(
                          child: TextFormField(
                            controller: _repsController,
                            focusNode: _repsFocus,
                            keyboardType: TextInputType.number,
                            textInputAction: TextInputAction.done,
                            decoration: _inputDecoration(_getRepOurDuration(l10n), ''),
                            onChanged: (_) => setState(() {}),
                            onFieldSubmitted: (_) {
                              if (!_isFormValid) {
                                _addExercise();
                              }
                            },
                          ),
                        ),
                        const SizedBox(width: 8),
                        SizedBox(
                          width: 44,
                          height: 44,
                          child: IconButton.filled(
                            onPressed: _isFormValid ? null : _addExercise,
                            icon: const Icon(Icons.add, size: 20),
                            style: IconButton.styleFrom(
                              backgroundColor: AppColors.primary,
                              disabledBackgroundColor: AppColors.primaryDisabled,
                              foregroundColor: Colors.white,
                            ),
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 16),

                    // Added exercises list
                    if (_exercises.isNotEmpty) ...[
                      Text(
                        '${l10n.workoutExercisesList} (${_exercises.length})',
                        style: const TextStyle(
                          fontSize: 14,
                          fontWeight: FontWeight.w600,
                          color: Color(0xFF333333),
                        ),
                      ),
                      const SizedBox(height: 8),
                      ..._exercises.map((ex) {
                        final i = _exercises.indexOf(ex);
                        return Card(
                          margin: const EdgeInsets.only(bottom: 6),
                          child: ListTile(
                            dense: true,
                            title: Text(
                              ex.name,
                              style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 14),
                            ),
                            subtitle: Text(
                              '${ex.sets} ${l10n.workoutSets} × ${ex.repsOrDuration} ${l10n.workoutReps}',
                              style: const TextStyle(fontSize: 12),
                            ),
                            trailing: IconButton(
                              icon: const Icon(Icons.close, size: 18, color: AppColors.danger),
                              onPressed: () => _removeExercise(i),
                            ),
                          ),
                        );
                      }),
                    ],
                  ],
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
                      onPressed: _exercises.isEmpty || workoutProvider.loading ? null : _handleSave,
                      style: ElevatedButton.styleFrom(
                        backgroundColor: AppColors.primary,
                        foregroundColor: Colors.white,
                        disabledBackgroundColor: AppColors.primaryDisabled,
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                        padding: const EdgeInsets.symmetric(vertical: 12),
                      ),
                      child: workoutProvider.loading
                          ? const SizedBox(
                              width: 20,
                              height: 20,
                              child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2),
                            )
                          : Text(l10n.buttonSave),
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
