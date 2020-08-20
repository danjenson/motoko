import 'package:flutter/material.dart';
import 'searchable_list.dart';
import 'analysis.dart' as a;
import 'nav.dart';

class Analyses extends StatelessWidget {
  Analyses({@required this.nav, @required this.projectID});
  final Nav nav;
  final String projectID;
  // TODO(danj): fetch datasets using datasets(projectID) or something;
  final List<String> analyses = ["analysis 1", "analysis 2", "analysis 3"];
  @override
  Widget build(BuildContext context) {
    return SearchableList(
        items: analyses
            .map((analysisID) => Card(
                elevation: 3.0,
                child: ListTile(
                    onTap: () => nav.push(analysisID,
                        a.Analysis(nav: nav, analysisID: analysisID)),
                    title: Text(analysisID))))
            .toList(),
        getter: (item) => item.child.title.data);
  }
}

void add(context) {
  showDialog(
      context: context,
      builder: (BuildContext context) {
        return AlertDialog(
          title: Text('New Analysis',
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
