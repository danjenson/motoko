import 'package:flutter/material.dart';

class Nav extends ChangeNotifier {
  final List<String> _names = [];
  final List<Widget> _bodies = [];
  final List<void Function(BuildContext)> _adds = [];

  List<String> get names {
    if (_names.isEmpty) {
      throw new NavException();
    }
    return _names;
  }

  Widget get body {
    if (_bodies.isEmpty) {
      throw new NavException();
    }
    return _bodies.last;
  }

  Function get add {
    if (_adds.isEmpty) {
      throw new NavException();
    }
    return _adds.last;
  }

  void push(String name, Widget body, [void Function(BuildContext) add]) {
    _names.add(name);
    _bodies.add(body);
    _adds.add(add);
    notifyListeners();
  }

  void to(int index) {
    var start = index + 1;
    var end = _names.length;
    _names.removeRange(start, end);
    _bodies.removeRange(start, end);
    _adds.removeRange(start, end);
    notifyListeners();
  }

  void back() {
    // don't go back past the last item
    if (_names.length > 1) {
      _names.removeLast();
      _bodies.removeLast();
      _adds.removeLast();
      notifyListeners();
    }
  }
}

class NavException implements Exception {
  String message;
  NavException({this.message = 'No navigation items!'});
  @override
  String toString() {
    return message;
  }
}
