import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'searchable_list.dart';
import 'project.dart' as p;
import 'nav.dart';

class Projects extends StatelessWidget {
  Projects({@required this.nav});
  final Nav nav;
  final query = '''
    query ReadRepositories(\$nRepositories: Int!) {
      viewer {
        repositories(last: \$nRepositories) {
          nodes {
            id
            name
            viewerHasStarred
          }
        }
      }
    }
  ''';
  final variables = {
    'nRepositories': 50,
  };
  @override
  Widget build(BuildContext context) {
    return Query(
        options: QueryOptions(
          fetchPolicy: FetchPolicy.cacheAndNetwork,
          documentNode: gql(query), // this is the query string you just created
          variables: variables,
        ),
        builder: (QueryResult result,
            {VoidCallback refetch, FetchMore fetchMore}) {
          if (result.hasException) {
            return Text(result.exception.toString());
          }
          // TODO(danj): QueryResultSource.Loading means no cache no network
          // https://github.com/zino-app/graphql-flutter/issues/603
          // https://github.com/zino-app/graphql-flutter/issues/153
          var loading =
              result.loading && result.source == QueryResultSource.Loading;
          List<Widget> items = loading
              ? []
              : result.data['viewer']['repositories']['nodes']
                  .map<Widget>((repo) => Card(
                      elevation: 3.0,
                      child: ListTile(
                          onTap: () {
                            var projectID = repo['name'].toString();
                            nav.push(projectID,
                                p.Project(nav: nav, projectID: projectID));
                          },
                          title: Text(repo['name'].toString()))))
                  .toList();

          return SearchableList(
              items: items,
              getter: (item) => item.child.title.data,
              loading: loading);
        });
  }
}

void add(BuildContext context) {
  showDialog(
      context: context,
      builder: (BuildContext context) {
        return AlertDialog(
          title: Text('New Project',
              textAlign: TextAlign.center,
              style: TextStyle(color: Theme.of(context).colorScheme.secondary)),
          content: Form(
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: <Widget>[
                Padding(
                    padding: EdgeInsets.all(5),
                    child: TextFormField(
                        decoration: InputDecoration(hintText: 'Name'))),
              ],
            ),
          ),
        );
      });
}
