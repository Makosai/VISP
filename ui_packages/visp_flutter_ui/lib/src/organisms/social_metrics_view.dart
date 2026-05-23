import 'package:flutter/material.dart';
import 'package:visp_bridge/visp_bridge.dart';
import '../atoms/btn_fetch_metrics.dart';

class SocialMetricsView extends StatefulWidget {
  const SocialMetricsView({super.key});

  @override
  State<SocialMetricsView> createState() => _SocialMetricsViewState();
}

class _SocialMetricsViewState extends State<SocialMetricsView> {
  String _metrics = 'No data fetched';
  bool _isLoading = false;

  Future<void> _handleFetch() async {
    setState(() {
      _isLoading = true;
    });

    try {
      final result = await fetchMockSocialMetrics();
      if (mounted) {
        setState(() {
          _metrics = result;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _metrics = 'Error: $e';
        });
      }
    } finally {
      if (mounted) {
        setState(() {
          _isLoading = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Text(
          _metrics,
          style: Theme.of(context).textTheme.headlineSmall,
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 24),
        BtnFetchMetrics(
          onPressed: _handleFetch,
          isLoading: _isLoading,
        ),
      ],
    );
  }
}
