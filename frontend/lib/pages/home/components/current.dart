import 'package:flutter/material.dart';
import 'globals.dart' as globals;

class Current extends ChangeNotifier {
  Current(widget, name, {hasCreateButton = true})
      : _stack = [widget],
        names = [name],
        _buttons = [hasCreateButton];
  final List<Widget> _stack;
  final List<String> names;
  final List<bool> _buttons;

  bool get canGoBack {
    return _stack.length > 1;
  }

  Widget get content {
    if (_stack.isEmpty) {
      throw new CurrentException();
    }
    return _stack.last;
  }

  bool get hasCreateButton {
    if (_buttons.isEmpty) {
      throw new CurrentException();
    }
    return _buttons.last;
  }

  String get name {
    if (names.isEmpty) {
      throw new CurrentException();
    }
    return names.last;
  }

  void push(Widget content, String name, {hasCreateButton = true}) {
    _clearCreate();
    _stack.add(content);
    names.add(name);
    _buttons.add(hasCreateButton);
    notifyListeners();
  }

  void _clearCreate() {
    globals.create = () => {};
  }

  void to(int index) {
    final start = index + 1;
    final end = _stack.length;
    _clearCreate();
    _stack.removeRange(start, end);
    names.removeRange(start, end);
    _buttons.removeRange(start, end);
    notifyListeners();
  }

  void back() {
    // don't go back past the last item
    if (_stack.length > 1) {
      _clearCreate();
      _stack.removeLast();
      names.removeLast();
      _buttons.removeLast();
      notifyListeners();
    }
  }
}

class CurrentException implements Exception {
  String message;
  CurrentException({this.message = 'No content items!'});
  @override
  String toString() {
    return message;
  }
}
