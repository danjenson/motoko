import 'package:flutter/material.dart';
import 'preview_panel.dart';

class Statistic extends StatelessWidget {
  Statistic({@required this.statID});
  final String statID;
  final List<String> items = ['field 1', 'field 2', 'field 3'];
  @override
  Widget build(BuildContext context) {
    return PreviewPanel(
      main: ListView(
          padding: EdgeInsets.all(20),
          children: items
              .map((opID) =>
                  Card(elevation: 3.0, child: ListTile(title: Text(opID))))
              .toList()),
      title: 'results',
      preview: Expanded(child: Container(child: Text('summary'))),
    );
  }
}
