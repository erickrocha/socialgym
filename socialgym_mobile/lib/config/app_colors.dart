import 'package:flutter/material.dart';

class AppColors {
  // Primary colors (matching web: $color-primary: #7CB2A8)
  static const Color primary = Color(0xFF7CB2A8);
  static const Color primaryHover = Color(0xFF6AA298);
  static const Color primaryDisabled = Color(0xFFB0D9D3);

  // Secondary colors (matching web: $color-secondary: #F19A9A)
  static const Color secondary = Color(0xFFF19A9A);
  static const Color secondaryHover = Color(0xFFE67E7E);
  static const Color secondaryDisabled = Color(0xFFF9D6D6);

  // Professional secondary colors (matching web: $color-professional-secondary: #1B1795)
  static const Color professionalSecondary = Color(0xFF1B1795);
  static const Color professionalSecondaryHover = Color(0xFF1A168C);
  static const Color professionalSecondaryDisabled = Color(0xFF3A37B0);

  // Third colors (matching web: $color-third: #F8C491)
  static const Color third = Color(0xFFF8C491);
  static const Color thirdHover = Color(0xFFF5B073);
  static const Color thirdDisabled = Color(0xFFFDE9D9);

  // Status colors
  static const Color success = Color(0xFF4CAF50);
  static const Color danger = Color(0xFFF44336);

  // Background / Foreground
  static const Color background = Color(0xFFF5F5F5);
  static const Color foreground = Color(0xFF050505);

  // Gradient matching web gradient-3
  static const LinearGradient gradient3 = LinearGradient(
    begin: Alignment.centerLeft,
    end: Alignment.centerRight,
    colors: [primary, secondary, third],
    stops: [0.0, 0.5, 1.0],
  );

  // Gradient matching web gradient-3
  static const LinearGradient professionalGradient3 = LinearGradient(
    begin: Alignment.centerLeft,
    end: Alignment.centerRight,
    colors: [primary, professionalSecondary, third],
    stops: [0.0, 0.5, 1.0],
  );
}
