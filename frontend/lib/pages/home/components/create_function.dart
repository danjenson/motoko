import 'package:flutter/material.dart';

class CreateFunction extends ChangeNotifier {
  VoidCallback _func;

  bool exists() {
    return _func != null;
  }

  set func(VoidCallback func) {
    _func = func;
    notifyListeners();
  }

  void call() {
    _func();
  }
}
