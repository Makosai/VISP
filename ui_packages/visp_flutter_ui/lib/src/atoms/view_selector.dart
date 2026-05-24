import 'package:flutter/material.dart';
import 'package:shadcn_ui/shadcn_ui.dart';

import '../theme/visp_theme.dart';

enum VispView {
  video(LucideIcons.clapperboard, 'Video'),
  image(LucideIcons.image, 'Image'),
  sound(LucideIcons.music, 'Sound'),
  post(LucideIcons.megaphone, 'Post');

  final IconData icon;
  final String label;
  const VispView(this.icon, this.label);
}

class ViewSelector extends StatelessWidget {
  final VispView currentView;
  final ValueChanged<VispView?> onChanged;

  const ViewSelector({
    super.key,
    required this.currentView,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final fgColor = isDark ? VispColors.textDark : VispColors.textLight;
    final mutedColor = VispColors.textMuted;

    return ShadSelect<VispView>(
      initialValue: currentView,
      onChanged: onChanged,
      minWidth: 130,
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 0),
      selectedOptionBuilder: (context, view) {
        return Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(view.icon, size: 16, color: fgColor),
            const SizedBox(width: 10),
            Text(
              view.label,
              style: TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.w500,
                color: fgColor,
              ),
            ),
          ],
        );
      },
      options: VispView.values
          .map(
            (view) => ShadOption(
              value: view,
              padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
              child: Row(
                children: [
                  Icon(
                    view.icon,
                    size: 16,
                    color: view == currentView ? fgColor : mutedColor,
                  ),
                  const SizedBox(width: 10),
                  Text(
                    view.label,
                    style: TextStyle(
                      fontSize: 13,
                      color: view == currentView ? fgColor : mutedColor,
                    ),
                  ),
                ],
              ),
            ),
          )
          .toList(),
    );
  }
}
