import 'package:flutter/material.dart';
import 'searchable_list.dart';
import 'preview_panel.dart';
import 'scrollable_table.dart';

class Datasets extends StatelessWidget {
  final String projectID;
  Datasets({@required this.projectID});
  // TODO(danj): fetch datasets using datasets(projectID) or something;
  final List<String> datasetIDs = ['dataset 1', 'dataset 2', 'dataset 3'];
  final schema = {
    'Name': 'string',
    'Age': 'int',
    'Role': 'enum',
    'Color': 'enum',
    'Size': 'float',
  };
  final rows = List.filled(20, {
    'Name': 'Sarah',
    'Age': '19',
    'Role': 'Student',
    'Color': 'Red',
    'Size': '10'
  });
  @override
  Widget build(BuildContext context) {
    return PreviewPanel(
        main: SearchableList(
            items: datasetIDs
                .map((datasetID) => Card(
                    elevation: 3.0, child: ListTile(title: Text(datasetID))))
                .toList(),
            getter: (item) => item.child.title.data),
        preview: ScrollableTable(schema: schema, rows: rows));
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
