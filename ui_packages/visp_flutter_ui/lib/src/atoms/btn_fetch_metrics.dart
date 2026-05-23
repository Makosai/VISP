import 'package:flutter/material.dart';
import '../theme/visp_theme.dart';

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
    final isDark = Theme.of(context).brightness == Brightness.dark;

    return ElevatedButton(
      onPressed: isLoading ? null : onPressed,
      style: ElevatedButton.styleFrom(
        backgroundColor: VispColors.primary,
        foregroundColor: Colors.white,
        padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(4)),
      ).copyWith(
        backgroundColor: WidgetStateProperty.resolveWith((states) {
          if (states.contains(WidgetState.hovered)) {
            return isDark ? VispColors.primaryHoverDark : VispColors.primaryHoverLight;
          }
          return VispColors.primary;
        }),
      ),
      child: isLoading
          ? const SizedBox(
              width: 20,
              height: 20,
              child: CircularProgressIndicator(
                strokeWidth: 2,
                color: Colors.white,
              ),
            )
          : const Text(
              'Fetch Social Metrics',
              style: TextStyle(fontWeight: FontWeight.w600),
            ),
    );
  }
}
