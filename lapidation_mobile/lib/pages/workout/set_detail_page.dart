import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../providers/person_provider.dart';

/// Page to view and edit a single executed set from a workout session
class SetDetailPage extends StatefulWidget {
  final Map<String, dynamic> set;
  final void Function(Map<String, dynamic> updatedSet)? onSave;

  const SetDetailPage({super.key, required this.set, this.onSave});

  @override
  State<SetDetailPage> createState() => _SetDetailPageState();
}

class _SetDetailPageState extends State<SetDetailPage> {
  late double _weight;
  late int _reps;
  late TextEditingController _weightController;
  late TextEditingController _repsController;
  late FocusNode _weightFocus;
  late FocusNode _repsFocus;
  bool _isEditingWeight = false;
  bool _isEditingReps = false;

  @override
  void initState() {
    super.initState();
    _weight = (widget.set['weight'] as num?)?.toDouble() ?? 0.0;
    _reps = (widget.set['repsOrDuration'] as num?)?.toInt() ?? 0;
    _weightController = TextEditingController(text: _weight.toString());
    _repsController = TextEditingController(text: _reps.toString());
    _weightFocus = FocusNode();
    _repsFocus = FocusNode();
  }

  @override
  void dispose() {
    _weightController.dispose();
    _repsController.dispose();
    _weightFocus.dispose();
    _repsFocus.dispose();
    super.dispose();
  }

  void _startEditWeight() {
    setState(() {
      _isEditingWeight = true;
    });
    _weightController.selection = TextSelection(
      baseOffset: 0,
      extentOffset: _weightController.text.length,
    );
    Future.delayed(const Duration(milliseconds: 100), () {
      if (mounted && _weightFocus.canRequestFocus) {
        _weightFocus.requestFocus();
      }
    });
  }

  void _finishEditWeight() {
    final value = double.tryParse(_weightController.text) ?? 0;
    setState(() {
      _weight = value;
      _isEditingWeight = false;
    });
    // Move to reps field
    Future.delayed(const Duration(milliseconds: 100), () {
      _startEditReps();
    });
  }

  void _startEditReps() {
    setState(() {
      _isEditingReps = true;
    });
    _repsController.selection = TextSelection(
      baseOffset: 0,
      extentOffset: _repsController.text.length,
    );
    Future.delayed(const Duration(milliseconds: 100), () {
      if (mounted && _repsFocus.canRequestFocus) {
        _repsFocus.requestFocus();
      }
    });
  }

  void _finishEditReps() {
    final value = int.tryParse(_repsController.text) ?? 0;
    setState(() {
      _reps = value;
      _isEditingReps = false;
    });
  }

