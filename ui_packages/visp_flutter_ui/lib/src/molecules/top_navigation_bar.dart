import 'package:flutter/material.dart';
import 'package:shadcn_ui/shadcn_ui.dart';
import '../atoms/view_selector.dart';
import '../theme/visp_theme.dart';

class TopNavigationBar extends StatelessWidget {
  final VispView currentView;
  final ValueChanged<VispView?> onViewChanged;

  const TopNavigationBar({
    super.key,
    required this.currentView,
    required this.onViewChanged,
  });

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final textColor = isDark ? VispColors.textDark : VispColors.textLight;

    return Container(
      height: 36,
      padding: const EdgeInsets.symmetric(horizontal: 8),
      decoration: BoxDecoration(
        color: isDark ? VispColors.bgDark : VispColors.bgLight,
        border: Border(
          bottom: BorderSide(color: VispColors.borderSelection, width: 0.5),
        ),
      ),
      child: Row(
        children: [
          SizedBox(
            height: 28,
            child: ViewSelector(
              currentView: currentView,
              onChanged: onViewChanged,
            ),
          ),
          const SizedBox(width: 8),
          _MenuButton(label: 'File', textColor: textColor),
          _MenuButton(label: 'Edit', textColor: textColor),
          _MenuButton(label: 'View', textColor: textColor),
          _MenuButton(label: 'Help', textColor: textColor),
          VerticalDivider(
            indent: 8,
            endIndent: 8,
            width: 24,
            color: VispColors.borderSelection,
          ),
          Expanded(
            child: _buildDynamicToolbar(context),
          ),
        ],
      ),
    );
  }

  Widget _buildDynamicToolbar(BuildContext context) {
    switch (currentView) {
      case VispView.video:
        return _VideoToolbar();
      case VispView.image:
        return _ImageToolbar();
      case VispView.sound:
        return _SoundToolbar();
      case VispView.post:
        return _PostToolbar();
    }
  }
}

class _MenuButton extends StatelessWidget {
  final String label;
  final Color textColor;

  const _MenuButton({required this.label, required this.textColor});

  @override
  Widget build(BuildContext context) {
    return ShadButton.ghost(
      onPressed: () {},
      size: ShadButtonSize.sm,
      padding: const EdgeInsets.symmetric(horizontal: 10),
      foregroundColor: textColor,
      child: Text(
        label,
        style: const TextStyle(fontSize: 12, fontWeight: FontWeight.w400),
      ),
    );
  }
}

class _VideoToolbar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return const Row(
      children: [
        Icon(LucideIcons.scissors, size: 16, color: VispColors.iconUnselected),
        SizedBox(width: 12),
        Icon(LucideIcons.film, size: 16, color: VispColors.iconUnselected),
      ],
    );
  }
}

class _ImageToolbar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return const Row(
      children: [
        Icon(LucideIcons.crop, size: 16, color: VispColors.iconUnselected),
        SizedBox(width: 12),
        Icon(LucideIcons.palette, size: 16, color: VispColors.iconUnselected),
      ],
    );
  }
}

class _SoundToolbar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return const Row(
      children: [
        Icon(LucideIcons.sliders, size: 16, color: VispColors.iconUnselected),
        SizedBox(width: 12),
        Icon(LucideIcons.volume2, size: 16, color: VispColors.iconUnselected),
      ],
    );
  }
}

class _PostToolbar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return const Row(
      children: [
        Icon(LucideIcons.send, size: 16, color: VispColors.iconUnselected),
        SizedBox(width: 12),
        Icon(LucideIcons.barChart, size: 16, color: VispColors.iconUnselected),
      ],
    );
  }
}
