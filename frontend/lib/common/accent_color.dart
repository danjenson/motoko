import 'package:flutter/material.dart';
import 'storage.dart';

class AccentColor extends ChangeNotifier {
  AccentColor(this.storage);
  final Storage storage;

  Color _first = Colors.pinkAccent;
  Color _second = Colors.tealAccent;
  Color _value = Colors.pinkAccent;

  Color get value => _value;
  Color get first => _first;
  Color get second => _second;
  bool get isFirst => _value == _first;

  void init() async {
    if (await storage.hasKey('isFirst') &&
        await storage.getBool('isFirst') != isFirst) {
      flip();
    }
    notifyListeners();
  }

  void flip() async {
    _value =
        _value == Colors.pinkAccent ? Colors.tealAccent : Colors.pinkAccent;
    await storage.putBool(key: 'isFirst', value: isFirst);
    notifyListeners();
  }
}
