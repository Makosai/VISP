import 'package:flutter/material.dart';
import '../theme/visp_theme.dart';

enum VispView {
  video(Icons.movie_creation_outlined, 'Video'),
  image(Icons.photo_outlined, 'Image'),
  sound(Icons.music_note_outlined, 'Sound'),
  post(Icons.campaign_outlined, 'Post');

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

    return Container(
      height: 32,
      padding: const EdgeInsets.symmetric(horizontal: 8),
      decoration: BoxDecoration(
        color: isDark ? VispColors.badgeBgDark : VispColors.badgeBgLight,
        borderRadius: BorderRadius.circular(4),
        border: Border.all(color: VispColors.borderSelection, width: 0.5),
      ),
      child: DropdownButtonHideUnderline(
        child: DropdownButton<VispView>(
          value: currentView,
          dropdownColor: isDark ? VispColors.bgDark : VispColors.bgLight,
          icon: const Icon(
            Icons.keyboard_arrow_down,
            size: 16,
            color: VispColors.iconUnselected,
          ),
          style: TextStyle(
            color: isDark ? VispColors.textDark : VispColors.textLight,
            fontSize: 13,
            fontWeight: FontWeight.w500,
          ),
          onChanged: onChanged,
          items: VispView.values.map((VispView view) {
            return DropdownMenuItem<VispView>(
              value: view,
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(view.icon, size: 18, color: VispColors.primary),
                  const SizedBox(width: 8),
                  Text(view.label),
                ],
              ),
            );
          }).toList(),
        ),
      ),
    );
  }
}
