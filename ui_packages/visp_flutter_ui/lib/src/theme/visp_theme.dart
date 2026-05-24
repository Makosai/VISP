import 'package:flutter/material.dart';
import 'package:shadcn_ui/shadcn_ui.dart';

class VispColors {
  static const Color primary = Color(0xFF1B77F4);
  static const Color primaryHoverDark = Color(0xFF1978FC);
  static const Color primaryHoverLight = Color(0xFF1670DE);
  static const Color textDark = Color(0xFFEBEBEB);
  static const Color textLight = Color(0xFF4D4D4D);
  static const Color textMuted = Color(0xFF7A7A7A);
  static const Color badgeBgDark = Color(0xFF222222);
  static const Color badgeBgLight = Color(0xFFEBEBEB);
  static const Color borderSelection = Color(0xFF394761);
  static const Color bgDark = Color(0xFF232323);
  static const Color bgLight = Color(0xFFD5D5D4);
  static const Color iconUnselected = Color(0xFF7A7A7A);
}

class VispTheme {
  static ShadThemeData dark() {
    return ShadThemeData(
      brightness: Brightness.dark,
      disableSecondaryBorder: true,
      colorScheme: const ShadZincColorScheme.dark(
        primary: VispColors.primary,
        background: VispColors.bgDark,
        foreground: VispColors.textDark,
        muted: VispColors.badgeBgDark,
        mutedForeground: VispColors.textMuted,
        popover: VispColors.bgDark,
        secondary: VispColors.badgeBgDark,
        secondaryForeground: VispColors.textDark,
      ),
      primaryButtonTheme: ShadButtonTheme(
        backgroundColor: VispColors.primary,
        hoverBackgroundColor: VispColors.primaryHoverDark,
        foregroundColor: Colors.white,
        decoration: ShadDecoration(
          border: ShadBorder.all(radius: BorderRadius.circular(10)),
        ),
      ),
      menubarTheme: const ShadMenubarTheme(
        radius: BorderRadius.all(Radius.circular(10)),
        border: ShadBorder.none,
        backgroundColor: Colors.transparent,
      ),
      selectTheme: ShadSelectTheme(
        decoration: ShadDecoration(
          color: VispColors.badgeBgDark,
          border: ShadBorder.all(color: VispColors.borderSelection, width: 0.5),
        ),
      ),
    );
  }

  static ShadThemeData light() {
    return ShadThemeData(
      brightness: Brightness.light,
      disableSecondaryBorder: true,
      colorScheme: const ShadZincColorScheme.light(
        primary: VispColors.primary,
        background: VispColors.bgLight,
        foreground: VispColors.textLight,
        muted: VispColors.badgeBgLight,
        mutedForeground: VispColors.textMuted,
        border: VispColors.borderSelection,
        popover: VispColors.bgLight,
        secondary: VispColors.badgeBgLight,
        secondaryForeground: VispColors.textLight,
      ),
      primaryButtonTheme: ShadButtonTheme(
        backgroundColor: VispColors.primary,
        hoverBackgroundColor: VispColors.primaryHoverLight,
        foregroundColor: Colors.white,
        decoration: ShadDecoration(
          border: ShadBorder.all(radius: BorderRadius.circular(4)),
        ),
      ),
    );
  }
}
