import 'package:flutter/material.dart';

ThemeData theme(Color accentColor) {
  return ThemeData(
    appBarTheme: AppBarTheme(iconTheme: IconThemeData(color: accentColor)),
    iconTheme: IconThemeData(color: accentColor),
    primaryColor: Colors.black,
    shadowColor: accentColor,
    canvasColor: Colors.black,
    cardColor: Colors.black,
    accentColor: accentColor.withOpacity(0.8),
    cardTheme: CardTheme(
        elevation: 0.0,
        shape: RoundedRectangleBorder(
            side: BorderSide(color: accentColor.withOpacity(0.65), width: 1),
            borderRadius: BorderRadius.all(Radius.circular(10)))),
    buttonTheme: ButtonThemeData(alignedDropdown: true),
    dialogBackgroundColor: Colors.black,
    inputDecorationTheme: InputDecorationTheme(
      focusedBorder:
          UnderlineInputBorder(borderSide: BorderSide(color: accentColor)),
    ),
    colorScheme: ColorScheme.dark(secondary: accentColor),
    visualDensity: VisualDensity.adaptivePlatformDensity,
  );
}
