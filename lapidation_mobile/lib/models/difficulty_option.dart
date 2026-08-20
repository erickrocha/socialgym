import 'package:flutter/material.dart';

import '../l10n/app_localizations.dart';

enum DifficultyOption {
  soft,
  easy,
  medium,
  hard,
  strong;

  String get apiValue {
    switch (this) {
      case DifficultyOption.soft:
        return 'soft';
      case DifficultyOption.easy:
        return 'easy';
      case DifficultyOption.medium:
        return 'medium';
      case DifficultyOption.hard:
        return 'hard';
      case DifficultyOption.strong:
        return 'strong';
    }
  }

  static DifficultyOption fromApiValue(
    String? value, {
    DifficultyOption fallback = DifficultyOption.soft,
  }) {
    switch (value?.trim().toLowerCase()) {
      case 'soft':
        return DifficultyOption.soft;
      case 'easy':
        return DifficultyOption.easy;
      case 'medium':
        return DifficultyOption.medium;
      case 'hard':
        return DifficultyOption.hard;
      case 'strong':
        return DifficultyOption.strong;
      default:
        return fallback;
    }
  }

  String label(AppLocalizations l10n) {
    switch (this) {
      case DifficultyOption.soft:
        return l10n.difficultySoft;
      case DifficultyOption.easy:
        return l10n.difficultyEasy;
      case DifficultyOption.medium:
        return l10n.difficultyMedium;
      case DifficultyOption.hard:
        return l10n.difficultyHard;
      case DifficultyOption.strong:
        return l10n.difficultyStrong;
    }
  }

  String get emoji {
    switch (this) {
      case DifficultyOption.soft:
        return '🟢';
      case DifficultyOption.easy:
        return '🟡';
      case DifficultyOption.medium:
        return '🟠';
      case DifficultyOption.hard:
        return '🔴';
      case DifficultyOption.strong:
        return '🔥';
    }
  }

  Color get color {
    switch (this) {
      case DifficultyOption.soft:
        return const Color(0xFF4CAF50);
      case DifficultyOption.easy:
        return const Color(0xFF8BC34A);
      case DifficultyOption.medium:
        return const Color(0xFFFFC107);
      case DifficultyOption.hard:
        return const Color(0xFFFF9800);
      case DifficultyOption.strong:
        return const Color(0xFFF44336);
    }
  }
}
