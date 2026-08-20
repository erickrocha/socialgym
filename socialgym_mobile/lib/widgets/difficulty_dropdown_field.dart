import 'package:flutter/material.dart';

import '../l10n/app_localizations.dart';
import '../models/difficulty_option.dart';

class DifficultyDropdownField extends StatelessWidget {
  final String value;
  final ValueChanged<String?> onChanged;
  final InputDecoration? decoration;
  final bool isExpanded;
  final DifficultyOption fallback;

  const DifficultyDropdownField({
    super.key,
    required this.value,
    required this.onChanged,
    this.decoration,
    this.isExpanded = true,
    this.fallback = DifficultyOption.soft,
  });

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final selectedDifficulty = DifficultyOption.fromApiValue(value, fallback: fallback);

    return DropdownButtonFormField<DifficultyOption>(
      key: ValueKey<String>(selectedDifficulty.apiValue),
      initialValue: selectedDifficulty,
      isExpanded: isExpanded,
      decoration: decoration ?? InputDecoration(labelText: l10n.workoutDifficulty),
      items: DifficultyOption.values.map((option) {
        return DropdownMenuItem<DifficultyOption>(
          value: option,
          child: Text('${option.emoji} ${option.label(l10n)}', overflow: TextOverflow.ellipsis),
        );
      }).toList(),
      onChanged: (option) => onChanged(option?.apiValue),
    );
  }
}
