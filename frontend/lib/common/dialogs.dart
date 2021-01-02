import '../pages/home/components/globals.dart' as globals;
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

class CustomDialog extends StatelessWidget {
  CustomDialog({@required this.title, this.content, this.actions});
  final String title;
  final Widget content;
  final List<Widget> actions;
  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(title, textAlign: TextAlign.center),
      content: SingleChildScrollView(child: content),
      contentPadding: EdgeInsets.fromLTRB(25, 0, 25, 5),
      actions: actions,
    );
  }
}

void showErrorDialog(BuildContext context, String message) {
  showSimpleDialog(context, 'Error', message);
}

void showInfoDialog(BuildContext context, String message) {
  showSimpleDialog(context, 'Info', message);
}

void showSimpleDialog(BuildContext context, String title, String message) {
  showDialog(
      context: context,
      child: AlertDialog(
          title: Text(title, textAlign: TextAlign.center),
          content: Text(message),
          contentPadding: EdgeInsets.fromLTRB(25, 25, 25, 0),
          actions: <Widget>[
            FlatButton(
                child: Text('OK'),
                onPressed: () {
                  Navigator.of(context).pop();
                })
          ]));
}

VoidCallback showProgessDialog([message = 'Kicking the tires...']) {
  showDialog(
      context: globals.homeKey.currentContext,
      child: AlertDialog(
          backgroundColor: Colors.black,
          content: ListTile(
              leading: CircularProgressIndicator(),
              title: Text(message,
                  textAlign: TextAlign.center, style: TextStyle(fontSize: 20))),
          contentPadding: EdgeInsets.all(10)));
  return () => Navigator.of(globals.homeKey.currentContext).pop();
}

class CreateDialog extends StatefulWidget {
  CreateDialog({
    this.name,
    this.makeForm,
    this.mutation,
    this.fieldsToVariables,
    this.onCreate,
  });
  final String name;
  final Widget Function(
          void Function(Map<String, dynamic>, bool Function()) setFormState)
      makeForm;
  final String mutation;
  final Map<String, dynamic> Function(dynamic fields) fieldsToVariables;
  final void Function(dynamic v) onCreate;
  @override
  _CreateDialogState createState() => _CreateDialogState();
}

class _CreateDialogState extends State<CreateDialog> {
  Map<String, dynamic> _fields;
  bool Function() _validate = () => false;
  @override
  Widget build(BuildContext context) {
    return CustomDialog(
        title: 'New ${widget.name}',
        content: Column(children: [
          widget.makeForm(
            (fields, validate) => setState(() {
              debugPrint(jsonEncode(fields));
              _fields = fields;
              _validate = validate;
            }),
          ),
          SizedBox(height: 5),
          Row(mainAxisAlignment: MainAxisAlignment.spaceEvenly, children: [
            FlatButton(
                child: Text('CANCEL'),
                onPressed: () => Navigator.of(context).pop()),
            FlatButton(
              child: Text('CREATE'),
              onPressed: () async {
                if (_validate()) {
                  Navigator.of(context).pop();
                  var closeProgress = showProgessDialog();
                  final client = GraphQLProvider.of(context).value;
                  _fields.removeWhere((k, v) => v == null);
                  final mutOpts = MutationOptions(
                      fetchPolicy: FetchPolicy.networkOnly,
                      documentNode: gql(widget.mutation),
                      variables: widget.fieldsToVariables(_fields),
                      onCompleted: (v) {
                        if (v != null) {
                          widget.onCreate((v as Map).values.first);
                        }
                      });
                  final res = await client.mutate(mutOpts);
                  closeProgress();
                  if (res.hasException) {
                    showErrorDialog(context, res.exception.toString());
                  }
                }
              },
            )
          ]),
        ]));
  }
}

void showCreateDialog({
  BuildContext context,
  String name,
  final Widget Function(
          void Function(Map<String, dynamic>, bool Function()) setFormState)
      makeForm,
  String mutation,
  Map<String, dynamic> Function(dynamic fields) fieldsToVariables,
  void Function(dynamic v) onCreate,
}) {
  showDialog(
      context: context,
      child: CreateDialog(
          name: name,
          makeForm: makeForm,
          mutation: mutation,
          fieldsToVariables: fieldsToVariables,
          onCreate: onCreate));
}
