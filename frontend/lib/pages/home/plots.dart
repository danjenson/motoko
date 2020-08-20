import 'package:flutter/material.dart';
import 'searchable_list.dart';
import 'plot.dart' as p;
import 'nav.dart';

class Plots extends StatelessWidget {
  Plots({@required this.nav, @required this.analysisID});
  final Nav nav;
  final String analysisID;
  final List<String> plots = ["plot 1", "plot 2", "plot 3"];
  @override
  Widget build(BuildContext context) {
    return SearchableList(
        items: plots
            .map((plotID) => Card(
                elevation: 3.0,
                child: ListTile(
                    onTap: () => nav.push(plotID, p.Plot(plotID: plotID)),
                    title: Text(plotID))))
            .toList(),
        getter: (item) => item.child.title.data);
  }
}

void add(context) {
  showDialog(
      context: context,
      builder: (BuildContext context) {
        return AlertDialog(
          title: Text('New Plot',
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
