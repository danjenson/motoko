import 'package:flutter/material.dart';
import 'searchable_list.dart';
import 'statistic.dart' as s;
import 'nav.dart';

class Statistics extends StatelessWidget {
  Statistics({@required this.nav, @required this.analysisID});
  final Nav nav;
  final String analysisID;
  final List<String> statIDs = ["stat 1", "stat 2", "stat 3"];
  @override
  Widget build(BuildContext context) {
    return SearchableList(
        items: statIDs
            .map((statID) => Card(
                elevation: 3.0,
                child: ListTile(
                    onTap: () => nav.push(statID, s.Statistic(statID: statID)),
                    title: Text(statID))))
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
