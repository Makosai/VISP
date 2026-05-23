import 'package:flutter/material.dart';

class BtnFetchMetrics extends StatelessWidget {
  final VoidCallback onPressed;
  final bool isLoading;

  const BtnFetchMetrics({
    super.key,
    required this.onPressed,
    this.isLoading = false,
  });

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: isLoading ? null : onPressed,
      child: isLoading
          ? const SizedBox(
              width: 20,
              height: 20,
              child: CircularProgressIndicator(strokeWidth: 2),
            )
          : const Text('Fetch Social Metrics'),
    );
  }
}
