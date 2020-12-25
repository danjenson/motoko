import 'package:flutter/material.dart';
import 'searchable_list.dart';

class Statistics extends StatelessWidget {
  Statistics(this.dataviewId);
  final String dataviewId;
  final List<String> statisticIds = ["stat 1", "stat 2", "stat 3"];
  @override
  Widget build(BuildContext context) {
    return SearchableList(
        items: statisticIds
            .map((statisticId) =>
                Card(elevation: 3.0, child: ListTile(title: Text(statisticId))))
            .toList(),
        getter: (item) => item.child.title.data);
  }
}

void add(context) {
  showDialog(
      context: context,
      builder: (BuildContext context) {
        return AlertDialog(
          title: Text('New Statistic',
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
