import 'package:flutter/material.dart';

Future<void> errorDialog({BuildContext context, String message}) async {
  return showDialog<void>(
      context: context,
      barrierDismissible: false,
      builder: (BuildContext context) => ErrorDialog(message: message));
}

class ErrorDialog extends StatelessWidget {
  final String message;
  ErrorDialog({@required this.message});
  @override
  Widget build(BuildContext context) {
    return AlertDialog(
        title: Text('Error',
            textAlign: TextAlign.center,
            style: TextStyle(color: Theme.of(context).colorScheme.secondary)),
        content: SingleChildScrollView(child: Text(message)),
        actions: <Widget>[
          FlatButton(
              child: Text('OK',
                  style: TextStyle(
                      color: Theme.of(context).colorScheme.secondary)),
              onPressed: () {
                Navigator.of(context).pop();
              })
        ]);
  }
}
