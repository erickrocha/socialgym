import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../config/app_colors.dart';
import '../../l10n/app_localizations.dart';
import '../../providers/auth_provider.dart';
import '../../providers/evolution_provider.dart';
import '../../providers/person_provider.dart';
import '../../widgets/visibility_dropdown_field.dart';

class EvolutionCheckInFormDialog extends StatefulWidget {
  final String personUuid;

  const EvolutionCheckInFormDialog({
    super.key,
    required this.personUuid,
    required AppLocalizations l10n,
  });

  @override
  State<EvolutionCheckInFormDialog> createState() => _EvolutionCheckInFormDialogState();
}

class _EvolutionCheckInFormDialogState extends State<EvolutionCheckInFormDialog> {
  final _formKey = GlobalKey<FormState>();

  final _noteController = TextEditingController();
  final _weightController = TextEditingController();
  final _bodyFatController = TextEditingController();
  final _muscleMassController = TextEditingController();
  final _visceralFatController = TextEditingController();
  final _neckController = TextEditingController();
  final _chestController = TextEditingController();
  final _waistController = TextEditingController();
  final _abdomenController = TextEditingController();
  final _hipController = TextEditingController();
  final _bicepsRightController = TextEditingController();
  final _bicepsLeftController = TextEditingController();
  final _thighRightController = TextEditingController();
  final _thighLeftController = TextEditingController();

  // Focus nodes for keyboard navigation
  late FocusNode _noteFocus;
  late FocusNode _weightFocus;
  late FocusNode _bodyFatFocus;
  late FocusNode _muscleMassFocus;
  late FocusNode _visceralFatFocus;
  late FocusNode _neckFocus;
  late FocusNode _chestFocus;
  late FocusNode _waistFocus;
  late FocusNode _abdomenFocus;
  late FocusNode _hipFocus;
  late FocusNode _bicepsRightFocus;
  late FocusNode _bicepsLeftFocus;
  late FocusNode _thighRightFocus;
  late FocusNode _thighLeftFocus;

  String _visibility = 'Private';

  @override
  void initState() {
    super.initState();
    _noteFocus = FocusNode();
    _weightFocus = FocusNode();
    _bodyFatFocus = FocusNode();
    _muscleMassFocus = FocusNode();
    _visceralFatFocus = FocusNode();
    _neckFocus = FocusNode();
    _chestFocus = FocusNode();
    _waistFocus = FocusNode();
    _abdomenFocus = FocusNode();
    _hipFocus = FocusNode();
    _bicepsRightFocus = FocusNode();
    _bicepsLeftFocus = FocusNode();
    _thighRightFocus = FocusNode();
    _thighLeftFocus = FocusNode();
  }

  @override
  void dispose() {
    _noteController.dispose();
    _weightController.dispose();
    _bodyFatController.dispose();
    _muscleMassController.dispose();
    _visceralFatController.dispose();
    _neckController.dispose();
    _chestController.dispose();
    _waistController.dispose();
    _abdomenController.dispose();
    _hipController.dispose();
    _bicepsRightController.dispose();
    _bicepsLeftController.dispose();
    _thighRightController.dispose();
    _thighLeftController.dispose();

    _noteFocus.dispose();
    _weightFocus.dispose();
    _bodyFatFocus.dispose();
    _muscleMassFocus.dispose();
    _visceralFatFocus.dispose();
    _neckFocus.dispose();
    _chestFocus.dispose();
    _waistFocus.dispose();
    _abdomenFocus.dispose();
    _hipFocus.dispose();
    _bicepsRightFocus.dispose();
    _bicepsLeftFocus.dispose();
    _thighRightFocus.dispose();
    _thighLeftFocus.dispose();
    super.dispose();
  }

  double? _parseOptionalDouble(String value) {
    final trimmed = value.trim();
    if (trimmed.isEmpty) return null;
    return double.tryParse(trimmed.replaceAll(',', '.'));
  }

  bool _hasAtLeastOneMetric() {
    return [
      _weightController,
      _bodyFatController,
      _muscleMassController,
      _visceralFatController,
      _neckController,
      _chestController,
      _waistController,
      _abdomenController,
      _hipController,
      _bicepsRightController,
      _bicepsLeftController,
      _thighRightController,
      _thighLeftController,
    ].any((c) => c.text.trim().isNotEmpty);
  }

