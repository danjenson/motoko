import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart' show kIsWeb;

bool isPortrait(BuildContext context) {
  return MediaQuery.of(context).orientation == Orientation.portrait;
}

bool isWeb() {
  return kIsWeb;
}

bool isDark(BuildContext context) {
  final Brightness brightnessValue = MediaQuery.of(context).platformBrightness;
  return brightnessValue == Brightness.dark;
}

int timestamp() {
  return DateTime.now().millisecondsSinceEpoch ~/ 1000;
}
