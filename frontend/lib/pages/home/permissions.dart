import 'package:flutter/material.dart';
import 'searchable_list.dart';

class Permissions extends StatelessWidget {
  Permissions({@required this.projectId});
  final String projectId;
  // TODO(danj): fetch using permissions(projectId)
  final List<String> permissions = ['user 1', 'user 2', 'user 3'];
  @override
  Widget build(BuildContext context) {
    return SearchableList(
        items: permissions
            .map((permission) =>
                Card(elevation: 3.0, child: ListTile(title: Text(permission))))
            .toList(),
        getter: (item) => item.child.title.data);
  }
}

void add(context) {
  showDialog(
      context: context,
      builder: (BuildContext context) {
        return AlertDialog(
          title: Text('Add User',
              textAlign: TextAlign.center,
              style: TextStyle(color: Theme.of(context).colorScheme.secondary)),
          content: Form(
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: <Widget>[
                Padding(
                    padding: EdgeInsets.all(5),
                    child: TextFormField(
                        decoration: InputDecoration(hintText: 'User'))),
                Padding(
                    padding: EdgeInsets.all(5),
                    child: TextFormField(
                        decoration: InputDecoration(hintText: 'Role'))),
              ],
            ),
          ),
        );
      });
}
