import 'package:flutter/material.dart';

class VispColors {
  // Brand / Primary
  static const Color primary = Color(0xFF1B77F4);
  static const Color primaryHoverDark = Color(0xFF77BFEB);
  static const Color primaryHoverLight = Color(0xFF1262C7);

  // Text
  static const Color textDark = Color(0xFFEBEBEB);
  static const Color textLight = Color(0xFF4D4D4D);
  static const Color textMuted = Color(0xFF4D4D4D);

  // Badge / Icon BG
  static const Color badgeBgDark = Color(0xFF222222);
  static const Color badgeFgDark = Color(0xFF4D4D4D);

  // Inverted for light
  static const Color badgeBgLight = Color(0xFFEBEBEB);
  static const Color badgeFgLight = Color(0xFF222222);

  // Borders & Selection
  static const Color borderSelection = Color(0xFF394761);

  // Backgrounds
  static const Color bgDark = Color(0xFF232325);
  static const Color bgLight = Color(0xFFD5D5D4);

  // Icons
  static const Color iconUnselected = Color(0xFF79797B);

  // Standard/Utility Colors
  static const Color blue = Color(0xFF1F3554);
  static const Color purple = Color(0xFF52377F);
  static const Color green = Color(0xFF17592C);
  static const Color teal = Color(0xFF1D4E57);
  static const Color navy = Color(0xFF213452);
}

class VispTheme {
  static ThemeData dark() {
    return ThemeData(
      brightness: Brightness.dark,
      scaffoldBackgroundColor: VispColors.bgDark,
      primaryColor: VispColors.primary,
      colorScheme: const ColorScheme.dark(
        primary: VispColors.primary,
        surface: VispColors.bgDark,
      ),
      textTheme: const TextTheme(
        bodyLarge: TextStyle(color: VispColors.textDark),
        bodyMedium: TextStyle(color: VispColors.textDark),
      ),
      // Custom extensions here
    );
  }

  static ThemeData light() {
    return ThemeData(
      brightness: Brightness.light,
      scaffoldBackgroundColor: VispColors.bgLight,
      primaryColor: VispColors.primary,
      colorScheme: const ColorScheme.light(
        primary: VispColors.primary,
        surface: VispColors.bgLight,
      ),
      textTheme: const TextTheme(
        bodyLarge: TextStyle(color: VispColors.textLight),
        bodyMedium: TextStyle(color: VispColors.textLight),
      ),
    );
  }
}
