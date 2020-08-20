import 'package:flutter/material.dart';
import 'preview_panel.dart';
import 'scrollable_table.dart';

class Data extends StatelessWidget {
  Data({@required this.analysisID});
  final String analysisID;
  final List<String> items = [
    "filter x > 5",
    "summarize y with sum as z group by x",
    "select z",
  ];
  final schema = {
    'Name': 'string',
    'Age': 'int',
    'Role': 'enum',
    'Color': 'enum',
    'Size': 'float',
  };
  final rows = List.filled(100, {
    'Name': 'Sarah',
    'Age': '19',
    'Role': 'Student',
    'Color': 'Red',
    'Size': '10'
  });

  @override
  Widget build(BuildContext context) {
    return PreviewPanel(
        main: ListView(
            padding: EdgeInsets.all(20),
            children: items
                .map((opID) =>
                    Card(elevation: 3.0, child: ListTile(title: Text(opID))))
                .toList()),
        preview: ScrollableTable(schema: schema, rows: rows));
  }
}

void add(BuildContext context) {
  return;
}
