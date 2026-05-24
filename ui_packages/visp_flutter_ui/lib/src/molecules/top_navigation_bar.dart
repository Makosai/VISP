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

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        // Top Tier: VISP Title and System Menu (Thin, 28px)
        Container(
          height: 28,
          padding: const EdgeInsets.symmetric(horizontal: 12),
          alignment: Alignment.center,
          decoration: BoxDecoration(
            color: isDark ? VispColors.bgDark : VispColors.bgLight,
            border: Border(
              bottom: BorderSide(color: VispColors.borderSelection, width: 0.5),
            ),
          ),
          child: Row(
            children: [
              Text(
                'VISP',
                style: TextStyle(
                  fontSize: 11,
                  fontWeight: FontWeight.w400,
                  color: isDark ? VispColors.textDark : VispColors.textLight,
                  letterSpacing: 1.2,
                ),
              ),
              const SizedBox(width: 16),
              const Expanded(child: _SystemMenubar()),
            ],
          ),
        ),
        // Bottom Tier: View Selector and Toolbars (Thicker, 52px)
        Container(
          height: 52,
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
                height: 40,
                child: ViewSelector(
                  currentView: currentView,
                  onChanged: onViewChanged,
                ),
              ),
              Expanded(child: Center(child: _buildDynamicToolbar(context))),
              // Right-side actions
              ShadButton.outline(
                onPressed: () {},
                size: ShadButtonSize.sm,
                leading: const Icon(LucideIcons.share2, size: 16),
                child: const Text('Share'),
              ),
              const SizedBox(width: 8),
              ShadButton(
                onPressed: () {},
                size: ShadButtonSize.sm,
                leading: const Icon(LucideIcons.arrowBigDownDash, size: 16),
                child: const Text('Export'),
              ),
            ],
          ),
        ),
      ],
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

class _SystemMenubar extends StatelessWidget {
  const _SystemMenubar();

  @override
  Widget build(BuildContext context) {
    return ShadMenubar(
      items: [
        ShadMenubarItem(
          items: [
            ShadContextMenuItem(
              child: const Text('New Project'),
              onPressed: () {},
            ),
            ShadContextMenuItem(child: const Text('Open...'), onPressed: () {}),
            const Divider(),
            ShadContextMenuItem(child: const Text('Save'), onPressed: () {}),
            ShadContextMenuItem(child: const Text('Exit'), onPressed: () {}),
          ],
          child: const Text('File', style: TextStyle(fontSize: 12)),
        ),
        ShadMenubarItem(
          items: [
            ShadContextMenuItem(child: const Text('Undo'), onPressed: () {}),
            ShadContextMenuItem(child: const Text('Redo'), onPressed: () {}),
          ],
          child: const Text('Edit', style: TextStyle(fontSize: 12)),
        ),
        ShadMenubarItem(
          items: [
            ShadContextMenuItem(
              child: const Text('Full Screen'),
              onPressed: () {},
            ),
          ],
          child: const Text('View', style: TextStyle(fontSize: 12)),
        ),
        ShadMenubarItem(
          items: [
            ShadContextMenuItem(
              child: const Text('About VISP'),
              onPressed: () {},
            ),
          ],
          child: const Text('Help', style: TextStyle(fontSize: 12)),
        ),
      ],
    );
  }
}

class _ToolbarAction extends StatefulWidget {
  final IconData icon;
  final String label;
  final VoidCallback onPressed;
  final bool isOutline;

  const _ToolbarAction({
    required this.icon,
    required this.label,
    required this.onPressed,
    this.isOutline = false,
  });

  @override
  State<_ToolbarAction> createState() => _ToolbarActionState();
}

class _ToolbarActionState extends State<_ToolbarAction> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final color = _isHovered ? VispColors.textDark : VispColors.iconUnselected;

    Widget content = Column(
      mainAxisSize: MainAxisSize.min,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Icon(widget.icon, size: 18, color: color),
        const SizedBox(height: 2),
        Text(widget.label, style: TextStyle(fontSize: 9, color: color)),
      ],
    );

    if (widget.isOutline) {
      return MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: ShadButton.outline(
          onPressed: widget.onPressed,
          size: ShadButtonSize.sm,
          padding: const EdgeInsets.symmetric(horizontal: 12),
          decoration: ShadDecoration(
            border: ShadBorder.all(color: color, width: 1),
          ),
          child: content,
        ),
      );
    }

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        behavior: HitTestBehavior.opaque,
        onTap: widget.onPressed,
        child: SizedBox(
          width: 50, // Fixed width for action buttons
          child: content,
        ),
      ),
    );
  }
}

class _VideoToolbar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        _ToolbarAction(
          icon: LucideIcons.scissors,
          label: 'Cut',
          onPressed: () {},
        ),
        _ToolbarAction(icon: LucideIcons.copy, label: 'Copy', onPressed: () {}),
        _ToolbarAction(
          icon: LucideIcons.clipboardPaste,
          label: 'Paste',
          onPressed: () {},
        ),
        _ToolbarAction(
          icon: LucideIcons.split,
          label: 'Split',
          onPressed: () {},
        ),
        _ToolbarAction(
          icon: LucideIcons.eraser,
          label: 'Delete',
          onPressed: () {},
        ),
      ],
    );
  }
}

class _ImageToolbar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        _ToolbarAction(icon: LucideIcons.crop, label: 'Crop', onPressed: () {}),
        _ToolbarAction(
          icon: LucideIcons.palette,
          label: 'Color',
          onPressed: () {},
        ),
        _ToolbarAction(
          icon: LucideIcons.layers,
          label: 'Layers',
          onPressed: () {},
        ),
        _ToolbarAction(
          icon: LucideIcons.wand2,
          label: 'Effects',
          onPressed: () {},
        ),
        _ToolbarAction(
          icon: LucideIcons.rotateCcw,
          label: 'Rotate',
          onPressed: () {},
        ),
      ],
    );
  }
}

class _SoundToolbar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        _ToolbarAction(
          icon: LucideIcons.sliders,
          label: 'Mix',
          onPressed: () {},
        ),
        _ToolbarAction(
          icon: LucideIcons.volume2,
          label: 'Volume',
          onPressed: () {},
        ),
        _ToolbarAction(
          icon: LucideIcons.mic,
          label: 'Record',
          onPressed: () {},
        ),
        _ToolbarAction(
          icon: LucideIcons.activity,
          label: 'Wave',
          onPressed: () {},
        ),
        _ToolbarAction(
          icon: LucideIcons.music,
          label: 'Library',
          onPressed: () {},
        ),
      ],
    );
  }
}

class _PostToolbar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        _ToolbarAction(icon: LucideIcons.send, label: 'Send', onPressed: () {}),
        _ToolbarAction(
          icon: LucideIcons.calendar,
          label: 'Schedule',
          onPressed: () {},
        ),
        _ToolbarAction(
          icon: LucideIcons.barChart,
          label: 'Stats',
          onPressed: () {},
        ),
        _ToolbarAction(
          icon: LucideIcons.messageSquare,
          label: 'Inbox',
          onPressed: () {},
        ),
        _ToolbarAction(icon: LucideIcons.hash, label: 'Tags', onPressed: () {}),
      ],
    );
  }
}
