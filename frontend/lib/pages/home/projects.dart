import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'searchable_list.dart';
import 'project.dart' as p;
import '../../common/error_dialog.dart';
import 'nav.dart';

class Projects extends StatelessWidget {
  Projects({@required this.nav});
  final Nav nav;
  final query = '''
    query {
      me {
        projects {
          __typename
          id
          createdAt
          updatedAt
          name
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
          var projects = [];
          var loading =
              result.loading && result.source == QueryResultSource.Loading;
          if (result.hasException) {
            errorDialog(context: context, message: result.exception.toString());
          } else if (!loading) {
            projects = result.data['projects'] ?? [];
          }
          // TODO(danj): QueryResultSource.Loading means no cache no network
          // https://github.com/zino-app/graphql-flutter/issues/603
          // https://github.com/zino-app/graphql-flutter/issues/153
          List<Widget> items = projects
              .map<Widget>((p) => Card(
                  elevation: 3.0,
                  child: ListTile(
                      onTap: () {
                        var projectID = p['name'].toString();
                        nav.push(projectID,
                            p.Project(nav: nav, projectID: projectID));
                      },
                      title: Text(p['name'].toString()))))
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
