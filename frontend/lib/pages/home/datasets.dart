import '../../common/error_dialog.dart';
import '../../common/form_dialog.dart';
import '../../common/globals.dart' as globals;
import '../../common/types.dart';
import 'data_preview.dart';
import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:sliding_up_panel/sliding_up_panel.dart';
import 'preview_panel.dart';
import 'query_results_list.dart';

class Datasets extends StatefulWidget {
  Datasets({@required this.projectId});
  final String projectId;
  final datasets = '''
    query Datasets(\$projectId: ID!) {
      datasets(projectId: \$projectId) {
        __typename
        id
        createdAt
        updatedAt
        name
        status
      }
    }
  ''';
  @override
  _DatasetsState createState() => _DatasetsState();
}

class _DatasetsState extends State<Datasets> {
  String _selectedId;
  PanelController _controller = PanelController();
  @override
  Widget build(BuildContext context) {
    return PreviewPanel(
        controller: _controller,
        main: QueryResultsList(
          query: widget.datasets,
          variables: {"projectId": widget.projectId},
          getter: (v) => v["datasets"],
          title: (v) => v["name"],
          onTap: (v) {
            setState(() {
              _selectedId = v["id"].toString();
              _controller.animatePanelToPosition(1.0);
            });
          },
          selectedId: _selectedId,
        ),
        preview: _selectedId == null
            ? Expanded(
                child: Center(
                    child: Text("Tap a dataset for sample rows.",
                        style: TextStyle(fontSize: 20.0))))
            : DataPreview(id: _selectedId));
  }
}

class NewDatasetForm extends StatefulWidget {
  NewDatasetForm({@required this.client, @required this.projectId});
  final GraphQLClient client;
  final String projectId;
  final createDataset = '''
    mutation CreateDataset(\$projectId: ID!, \$name: String!, \$uri: String!) {
      createDataset(projectId: \$projectId, name: \$name, uri: \$uri) {
        __typename
        id
        createdAt
        updatedAt
        name
        status
      }
    }
  ''';
  @override
  _NewDatasetFormState createState() => _NewDatasetFormState();
}

class _NewDatasetFormState extends State<NewDatasetForm> {
  String _name;
  String _uri;
  final _formKey = GlobalKey<FormState>();
  @override
  Widget build(BuildContext context) {
    return Form(
        key: _formKey,
        child: Column(mainAxisSize: MainAxisSize.min, children: <Widget>[
          Padding(
              padding: EdgeInsets.fromLTRB(20, 20, 20, 0),
              child: TextFormField(
                autofocus: true,
                validator: (value) {
                  if (value.isEmpty) {
                    return "Invalid Name";
                  }
                  return null;
                },
                onSaved: (String value) {
                  _name = value;
                },
                decoration: InputDecoration(isDense: true, hintText: "Name"),
              )),
          Padding(
              padding: EdgeInsets.fromLTRB(20, 20, 20, 0),
              child: TextFormField(
                validator: (value) {
                  if (value.isEmpty) {
                    return "Invalid URI";
                  }
                  // TODO(danj): validate URI
                  return null;
                },
                onSaved: (String value) {
                  _uri = value;
                },
                decoration: InputDecoration(isDense: true, hintText: "URI"),
              )),
          Padding(
              padding: EdgeInsets.all(5),
              child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                  children: <Widget>[
                    FlatButton(
                        child: Text(
                          "CANCEL",
                          style: TextStyle(
                              color: Theme.of(context).colorScheme.secondary),
                        ),
                        onPressed: () {
                          Navigator.of(context).pop();
                        }),
                    FlatButton(
                        child: Text(
                          "CREATE",
                          style: TextStyle(
                              color: Theme.of(context).colorScheme.secondary),
                        ),
                        onPressed: () async {
                          if (_formKey.currentState.validate()) {
                            _formKey.currentState.save();
                            Navigator.of(context).pop();
                            final createOpts = QueryOptions(
                                fetchPolicy: FetchPolicy.networkOnly,
                                documentNode: gql(widget.createDataset),
                                variables: {
                                  "projectId": widget.projectId,
                                  "name": _name,
                                  "uri": _uri
                                });
                            var res = await widget.client.query(createOpts);
                            if (res.hasException) {
                              errorDialog(
                                  context: context,
                                  message: res.exception.toString());
                            }
                            globals.refetch();
                          }
                        })
                  ]))
        ]));
  }
}

Adder makeAdder(String projectId) {
  void add(BuildContext context) {
    final client = GraphQLProvider.of(context).value;
    final form = NewDatasetForm(client: client, projectId: projectId);
    formDialog(context: context, title: 'New Dataset', form: form);
  }

  return add;
}
