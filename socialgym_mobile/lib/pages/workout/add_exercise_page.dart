import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:socialgym_mobile/models/exercise.dart';
import 'package:socialgym_mobile/services/grpc/grpc_exercise_service.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../providers/auth_provider.dart';
import '../../providers/exercise_selection_provider.dart';
import '../../providers/person_provider.dart';
import '../../services/base_service.dart';
import '../../widgets/visibility_dropdown_field.dart';

/// Full-page "create a new exercise" flow, reached from the Exercises page.
/// Unrelated to `ExerciseFormDialog`, which adds exercises to an existing
/// workout.
class AddExercisePage extends StatefulWidget {
  final String? workoutVisibility;

  const AddExercisePage({super.key, this.workoutVisibility});

  @override
  State<AddExercisePage> createState() => _AddExercisePageState();
}

class _AddExercisePageState extends State<AddExercisePage> {
  final _nameController = TextEditingController();
  final _descriptionController = TextEditingController();
  final _setsController = TextEditingController();
  final _repsController = TextEditingController();
  final _nameFocus = FocusNode();
  final _descriptionFocus = FocusNode();
  final _setsFocus = FocusNode();
  final _repsFocus = FocusNode();

  String _category = 'Force';
  late String _visibility;
  bool _isLoading = false;
  String? _errorMessage;

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
    super.dispose();
  }

  String _getRepOrDurationLabel(AppLocalizations l10n) {
    return _category == 'Cardio' ? l10n.exerciseDurationMin : l10n.workoutReps;
  }

  bool get _isFormValid =>
      _nameController.text.trim().isNotEmpty &&
      _setsController.text.trim().isNotEmpty &&
      _repsController.text.trim().isNotEmpty;

  Future<void> _handleSave() async {
    if (!_isFormValid) return;

    final authProvider = context.read<AuthProvider>();
    final personProvider = context.read<PersonProvider>();
    final exerciseProvider = context.read<ExerciseSelectionProvider>();

    final token = authProvider.auth?.accessToken ?? '';
    final ownerId = personProvider.activeAuthorId;
    final ownerUuid = personProvider.activeAuthorUuid;
    final ownerName = personProvider.activeAuthorName;

    if (token.isEmpty) {
      setState(() {
        _errorMessage = AppLocalizations.of(context)!.exerciseUserDataUnavailable;
      });
      return;
    }

    setState(() {
      _isLoading = true;
      _errorMessage = null;
    });

    try {
      final exerciseData = Exercise(
        ownerId: ownerId,
        ownerUuid: ownerUuid,
        ownerName: ownerName,
        name: _nameController.text.trim(),
        description: _descriptionController.text.trim().isEmpty ? null : _descriptionController.text.trim(),
        category: _category,
        visibility: _visibility,
        sets: int.tryParse(_setsController.text) ?? 0,
        repsOrDuration: int.tryParse(_repsController.text) ?? 0,
      );

      final exercise = await GrpcExerciseService.addExercise( exercise: exerciseData);

      // Add exercise to list (optimistic UI confirmation)
      if (mounted) {
        exerciseProvider.addExercise(exercise);
        Navigator.of(context).pop(true);

        // Show success message
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(AppLocalizations.of(context)!.exerciseCreateSuccess(exercise.name)),
            backgroundColor: AppColors.success,
            duration: const Duration(seconds: 2),
          ),
        );
      }
    } on AppException catch (e) {
      if (mounted) {
        setState(() {
          _isLoading = false;
          _errorMessage = e.message;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _isLoading = false;
          _errorMessage = AppLocalizations.of(context)!.exerciseCreateFailed;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return Scaffold(
      appBar: AppBar(
        title: Text(l10n.workoutAddExercise),
        backgroundColor: AppColors.primary,
        foregroundColor: Colors.white,
      ),
      body: SafeArea(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              // Error message
              if (_errorMessage != null) ...[
                Container(
                  padding: const EdgeInsets.all(12),
                  decoration: BoxDecoration(
                    color: AppColors.danger.withAlpha(25),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: AppColors.danger.withAlpha(76)),
                  ),
                  child: Text(
                    _errorMessage!,
                    style: const TextStyle(color: AppColors.danger, fontSize: 13),
                  ),
                ),
                const SizedBox(height: 12),
              ],

              // Category and Visibility
              Row(
                children: [
                  Expanded(
                    child: DropdownButtonFormField<String>(
                      isExpanded: true,
                      initialValue: _category,
                      decoration: _inputDecoration(l10n.exerciseCategory, ''),
                      items: [
                        DropdownMenuItem(
                          value: 'Force',
                          child: Text(l10n.categoryForce, overflow: TextOverflow.ellipsis),
                        ),
                        DropdownMenuItem(
                          value: 'Cardio',
                          child: Text(l10n.categoryCardio, overflow: TextOverflow.ellipsis),
                        ),
                      ],
                      onChanged: (v) => setState(() => _category = v ?? 'Force'),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: VisibilityDropdownField(
                      value: _visibility,
                      isExpanded: true,
                      useFriendsOnlyAlias: true,
                      decoration: _inputDecoration(l10n.workoutVisibility, ''),
                      onChanged: (v) => setState(() => _visibility = v ?? 'Private'),
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 12),

              // Exercise Name
              TextFormField(
                controller: _nameController,
                focusNode: _nameFocus,
                textInputAction: TextInputAction.next,
                decoration: _inputDecoration(
                  l10n.workoutExerciseName,
                  l10n.workoutExerciseNamePlaceholder,
                ),
                enabled: !_isLoading,
                onChanged: (_) => setState(() {}),
                onFieldSubmitted: (_) {
                  FocusScope.of(context).requestFocus(_descriptionFocus);
                },
              ),
              const SizedBox(height: 12),

              // Description
              TextFormField(
                controller: _descriptionController,
                focusNode: _descriptionFocus,
                textInputAction: TextInputAction.next,
                decoration: _inputDecoration(
                  l10n.workoutExerciseDescription,
                  l10n.workoutExerciseDescriptionPlaceholder,
                ).copyWith(
                  contentPadding: const EdgeInsets.symmetric(
                    horizontal: 12,
                    vertical: 10,
                  ),
                ),
                enabled: !_isLoading,
                minLines: 1,
                maxLines: 1,
                onFieldSubmitted: (_) {
                  FocusScope.of(context).requestFocus(_setsFocus);
                },
              ),
              const SizedBox(height: 12),

              // Sets and Reps/Duration
              Row(
                spacing: 12,
                children: [
                  Expanded(
                    child: TextFormField(
                      controller: _setsController,
                      focusNode: _setsFocus,
                      keyboardType: TextInputType.number,
                      textInputAction: TextInputAction.next,
                      decoration: _inputDecoration(l10n.workoutSets, ''),
                      enabled: !_isLoading,
                      onChanged: (_) => setState(() {}),
                      onFieldSubmitted: (_) {
                        FocusScope.of(context).requestFocus(_repsFocus);
                      },
                    ),
                  ),
                  Expanded(
                    child: TextFormField(
                      controller: _repsController,
                      focusNode: _repsFocus,
                      keyboardType: TextInputType.number,
                      textInputAction: TextInputAction.done,
                      decoration: _inputDecoration(_getRepOrDurationLabel(l10n), ''),
                      enabled: !_isLoading,
                      onChanged: (_) => setState(() {}),
                      onFieldSubmitted: (_) {
                        if (_isFormValid) {
                          _handleSave();
                        }
                      },
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 24),

              ElevatedButton(
                onPressed: !_isFormValid || _isLoading ? null : _handleSave,
                style: ElevatedButton.styleFrom(
                  backgroundColor: AppColors.primary,
                  foregroundColor: Colors.white,
                  disabledBackgroundColor: AppColors.primaryDisabled,
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                  padding: const EdgeInsets.symmetric(vertical: 14),
                ),
                child: _isLoading
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
      disabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: Colors.grey),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: AppColors.primaryHover, width: 2),
      ),
      isDense: true,
    );
  }
}
