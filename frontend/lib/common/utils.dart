import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart' show kIsWeb;

bool isPortrait(BuildContext context) {
  return MediaQuery.of(context).orientation == Orientation.portrait;
}

bool isCategoricalDataType(String name) {
  // https://www.postgresql.org/docs/9.5/datatype-character.html
  return name == 'text';
}

bool isNumericDataType(String name) {
  // https://www.postgresql.org/docs/9.5/datatype-numeric.html
  return [
    'smallint',
    'integer',
    'bigint',
    'decimal',
    'numeric',
    'real',
    'double precision',
    'smallserial',
    'serial',
    'bigserial',
  ].contains(name);
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

extension StringExtension on String {
  String capitalize() {
    return "${this[0].toUpperCase()}${this.substring(1)}";
  }
}
