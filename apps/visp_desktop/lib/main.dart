import 'package:flutter/material.dart';
import 'package:shadcn_ui/shadcn_ui.dart';
import 'package:visp_bridge/visp_bridge.dart';
import 'package:visp_flutter_ui/visp_flutter_ui.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const VispApp());
}

class VispApp extends StatelessWidget {
  const VispApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ShadApp(
      title: 'VISP Desktop',
      debugShowCheckedModeBanner: false,
      theme: VispTheme.light(),
      darkTheme: VispTheme.dark(),
      themeMode: ThemeMode.dark,
      home: const EditorWorkspace(),
    );
  }
}
