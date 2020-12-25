import 'package:flutter/material.dart';

Future<void> formDialog(
    {BuildContext context, String title, Widget form}) async {
  return showDialog<void>(
      context: context,
      barrierDismissible: true,
      builder: (BuildContext context) => FormDialog(title, form));
}

class FormDialog extends StatelessWidget {
  FormDialog(this.title, this.form);
  final String title;
  final Widget form;
  @override
  Widget build(BuildContext context) {
    final color = Theme.of(context).colorScheme.secondary;
    return AlertDialog(
      elevation: 0.0,
      shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(10),
          side: BorderSide(width: 1.0, color: color)),
      title: Text(title,
          textAlign: TextAlign.center,
          style: TextStyle(color: Theme.of(context).colorScheme.secondary)),
      contentPadding: EdgeInsets.all(5),
      content: form,
    );
  }
}
