import 'package:flutter/material.dart';
import '../organisms/social_metrics_view.dart';

class EditorWorkspace extends StatelessWidget {
  const EditorWorkspace({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('VISP Editor'),
      ),
      body: const Center(
        child: Padding(
          padding: EdgeInsets.all(16.0),
          child: SocialMetricsView(),
        ),
      ),
    );
  }
}
