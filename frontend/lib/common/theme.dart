import 'package:flutter/material.dart';

ThemeData theme(Color accentColor) {
  return ThemeData(
    appBarTheme: AppBarTheme(iconTheme: IconThemeData(color: accentColor)),
    iconTheme: IconThemeData(color: accentColor),
    accentColor: accentColor,
    inputDecorationTheme: InputDecorationTheme(
      focusedBorder:
          UnderlineInputBorder(borderSide: BorderSide(color: accentColor)),
    ),
    colorScheme: ColorScheme.dark(secondary: accentColor),
    visualDensity: VisualDensity.adaptivePlatformDensity,
  );
}
