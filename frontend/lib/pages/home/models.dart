import 'package:flutter/material.dart';
import 'searchable_list.dart';

class Models extends StatelessWidget {
  Models(this.dataviewId);
  final String dataviewId;
  final List<String> modelIds = ["model 1", "model 2", "model 3"];
  @override
  Widget build(BuildContext context) {
    return SearchableList(
        items: modelIds
            .map((modelId) =>
                Card(elevation: 3.0, child: ListTile(title: Text(modelId))))
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
