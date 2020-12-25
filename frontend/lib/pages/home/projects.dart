import '../../common/form_dialog.dart';
import '../../common/globals.dart' as globals;
import 'nav.dart';
import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:provider/provider.dart';
import 'project.dart' as p;
import 'query_results_list.dart';

class Projects extends StatelessWidget {
  Projects({@required this.nav});
  final Nav nav;
  final query = '''
    query {
      projects {
        __typename
        id
        createdAt
        updatedAt
        name
      }
    }
  ''';
  @override
  Widget build(BuildContext context) {
    final onTap = (dynamic proj) => nav.push(proj["name"].toString(),
        p.Project(nav: nav, projectId: proj["id"].toString()));
    return QueryResultsList(
      query: query,
      variables: {},
      getter: (v) => v["projects"],
      title: (v) => v['name'],
      onTap: onTap,
    );
  }
}

class NewProjectForm extends StatefulWidget {
  NewProjectForm({@required this.nav, @required this.client});
  final Nav nav;
  final GraphQLClient client;
  @override
  _NewProjectFormState createState() => _NewProjectFormState();
}

class _NewProjectFormState extends State<NewProjectForm> {
  final createProject = '''
    mutation CreateProject(\$name: String!) {
      createProject(name: \$name) {
        __typename
        id
        createdAt
        updatedAt
        name
      }
    }
  ''';
  String name;
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
                  name = value;
                },
                decoration: InputDecoration(isDense: true, hintText: "Name"),
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
                            final queryOpts = QueryOptions(
                                fetchPolicy: FetchPolicy.networkOnly,
                                documentNode: gql(createProject),
                                variables: {"name": name});
                            final res = await widget.client.query(queryOpts);
                            final project = res.data["createProject"];
                            Navigator.of(context).pop();
                            widget.nav.push(
                                project["name"].toString(),
                                p.Project(
                                    nav: widget.nav,
                                    projectId: project["id"].toString()));
                            globals.refetch();
                          }
                        })
                  ]))
        ]));
  }
}

void add(BuildContext context) {
  final nav = Provider.of<Nav>(context, listen: false);
  final client = GraphQLProvider.of(context).value;
  final form = NewProjectForm(nav: nav, client: client);
  formDialog(context: context, title: 'New Project', form: form);
}
