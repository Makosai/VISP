import 'package:flutter/material.dart';
import 'package:shadcn_ui/shadcn_ui.dart';

class VispColors {
  // Brand / Primary
  static const Color primary = Color(0xFF1B77F4);
  static const Color primaryHoverDark = Color(0xFF1978FC);
  static const Color primaryHoverLight = Color(0xFF1670DE);

  // Text
  static const Color textDark = Color(0xFFEBEBEB);
  static const Color textLight = Color(0xFF4D4D4D);
  static const Color textMuted = Color(0xFF7A7A7A);

  // Badge / Icon BG
  static const Color badgeBgDark = Color(0xFF222222);
  static const Color badgeBgLight = Color(0xFFEBEBEB);
  static const Color borderSelection = Color(0xFF394761);
  static const Color borderNormal = Color(0x407A7A7A);
  static const Color bgDark = Color(0xFF232323);
  static const Color bgLight = Color(0xFFD5D5D4);

  // Icons
  static const Color iconUnselected = Color(0xFF7A7A7A);

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
      ),
      radius: BorderRadius.all(Radius.circular(10)),
      menubarTheme: ShadMenubarTheme(
        backgroundColor: Colors.transparent,
        padding: EdgeInsets.zero,
        decoration: ShadDecoration(
          border: ShadBorder.all(color: VispColors.borderNormal, width: 0.5),
        ),
      ),
      selectTheme: ShadSelectTheme(
        decoration: ShadDecoration(
          color: VispColors.badgeBgDark,
          border: ShadBorder.all(
            color: VispColors.borderNormal,
            width: 0.5,
            radius: BorderRadius.circular(10),
          ),
          focusedBorder: ShadBorder.all(
            color: VispColors.borderNormal,
            width: 0.5,
            radius: BorderRadius.circular(10),
          ),
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
        foregroundColor: VispColors.textMuted,
        hoverForegroundColor: VispColors.textLight,
      ),
      outlineButtonTheme: ShadButtonTheme(
        backgroundColor: VispColors.primary,
        foregroundColor: VispColors.textMuted,
        hoverForegroundColor: VispColors.textLight,
      ),
    );
  }
}
