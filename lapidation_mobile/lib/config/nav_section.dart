import 'package:flutter/material.dart';

/// Top-level navigation sections shown as tabs in [AppHeader].
/// Add new enum values here as the app grows — both the header tabs
/// and the sidebar items are data-driven from this enum.
enum NavSection { home, gallery, workout }

extension NavSectionExt on NavSection {
  /// The root route navigated to when this section tab is tapped.
  String get rootRoute {
    switch (this) {
      case NavSection.home:
        return '/feed';
      case NavSection.gallery:
        return '/gallery';
      case NavSection.workout:
        return '/evolution';
    }
  }

  /// Icon used when the section is NOT active.
  IconData get icon {
    switch (this) {
      case NavSection.home:
        return Icons.home_outlined;
      case NavSection.gallery:
        return Icons.image_outlined;
      case NavSection.workout:
        return Icons.fitness_center_outlined;
    }
  }

  /// Icon used when the section IS active.
  IconData get activeIcon {
    switch (this) {
      case NavSection.home:
        return Icons.home;
      case NavSection.gallery:
        return Icons.image;
      case NavSection.workout:
        return Icons.fitness_center;
    }
  }
}
