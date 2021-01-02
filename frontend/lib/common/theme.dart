import 'package:flutter/material.dart';

ThemeData makeTheme(Color accentColor) {
  return ThemeData(
    primaryColor: Colors.black,
    shadowColor: accentColor,
    canvasColor: Colors.black,
    cardColor: Colors.black,
    errorColor: accentColor,
    accentColor: accentColor,
    dialogBackgroundColor: Colors.black,
    colorScheme:
        ColorScheme.dark(primary: Colors.black, secondary: accentColor),
    appBarTheme:
        AppBarTheme(elevation: 0, iconTheme: IconThemeData(color: accentColor)),
    buttonTheme: ButtonThemeData(alignedDropdown: true),
    bottomAppBarTheme: BottomAppBarTheme(color: Colors.black, elevation: 0),
    cardTheme: CardTheme(
        elevation: 0.0,
        shape: RoundedRectangleBorder(
            side: BorderSide(color: accentColor.withOpacity(0.8), width: 1),
            borderRadius: BorderRadius.all(Radius.circular(10)))),
    dialogTheme: DialogTheme(
        elevation: 0,
        backgroundColor: Colors.black,
        titleTextStyle: TextStyle(
            color: accentColor, fontSize: 18, fontWeight: FontWeight.bold),
        shape: RoundedRectangleBorder(
            side: BorderSide(width: 1, color: accentColor),
            borderRadius: BorderRadius.circular(10))),
    floatingActionButtonTheme: FloatingActionButtonThemeData(elevation: 0),
    iconTheme: IconThemeData(color: accentColor, size: 30),
    dividerTheme: DividerThemeData(color: accentColor.withOpacity(0.65)),
    inputDecorationTheme: InputDecorationTheme(
      focusedBorder:
          UnderlineInputBorder(borderSide: BorderSide(color: accentColor)),
    ),
    visualDensity: VisualDensity.adaptivePlatformDensity,
  );
}
