import 'package:flutter/material.dart';

class Profile extends ChangeNotifier {
  final String _username = 'dark_motoko';

  String get username => _username;
}
