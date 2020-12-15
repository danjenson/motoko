import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:sliding_up_panel/sliding_up_panel.dart';
import '../../common/error_dialog.dart';
import '../../common/globals.dart' as globals;
import '../../common/types.dart';
import 'preview_panel.dart';
import 'data_preview.dart';
import 'query_results_list.dart';

class Datasets extends StatefulWidget {
  Datasets({@required this.projectID});
  final String projectID;
  final datasetsQuery = '''
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
  String _selectedDatasetID;
  PanelController _controller = PanelController();
  @override
  Widget build(BuildContext context) {
    return PreviewPanel(
        controller: _controller,
        main: QueryResultsList(
          query: widget.datasetsQuery,
          variables: {"projectId": widget.projectID},
          getter: (v) => v["datasets"],
          onTap: (ds) {
            this.setState(() {
              _controller.animatePanelToPosition(1.0);
              _selectedDatasetID = ds["id"].toString();
            });
          },
        ),
        preview: _selectedDatasetID == null
            ? Center(child: Text("Select a dataset!"))
            : DataPreview(id: _selectedDatasetID));
  }
}

class NewDatasetForm extends StatefulWidget {
  NewDatasetForm({@required this.client, @required this.projectID});
  final GraphQLClient client;
  final String projectID;
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
                                  "projectId": widget.projectID,
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

Adder makeAdder(String projectID) {
  void add(BuildContext context) {
    final client = GraphQLProvider.of(context).value;
    showDialog(
        context: context,
        builder: (BuildContext context) {
          return AlertDialog(
            title: Text("New Dataset",
                textAlign: TextAlign.center,
                style:
                    TextStyle(color: Theme.of(context).colorScheme.secondary)),
            contentPadding: EdgeInsets.zero,
            content: NewDatasetForm(client: client, projectID: projectID),
          );
        });
  }

  return add;
}
