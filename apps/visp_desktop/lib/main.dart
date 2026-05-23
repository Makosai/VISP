import 'package:flutter/material.dart';
import 'package:visp_bridge/visp_bridge.dart';
import 'package:visp_flutter_ui/visp_flutter_ui.dart';

void main() async {
  // Ensure Flutter is initialized
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize Rust Core via FFI
  await RustLib.init();

  runApp(const VispApp());
}

class VispApp extends StatelessWidget {
  const VispApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'VISP Desktop',
      debugShowCheckedModeBanner: false,
      theme: VispTheme.light(),
      darkTheme: VispTheme.dark(),
      themeMode: ThemeMode.dark, // Default to dark as per project vibe
      home: const EditorWorkspace(),
    );
  }
}
