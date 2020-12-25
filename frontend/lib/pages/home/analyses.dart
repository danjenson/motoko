import '../../common/error_dialog.dart';
import '../../common/form_dialog.dart';
import '../../common/globals.dart' as globals;
import '../../common/types.dart';
import 'analysis.dart' as a;
import 'nav.dart';
import 'package:dropdown_search/dropdown_search.dart';
import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:provider/provider.dart';
import 'query_results_list.dart';

class Analyses extends StatelessWidget {
  Analyses({@required this.nav, @required this.projectId});
  final Nav nav;
  final String projectId;
  final query = '''
    query Analyses(\$projectId: ID!) {
      analyses(projectId: \$projectId) {
        __typename
        id
        createdAt
        updatedAt
        name
        dataset {
          __typename
          id
        }
        dataview {
          __typename
          id
        }
      }
    }
  ''';
  @override
  Widget build(BuildContext context) {
    final onTap = (dynamic x) => nav.push(x["name"].toString(),
        a.Analysis(nav: nav, analysisId: x["id"].toString()));
    return QueryResultsList(
      query: query,
      variables: {"projectId": this.projectId},
      getter: (v) => v['analyses'],
      title: (v) => v['name'],
      onTap: onTap,
    );
  }
}

class NewAnalysisForm extends StatefulWidget {
  NewAnalysisForm(
      {@required this.nav, @required this.client, @required this.projectId});
  final Nav nav;
  final GraphQLClient client;
  final String projectId;
  final datasets = '''
    query Datasets(\$projectId: ID!) {
      datasets(projectId: \$projectId) {
        __typename
        id
        name
      }
    }
  ''';
  final createAnalysis = '''
    mutation CreateAnalysis(\$datasetId: ID!, \$name: String!) {
      createAnalysis(datasetId: \$datasetId, name: \$name) {
        __typename
        id
        createdAt
        updatedAt
        name
        dataset {
          __typename
          id
        }
        dataview {
          __typename
          id
        }
      }
    }
  ''';
  @override
  _NewAnalysisFormState createState() => _NewAnalysisFormState();
}

class _NewAnalysisFormState extends State<NewAnalysisForm> {
  String datasetId;
  String name;
  final _formKey = GlobalKey<FormState>();
  @override
  Widget build(BuildContext context) {
    return Query(
        options: QueryOptions(
          fetchPolicy: FetchPolicy.cacheAndNetwork,
          documentNode: gql(widget.datasets),
          variables: {"projectId": widget.projectId},
        ),
        builder: (QueryResult result,
            {VoidCallback refetch, FetchMore fetchMore}) {
          globals.refetch = refetch;
          var results = [];
          var loading =
              result.loading && result.source == QueryResultSource.Loading;
          if (result.hasException) {
            return ErrorDialog(result.exception.toString());
          } else if (!loading) {
            results = result.data["datasets"] ?? [];
          }
          return Padding(
              padding: EdgeInsets.fromLTRB(20, 20, 20, 0),
              child: Form(
                  key: _formKey,
                  child:
                      Column(mainAxisSize: MainAxisSize.min, children: <Widget>[
                    TextFormField(
                      autofocus: true,
                      validator: (value) {
                        if (value.isEmpty) {
                          return "Invalid Name";
                        }
                        return null;
                      },
                      onSaved: (String value) {
                        name = value;
                      },
                      decoration:
                          InputDecoration(isDense: true, hintText: "Name"),
                    ),
                    DropdownSearch<dynamic>(
                        mode: Mode.DIALOG,
                        showSelectedItem: true,
                        maxHeight: 300,
                        dialogMaxWidth: 300,
                        popupShape: RoundedRectangleBorder(
                            side: BorderSide(
                                width: 1.0,
                                color: Theme.of(context).colorScheme.secondary),
                            borderRadius:
                                BorderRadius.all(Radius.circular(10))),
                        items: results,
                        itemAsString: (ds) => ds["name"].toString(),
                        compareFn: (a, b) => (a ?? {})["id"] == (b ?? {})["id"],
                        onChanged: (ds) {
                          datasetId = ds["id"];
                        },
                        dropdownSearchDecoration: InputDecoration(
                            contentPadding:
                                EdgeInsets.only(top: 10, bottom: -5)),
                        hint: "Dataset"),
                    Padding(
                        padding: EdgeInsets.all(5),
                        child: Row(
                            mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                            children: <Widget>[
                              FlatButton(
                                  child: Text(
                                    "CANCEL",
                                    style: TextStyle(
                                        color: Theme.of(context)
                                            .colorScheme
                                            .secondary),
                                  ),
                                  onPressed: () {
                                    Navigator.of(context).pop();
                                  }),
                              FlatButton(
                                  child: Text(
                                    "CREATE",
                                    style: TextStyle(
                                        color: Theme.of(context)
                                            .colorScheme
                                            .secondary),
                                  ),
                                  onPressed: () async {
                                    if (_formKey.currentState.validate()) {
                                      _formKey.currentState.save();
                                      final queryOpts = QueryOptions(
                                          fetchPolicy: FetchPolicy.networkOnly,
                                          documentNode:
                                              gql(widget.createAnalysis),
                                          variables: {
                                            "datasetId": datasetId,
                                            "name": name
                                          });
                                      final res =
                                          await widget.client.query(queryOpts);
                                      final analysis =
                                          res.data["createAnalysis"];
                                      Navigator.of(context).pop();
                                      widget.nav.push(
                                          analysis["name"].toString(),
                                          a.Analysis(
                                              nav: widget.nav,
                                              analysisId:
                                                  analysis["id"].toString()));
                                      globals.refetch();
                                    }
                                  })
                            ]))
                  ])));
        });
  }
}

Adder makeAdder(String projectId) {
  void add(BuildContext context) {
    final nav = Provider.of<Nav>(context, listen: false);
    final client = GraphQLProvider.of(context).value;
    final form =
        NewAnalysisForm(nav: nav, client: client, projectId: projectId);
    formDialog(context: context, title: 'New Analysis', form: form);
  }

  return add;
}
