import 'package:flutter/material.dart';
import 'searchable_list.dart';
import 'model.dart' as m;
import 'nav.dart';

class Models extends StatelessWidget {
  Models({@required this.nav, @required this.analysisID});
  final Nav nav;
  final String analysisID;
  final List<String> modelIDs = ["model 1", "model 2", "model 3"];
  @override
  Widget build(BuildContext context) {
    return SearchableList(
        items: modelIDs
            .map((modelID) => Card(
                elevation: 3.0,
                child: ListTile(
                    onTap: () => nav.push(modelID, m.Model(modelID: modelID)),
                    title: Text(modelID))))
            .toList(),
        getter: (item) => item.child.title.data);
  }
}

void add(context) {
  showDialog(
      context: context,
      builder: (BuildContext context) {
        return AlertDialog(
          title: Text('New Model',
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
