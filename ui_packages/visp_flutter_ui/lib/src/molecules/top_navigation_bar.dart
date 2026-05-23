import 'package:flutter/material.dart';
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
    final isDark = Theme
        .of(context)
        .brightness == Brightness.dark;
    final textColor = isDark ? VispColors.textDark : VispColors.textLight;

    return Container(
      height: 48,
      padding: const EdgeInsets.symmetric(horizontal: 12),
      decoration: BoxDecoration(
        color: isDark ? VispColors.bgDark : VispColors.bgLight,
        border: Border(
          bottom: BorderSide(color: VispColors.borderSelection, width: 0.5),
        ),
      ),
      child: Row(
        children: [
          ViewSelector(
            currentView: currentView,
            onChanged: onViewChanged,
          ),
          const SizedBox(width: 16),
          _MenuButton(label: 'File', textColor: textColor),
          _MenuButton(label: 'Edit', textColor: textColor),
          _MenuButton(label: 'View', textColor: textColor),
          _MenuButton(label: 'Help', textColor: textColor),
          const VerticalDivider(
            indent: 12,
            endIndent: 12,
            width: 32,
            color: VispColors.iconUnselected,
          ),
          Expanded(
            child: _buildDynamicToolbar(context),
          ),
        ],
      ),
    );
  }

  Widget _buildDynamicToolbar(BuildContext context) {
    // Placeholder for dynamic items based on view
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
    return TextButton(
      onPressed: () {},
      style: TextButton.styleFrom(
        foregroundColor: textColor,
        textStyle: const TextStyle(fontSize: 13, fontWeight: FontWeight.w400),
        padding: const EdgeInsets.symmetric(horizontal: 8),
      ),
      child: Text(label),
    );
  }
}

// Dummy Toolbars for now
class _VideoToolbar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return const Row(
      children: [
        Icon(Icons.cut_outlined, size: 20, color: VispColors.iconUnselected),
        SizedBox(width: 16),
        Icon(Icons.slow_motion_video, size: 20,
            color: VispColors.iconUnselected),
      ],
    );
  }
}

class _ImageToolbar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return const Row(
      children: [
        Icon(Icons.crop_outlined, size: 20, color: VispColors.iconUnselected),
        SizedBox(width: 16),
        Icon(Icons.color_lens_outlined, size: 20,
            color: VispColors.iconUnselected),
      ],
    );
  }
}

class _SoundToolbar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return const Row(
      children: [
        Icon(Icons.equalizer_outlined, size: 20,
            color: VispColors.iconUnselected),
        SizedBox(width: 16),
        Icon(Icons.volume_up_outlined, size: 20,
            color: VispColors.iconUnselected),
      ],
    );
  }
}

class _PostToolbar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return const Row(
      children: [
        Icon(Icons.schedule_send_outlined, size: 20,
            color: VispColors.iconUnselected),
        SizedBox(width: 16),
        Icon(Icons.analytics_outlined, size: 20,
            color: VispColors.iconUnselected),
      ],
    );
  }
}
