import 'package:flutter/material.dart';

/// Lapidation Clinic's warm, neutral visual language.
class AppColors {
  static const Color primary = Color(0xFF8B7355);
  static const Color primaryHover = Color(0xFF725D44);
  static const Color primaryDisabled = Color(0xFFC9BDAE);

  static const Color secondary = Color(0xFF8B7D6A);
  static const Color secondaryHover = Color(0xFF746755);
  static const Color secondaryDisabled = Color(0xFFD4CCC1);

  // Professional profiles share the same restrained, premium accent system.
  static const Color professionalSecondary = Color(0xFF6F6252);
  static const Color professionalSecondaryHover = Color(0xFF594E41);
  static const Color professionalSecondaryDisabled = Color(0xFFB9AFA2);

  // Compatibility names used by the shared feature layer. They map onto
  // Lapidation's existing warm professional palette.
  static const Color professionalPrimary = professionalSecondary;
  static const Color professionalPrimaryHover = professionalSecondaryHover;
  static const Color professionalPrimaryDisabled =
      professionalSecondaryDisabled;

  static const Color companyPrimary = secondary;
  static const Color companyPrimaryHover = secondaryHover;
  static const Color companyPrimaryDisabled = secondaryDisabled;

  static const Color third = Color(0xFFD8CFC0);
  static const Color thirdHover = Color(0xFFC4B7A4);
  static const Color thirdDisabled = Color(0xFFEFE8DC);

  static const Color success = Color(0xFF66745F);
  static const Color danger = Color(0xFF9B5F56);

  static const Color background = Color(0xFFF7F1E8);
  static const Color surface = Color(0xFFFDFBF7);
  static const Color foreground = Color(0xFF2B2723);
  static const Color muted = Color(0xFF8C8378);
  static const Color outline = Color(0xFFD8CFC0);
  static const Color badge = Color(0xFFEFE8DC);
  static const Color darkSurface = Color(0xFF8B7D6A);

  static const LinearGradient gradient3 = LinearGradient(
    begin: Alignment.topLeft,
    end: Alignment.bottomRight,
    colors: [Color(0xFF6F6252), primary, darkSurface],
  );

  static const LinearGradient professionalGradient3 = LinearGradient(
    begin: Alignment.topLeft,
    end: Alignment.bottomRight,
    colors: [Color(0xFF594E41), professionalSecondary, darkSurface],
  );

  static Color primaryFor(String? businessType) => switch (businessType) {
    'Professional' => professionalPrimary,
    'Company' => companyPrimary,
    _ => primary,
  };

  static Color primaryHoverFor(String? businessType) => switch (businessType) {
    'Professional' => professionalPrimaryHover,
    'Company' => companyPrimaryHover,
    _ => primaryHover,
  };

  static Color primaryDisabledFor(String? businessType) =>
      switch (businessType) {
        'Professional' => professionalPrimaryDisabled,
        'Company' => companyPrimaryDisabled,
        _ => primaryDisabled,
      };
}
