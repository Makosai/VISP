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
    return ShadSelect<VispView>(
      initialValue: currentView,
      onChanged: onChanged,
      minWidth: 140,
      decoration: ShadDecoration(
        border: ShadBorder.all(
          color: VispColors.borderSelection,
          width: 0.5,
          radius: BorderRadius.circular(4),
        ),
        color: Theme.of(context).brightness == Brightness.dark
            ? VispColors.badgeBgDark
            : VispColors.badgeBgLight,
      ),
      selectedOptionBuilder: (context, view) {
        return Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(view.icon, size: 16, color: VispColors.primary),
            const SizedBox(width: 8),
            Text(
              view.label,
              style: const TextStyle(fontSize: 13, fontWeight: FontWeight.w500),
            ),
          ],
        );
      },
      options: VispView.values
          .map((view) => ShadOption(
                value: view,
                child: Row(
                  children: [
                    Icon(view.icon, size: 16, color: VispColors.primary),
                    const SizedBox(width: 8),
                    Text(view.label),
                  ],
                ),
              ))
          .toList(),
    );
  }
}
