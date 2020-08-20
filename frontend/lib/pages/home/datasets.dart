import 'package:flutter/material.dart';
import 'searchable_list.dart';

class Datasets extends StatelessWidget {
  final String projectID;
  Datasets({@required this.projectID});
  // TODO(danj): fetch datasets using datasets(projectID) or something;
  final List<String> datasetIDs = ['dataset 1', 'dataset 2', 'dataset 3'];
  @override
  Widget build(BuildContext context) {
    return SearchableList(
        items: datasetIDs
            .map((datasetID) =>
                Card(elevation: 3.0, child: ListTile(title: Text(datasetID))))
            .toList(),
        getter: (item) => item.child.title.data);
  }
}

void add(BuildContext context) {
  showDialog(
      context: context,
      builder: (BuildContext context) {
        return AlertDialog(
          title: Text('Upload Dataset',
              textAlign: TextAlign.center,
              style: TextStyle(color: Theme.of(context).colorScheme.secondary)),
          content: Form(
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: <Widget>[
                Padding(
                    padding: EdgeInsets.all(5),
                    child: TextFormField(
                        decoration: InputDecoration(hintText: 'URL'))),
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
