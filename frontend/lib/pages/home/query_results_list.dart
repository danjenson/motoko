import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import '../../common/error_dialog.dart';
import '../../common/globals.dart' as globals;
import '../../common/types.dart';
import 'searchable_list.dart';

typedef void OnTap(dynamic v);

class QueryResultsList extends StatelessWidget {
  QueryResultsList({
    @required this.query,
    @required this.variables,
    @required this.getter,
    @required this.onTap,
  });
  final String query;
  final Map<String, dynamic> variables;
  final deleteQuery = '''
    mutation DeleteNode(\$id: ID!) {
      deleteNode(id: \$id)
    }
  ''';
  final Getter getter;
  final OnTap onTap;
  var plannedRefetch = false;
  @override
  Widget build(BuildContext context) {
    return Query(
        options: QueryOptions(
          fetchPolicy: FetchPolicy.cacheAndNetwork,
          documentNode: gql(query),
          variables: variables,
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
            results = getter(result.data) ?? [];
          }
          // TODO(danj): QueryResultSource.Loading means no cache no network
          // https://github.com/zino-app/graphql-flutter/issues/603
          // https://github.com/zino-app/graphql-flutter/issues/153
          plannedRefetch = false;
          List<Widget> items = results.map<Widget>((v) {
            // some objects don't have a status field, so it's
            // safe to delete; others must be finished in order
            // to delete; i.e. if a dataset hasn't finished
            // uploading and you try to delete it, it will try
            // to a dataset before it exists
            final status = v["status"].toString();
            final isError = status == "FAILED";
            final inProgress =
                ["QUEUED", "RUNNING"].contains(v["status"].toString());
            if (inProgress && !plannedRefetch) {
              Future.delayed(Duration(milliseconds: 5000), refetch);
              plannedRefetch = true;
            }
            return Card(
                elevation: 3.0,
                child: ListTile(
                    onTap: () => onTap(v),
                    trailing: Wrap(spacing: 12, children: <Widget>[
                      Visibility(
                          visible: isError,
                          child: IconButton(
                              onPressed: () => errorDialog(
                                  context: context,
                                  message:
                                      '''Error uploading dataset. Check to ensure it is a csv.'''),
                              icon: Icon(Icons.error, size: 30))),
                      IconButton(
                          onPressed: () async {
                            if (!inProgress) {
                              final client = GraphQLProvider.of(context).value;
                              final queryOpts = QueryOptions(
                                fetchPolicy: FetchPolicy.networkOnly,
                                documentNode: gql(deleteQuery),
                                variables: {"id": v["id"].toString()},
                              );
                              var res = await client.query(queryOpts);
                              if (res.hasException) {
                                errorDialog(
                                    context: context,
                                    message: res.exception.toString());
                              }
                              refetch();
                            }
                          },
                          icon: inProgress
                              ? Icon(Icons.cloud_upload, size: 30.0)
                              : Icon(Icons.delete_outline, size: 30.0))
                    ]),
                    title: Text(v['name'].toString(),
                        style: TextStyle(fontSize: 20))));
          }).toList();

          return SearchableList(
              items: items,
              getter: (item) => item.child.title.data,
              loading: loading);
        });
  }
}
