import 'package:flutter/material.dart';
import '../molecules/top_navigation_bar.dart';
import '../atoms/view_selector.dart';
import '../organisms/social_metrics_view.dart';

class EditorWorkspace extends StatefulWidget {
  const EditorWorkspace({super.key});

  @override
  State<EditorWorkspace> createState() => _EditorWorkspaceState();
}

class _EditorWorkspaceState extends State<EditorWorkspace> {
  VispView _currentView = VispView.video;

  void _handleViewChanged(VispView? newView) {
    if (newView != null) {
      setState(() {
        _currentView = newView;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: [
          TopNavigationBar(
            currentView: _currentView,
            onViewChanged: _handleViewChanged,
          ),
          Expanded(
            child: _buildBody(),
          ),
        ],
      ),
    );
  }

  Widget _buildBody() {
    switch (_currentView) {
      case VispView.post:
        return const Center(child: SocialMetricsView());
      default:
        return Center(
          child: Text(
            '${_currentView.label} Workspace Placeholder',
            style: Theme.of(context).textTheme.headlineMedium,
          ),
        );
    }
  }
}