  Future<void> _handleSave() async {
    if (!_formKey.currentState!.validate()) return;

    final l10n = AppLocalizations.of(context)!;
    final authProvider = context.read<AuthProvider>();
    final person = context.read<PersonProvider>().person;
    final evolutionProvider = context.read<EvolutionProvider>();
    final token = authProvider.auth?.accessToken ?? '';

    if (!_hasAtLeastOneMetric()) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text(l10n.evolutionAtLeastOneMetric)));
      return;
    }

    final composition = <String, dynamic>{
      'weight': _parseOptionalDouble(_weightController.text),
      'bodyFatPct': _parseOptionalDouble(_bodyFatController.text),
      'muscleMassPct': _parseOptionalDouble(_muscleMassController.text),
      'visceralFat': _parseOptionalDouble(_visceralFatController.text),
    }..removeWhere((_, value) => value == null);

    final circumferences = <String, dynamic>{
      'neck': _parseOptionalDouble(_neckController.text),
      'chest': _parseOptionalDouble(_chestController.text),
      'waist': _parseOptionalDouble(_waistController.text),
      'abdomen': _parseOptionalDouble(_abdomenController.text),
      'hip': _parseOptionalDouble(_hipController.text),
      'bicepsRight': _parseOptionalDouble(_bicepsRightController.text),
      'bicepsLeft': _parseOptionalDouble(_bicepsLeftController.text),
      'thighRight': _parseOptionalDouble(_thighRightController.text),
      'thighLeft': _parseOptionalDouble(_thighLeftController.text),
    }..removeWhere((_, value) => value == null);

    final payload = <String, dynamic>{
      'personUuid': person?.uuid,
      'createdAt': DateTime.now().toIso8601String(),
      'note': _noteController.text.trim(),
      'visibility': _visibility,
      if (composition.isNotEmpty) 'composition': composition,
      if (circumferences.isNotEmpty) 'circumferences': circumferences,
    };

    final success = await evolutionProvider.addCheckIn(payload: payload, token: token);

    if (success && mounted) {
      Navigator.of(context).pop(true);
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final provider = context.watch<EvolutionProvider>();
    final businessType = context.watch<PersonProvider>().activeBusinessProfile?.businessType;

    return Dialog(
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      insetPadding: const EdgeInsets.all(16),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 560, maxHeight: 760),
        child: Column(
          children: [
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
                      l10n.evolutionAddCheckin,
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
            Flexible(
              child: SingleChildScrollView(
                padding: const EdgeInsets.all(16),
                child: Form(
                  key: _formKey,
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      if (provider.error != null) ...[
                        Container(
                          padding: const EdgeInsets.all(12),
                          decoration: BoxDecoration(
                            color: AppColors.danger.withAlpha(25),
                            borderRadius: BorderRadius.circular(8),
                            border: Border.all(color: AppColors.danger.withAlpha(76)),
                          ),
                          child: Text(
                            provider.error!,
                            style: const TextStyle(color: AppColors.danger, fontSize: 13),
                            textAlign: TextAlign.center,
                          ),
                        ),
                        const SizedBox(height: 12),
                      ],
                      TextFormField(
                        controller: _noteController,
                        focusNode: _noteFocus,
                        decoration: _inputDecoration(
                          l10n.evolutionFormNote,
                          l10n.evolutionFormNotePlaceholder,
                          businessType,
                        ),
                        maxLines: 2,
                        textInputAction: TextInputAction.next,
                        onFieldSubmitted: (_) => _weightFocus.requestFocus(),
                      ),
                      const SizedBox(height: 12),
                      VisibilityDropdownField(
                        value: _visibility,
                        decoration: _inputDecoration(l10n.evolutionFormVisibility, '', businessType),
                        onChanged: (value) => setState(() => _visibility = value ?? 'Private'),
                      ),
                      const SizedBox(height: 16),
                      Text(
                        l10n.evolutionSectionComposition,
                        style: const TextStyle(
                          fontWeight: FontWeight.bold,
                          color: Color(0xFF333333),
                        ),
                      ),
                      const SizedBox(height: 8),
                      _buildNumberField(
                        controller: _weightController,
                        label: '${l10n.evolutionCompositionWeight} (kg)',
                        focusNode: _weightFocus,
                        nextFocus: _bodyFatFocus,
                        businessType: businessType,
                      ),
                      _buildNumberField(
                        controller: _bodyFatController,
                        label: '${l10n.evolutionCompositionBodyFat} (%)',
                        focusNode: _bodyFatFocus,
                        nextFocus: _muscleMassFocus,
                        businessType: businessType,
                      ),
                      _buildNumberField(
                        controller: _muscleMassController,
                        label: '${l10n.evolutionCompositionMuscleMass} (%)',
                        focusNode: _muscleMassFocus,
                        nextFocus: _visceralFatFocus,
                        businessType: businessType,
                      ),
                      _buildNumberField(
                        controller: _visceralFatController,
                        label: '${l10n.evolutionCompositionVisceralFat} (%)',
                        focusNode: _visceralFatFocus,
                        nextFocus: _neckFocus,
                        businessType: businessType,
                      ),
                      const SizedBox(height: 12),
                      Text(
                        l10n.evolutionSectionCircumferences,
                        style: const TextStyle(
                          fontWeight: FontWeight.bold,
                          color: Color(0xFF333333),
                        ),
                      ),
                      const SizedBox(height: 8),
                      _buildNumberField(
                        controller: _neckController,
                        label: '${l10n.evolutionCircumferenceNeck} (cm)',
                        focusNode: _neckFocus,
                        nextFocus: _chestFocus,
                        businessType: businessType,
                      ),
                      _buildNumberField(
                        controller: _chestController,
                        label: '${l10n.evolutionCircumferenceChest} (cm)',
                        focusNode: _chestFocus,
                        nextFocus: _waistFocus,
                        businessType: businessType,
                      ),
                      _buildNumberField(
                        controller: _waistController,
                        label: '${l10n.evolutionCircumferenceWaist} (cm)',
                        focusNode: _waistFocus,
                        nextFocus: _abdomenFocus,
                        businessType: businessType,
                      ),
                      _buildNumberField(
                        controller: _abdomenController,
                        label: '${l10n.evolutionCircumferenceAbdomen} (cm)',
                        focusNode: _abdomenFocus,
                        nextFocus: _hipFocus,
                        businessType: businessType,
                      ),
                      _buildNumberField(
                        controller: _hipController,
                        label: '${l10n.evolutionCircumferenceHip} (cm)',
                        focusNode: _hipFocus,
                        nextFocus: _bicepsRightFocus,
                        businessType: businessType,
                      ),
                      _buildNumberField(
                        controller: _bicepsRightController,
                        label: '${l10n.evolutionCircumferenceBicepsRight} (cm)',
                        focusNode: _bicepsRightFocus,
                        nextFocus: _bicepsLeftFocus,
                        businessType: businessType,
                      ),
                      _buildNumberField(
                        controller: _bicepsLeftController,
                        label: '${l10n.evolutionCircumferenceBicepsLeft} (cm)',
                        focusNode: _bicepsLeftFocus,
                        nextFocus: _thighRightFocus,
                        businessType: businessType,
                      ),
                      _buildNumberField(
                        controller: _thighRightController,
                        label: '${l10n.evolutionCircumferenceThighRight} (cm)',
                        focusNode: _thighRightFocus,
                        nextFocus: _thighLeftFocus,
                        businessType: businessType,
                      ),
                      _buildNumberField(
                        controller: _thighLeftController,
                        label: '${l10n.evolutionCircumferenceThighLeft} (cm)',
                        focusNode: _thighLeftFocus,
                        businessType: businessType,
                      ),
                    ],
                  ),
                ),
              ),
            ),
            Padding(
              padding: const EdgeInsets.all(16),
              child: Row(
                children: [
                  Expanded(
                    child: OutlinedButton(
                      onPressed: provider.submitting ? null : () => Navigator.of(context).pop(),
                      style: OutlinedButton.styleFrom(
                        foregroundColor: AppColors.primaryFor(businessType),
                        side: BorderSide(color: AppColors.primaryFor(businessType)),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                        padding: const EdgeInsets.symmetric(vertical: 12),
                      ),
                      child: Text(l10n.buttonCancel),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: ElevatedButton(
                      onPressed: provider.submitting ? null : _handleSave,
                      style: ElevatedButton.styleFrom(
                        backgroundColor: AppColors.primaryFor(businessType),
                        foregroundColor: Colors.white,
                        disabledBackgroundColor: AppColors.primaryDisabledFor(businessType),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                        padding: const EdgeInsets.symmetric(vertical: 12),
                      ),
                      child: provider.submitting
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

  Widget _buildNumberField({
    required TextEditingController controller,
    required String label,
    required FocusNode focusNode,
    FocusNode? nextFocus,
    String? businessType,
  }) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 10),
      child: TextFormField(
        controller: controller,
        focusNode: focusNode,
        keyboardType: const TextInputType.numberWithOptions(decimal: true),
        decoration: _inputDecoration(label, '', businessType),
        textInputAction: nextFocus != null ? TextInputAction.next : TextInputAction.done,
        onFieldSubmitted: (_) {
          if (nextFocus != null) {
            nextFocus.requestFocus();
          }
        },
        validator: (value) {
          if (value == null || value.trim().isEmpty) return null;
          return _parseOptionalDouble(value) == null
              ? AppLocalizations.of(context)!.validationRequired
              : null;
        },
      ),
    );
  }

  InputDecoration _inputDecoration(String label, String hint, String? businessType) {
    return InputDecoration(
      labelText: label,
      hintText: hint.isNotEmpty ? hint : null,
      contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: BorderSide(color: AppColors.primaryFor(businessType)),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: BorderSide(color: AppColors.primaryFor(businessType)),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: BorderSide(color: AppColors.primaryHoverFor(businessType), width: 2),
      ),
      isDense: true,
    );
  }
}
