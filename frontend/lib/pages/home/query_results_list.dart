import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import '../../common/error_dialog.dart';
import '../../common/globals.dart' as globals;
import '../../common/types.dart';
import 'searchable_list.dart';

typedef bool Predicate(dynamic v);
typedef void Action(dynamic v);
typedef Widget WidgetFunc(dynamic v);

class QueryResultsList extends StatelessWidget {
  QueryResultsList({
    @required this.query,
    @required this.variables,
    @required this.getter,
    @required this.title,
    @required this.onTap,
    this.subtitle,
    this.below,
    this.selectedId,
    this.canDelete,
  });
  final String query;
  final Map<String, dynamic> variables;
  final deleteQuery = '''
    mutation DeleteNode(\$id: ID!) {
      deleteNode(id: \$id)
    }
  ''';
  final Getter getter;
  final Getter title;
  final Action onTap;
  final WidgetFunc subtitle;
  final WidgetFunc below;
  final String selectedId;
  final Predicate canDelete;
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
          if (!result.loading && result.data != null) {
            results = getter(result.data) ?? [];
          }
          var shouldRefetch = false;
          List<Widget> items = results.map<Widget>((v) {
            final status = v["status"].toString();
            final isError = status == "FAILED" || result.hasException;
            final inProgress =
                ["QUEUED", "RUNNING"].contains(v["status"].toString());
            shouldRefetch = shouldRefetch || inProgress;
            var children = <Widget>[
              ListTile(
                selected: v["id"].toString() == selectedId,
                onTap: () => onTap(v),
                trailing: Wrap(spacing: 12, children: <Widget>[
                  Visibility(
                      visible: isError || inProgress,
                      child: isError
                          ? IconButton(
                              onPressed: () => errorDialog(
                                  context: context,
                                  message: result.exception.toString()),
                              icon: Icon(Icons.error, size: 30))
                          : IconButton(
                              onPressed: () => {},
                              icon: Icon(Icons.cloud_upload, size: 30))),
                  Visibility(
                      visible: canDelete?.call(v) ?? true,
                      child: IconButton(
                          onPressed: () async {
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
                          },
                          icon: Icon(Icons.delete_outline, size: 30.0)))
                ]),
                title: Text(title(v), style: TextStyle(fontSize: 20)),
                subtitle: subtitle?.call(v),
              )
            ];
            if (this.below != null) {
              children.add(this.below(v));
            }
            return Card(elevation: 0.0, child: Column(children: children));
          }).toList();

          if (shouldRefetch && result.source == QueryResultSource.Network) {
            Future.delayed(Duration(milliseconds: 10000), refetch);
          }

          return SearchableList(
              items: items,
              getter: (item) => item.child.children[0].title.data,
              loading: result.loading);
        });
  }
}