  void _save() {
    final updatedSet = Map<String, dynamic>.from(widget.set);
    updatedSet['weight'] = _weight;
    updatedSet['repsOrDuration'] = _reps;
    widget.onSave?.call(updatedSet);
    Navigator.of(context).pop(updatedSet);
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final businessType = context
        .watch<PersonProvider>()
        .activeBusinessProfile
        ?.businessType;

    return Scaffold(
      backgroundColor: AppColors.background,
      appBar: AppBar(
        backgroundColor: AppColors.primaryFor(businessType),
        title: Text('${l10n.executionSet} ${l10n.menuSettings}'),
        elevation: 0,
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Exercise name
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(12),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    l10n.executionExercise,
                    style: const TextStyle(
                      fontSize: 12,
                      color: Color(0xFF999999),
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                  const SizedBox(height: 8),
                  Text(
                    widget.set['exerciseName'] as String? ?? '',
                    style: const TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                      color: Color(0xFF333333),
                    ),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 16),

            // Set information
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(12),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    l10n.workoutSets,
                    style: const TextStyle(
                      fontSize: 14,
                      fontWeight: FontWeight.w600,
                      color: Color(0xFF333333),
                    ),
                  ),
                  const SizedBox(height: 12),
                  Row(
                    children: [
                      Flexible(
                        child: _buildInfoChip(
                          l10n.executionSet,
                          '${widget.set['setNumber'] ?? 0}',
                        ),
                      ),
                      const SizedBox(width: 8),
                      Flexible(
                        child: _buildInfoChip(
                          l10n.exerciseCategory,
                          widget.set['category'] as String? ?? '',
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
            const SizedBox(height: 16),

            // Weight and Reps editing
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(12),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    l10n.workoutWeight,
                    style: const TextStyle(
                      fontSize: 14,
                      fontWeight: FontWeight.w600,
                      color: Color(0xFF333333),
                    ),
                  ),
                  const SizedBox(height: 16),
                  Row(
                    children: [
                      // Weight
                      Expanded(
                        child: _EditValueBox(
                          icon: '🏋️',
                          label: l10n.workoutWeight,
                          isEditing: _isEditingWeight,
                          value: _weight,
                          unit: l10n.workoutWeightUnit,
                          controller: _weightController,
                          focusNode: _weightFocus,
                          onTap: _startEditWeight,
                          onSubmit: _finishEditWeight,
                        ),
                      ),
                      const SizedBox(width: 12),
                      // Reps
                      Expanded(
                        child: _EditValueBox(
                          icon: '🔄',
                          label: l10n.workoutReps,
                          isEditing: _isEditingReps,
                          value: _reps.toDouble(),
                          unit: '',
                          controller: _repsController,
                          focusNode: _repsFocus,
                          onTap: _startEditReps,
                          onSubmit: _finishEditReps,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
            const SizedBox(height: 24),

            // Save button
            SizedBox(
              width: double.infinity,
              child: ElevatedButton.icon(
                onPressed: _save,
                icon: const Icon(Icons.save, size: 20),
                label: Text(l10n.buttonSave),
                style: ElevatedButton.styleFrom(
                  backgroundColor: AppColors.primaryFor(businessType),
                  foregroundColor: Colors.white,
                  padding: const EdgeInsets.symmetric(vertical: 14),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildInfoChip(String label, String value) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 6),
      decoration: BoxDecoration(
        color: const Color(0xFFF0F0F0),
        borderRadius: BorderRadius.circular(6),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            label,
            style: const TextStyle(fontSize: 10, color: Color(0xFF999999)),
          ),
          const SizedBox(height: 2),
          Text(
            value,
            style: const TextStyle(
              fontSize: 13,
              fontWeight: FontWeight.w600,
              color: Color(0xFF333333),
            ),
          ),
        ],
      ),
    );
  }
}

class _EditValueBox extends StatelessWidget {
  final String icon;
  final String label;
  final bool isEditing;
  final double value;
  final String unit;
  final TextEditingController controller;
  final FocusNode focusNode;
  final VoidCallback onTap;
  final VoidCallback onSubmit;

  const _EditValueBox({
    required this.icon,
    required this.label,
    required this.isEditing,
    required this.value,
    required this.unit,
    required this.controller,
    required this.focusNode,
    required this.onTap,
    required this.onSubmit,
  });

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: isEditing ? null : onTap,
      child: Container(
        padding: const EdgeInsets.all(12),
        decoration: BoxDecoration(
          color: const Color(0xFFF8F8F8),
          borderRadius: BorderRadius.circular(12),
        ),
        child: Column(
          children: [
            Text(icon, style: const TextStyle(fontSize: 20)),
            const SizedBox(height: 4),
            Text(
              label,
              style: const TextStyle(fontSize: 11, color: Color(0xFF888888)),
            ),
            const SizedBox(height: 6),
            if (isEditing)
              SizedBox(
                width: 80,
                height: 36,
                child: TextFormField(
                  controller: controller,
                  focusNode: focusNode,
                  keyboardType: TextInputType.number,
                  textAlign: TextAlign.center,
                  autofocus: true,
                  style: const TextStyle(
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                  ),
                  decoration: InputDecoration(
                    contentPadding: const EdgeInsets.symmetric(vertical: 4),
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(8),
                    ),
                    isDense: true,
                  ),
                  onFieldSubmitted: (_) => onSubmit(),
                  onTapOutside: (_) => onSubmit(),
                ),
              )
            else
              Text(
                unit.isNotEmpty
                    ? '${value.toStringAsFixed(value == value.roundToDouble() ? 0 : 1)} $unit'
                    : '${value.round()}',
                style: const TextStyle(
                  fontSize: 22,
                  fontWeight: FontWeight.bold,
                  color: Color(0xFF333333),
                ),
              ),
          ],
        ),
      ),
    );
  }
}
