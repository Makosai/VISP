import 'package:flutter/material.dart';
import 'package:shadcn_ui/shadcn_ui.dart';

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
  static ShadThemeData dark() {
    return ShadThemeData(
      brightness: Brightness.dark,
      colorScheme: const ShadZincColorScheme.dark(
        primary: VispColors.primary,
        background: VispColors.bgDark,
        foreground: VispColors.textDark,
        muted: VispColors.badgeBgDark,
        mutedForeground: VispColors.textMuted,
        border: VispColors.borderSelection,
        secondary: VispColors.badgeBgDark,
        secondaryForeground: VispColors.textDark,
      ),
      primaryButtonTheme: ShadButtonTheme(
        backgroundColor: VispColors.primary,
        hoverBackgroundColor: VispColors.primaryHoverDark,
        foregroundColor: Colors.white,
        decoration: ShadDecoration(
          border: ShadBorder.all(
            radius: const BorderRadius.all(Radius.circular(4)),
          ),
        ),
      ),
    );
  }

  static ShadThemeData light() {
    return ShadThemeData(
      brightness: Brightness.light,
      colorScheme: const ShadZincColorScheme.light(
        primary: VispColors.primary,
        background: VispColors.bgLight,
        foreground: VispColors.textLight,
        muted: VispColors.badgeBgLight,
        mutedForeground: VispColors.textMuted,
        border: VispColors.borderSelection,
        secondary: VispColors.badgeBgLight,
        secondaryForeground: VispColors.textLight,
      ),
      primaryButtonTheme: ShadButtonTheme(
        backgroundColor: VispColors.primary,
        hoverBackgroundColor: VispColors.primaryHoverLight,
        foregroundColor: Colors.white,
        decoration: ShadDecoration(
          border: ShadBorder.all(
            radius: const BorderRadius.all(Radius.circular(4)),
          ),
        ),
      ),
    );
  }
}
