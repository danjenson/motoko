import 'package:flutter/material.dart';

class ScrollableTable extends StatelessWidget {
  ScrollableTable({@required this.schema, @required this.rows});
  final Map<String, String> schema;
  final List<Map<String, String>> rows;
  @override
  Widget build(BuildContext context) {
    return Expanded(
        child: Scrollbar(
            child: SingleChildScrollView(
                scrollDirection: Axis.vertical,
                child: SingleChildScrollView(
                  scrollDirection: Axis.horizontal,
                  child: DataTable(
                    columns: schema.keys
                        .map<DataColumn>((k) => DataColumn(
                            label: Text(k,
                                style: TextStyle(fontStyle: FontStyle.italic))))
                        .toList(),
                    rows: rows
                        .map((row) => DataRow(
                            cells: schema.keys
                                .map<DataCell>(
                                    (key) => DataCell(Text(row[key])))
                                .toList()))
                        .toList(),
                  ),
                ))));
  }
}
