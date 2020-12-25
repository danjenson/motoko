import '../../common/utils.dart';
import 'dart:convert';
import 'data_preview.dart';
import 'models.dart' as m;
import 'nav.dart';
import 'package:flutter/material.dart';
import 'package:sliding_up_panel/sliding_up_panel.dart';
import 'plots.dart' as p;
import 'preview_panel.dart';
import 'query_results_list.dart';
import 'statistics.dart' as s;

class Analysis extends StatefulWidget {
  Analysis({@required this.nav, @required this.analysisId});
  final Nav nav;
  final String analysisId;
  final dataviews = '''
    query Dataviews(\$analysisId: ID!) {
      dataviews(analysisId: \$analysisId) {
        __typename
        id
        createdAt
        updatedAt
        analysis {
          __typename
          id
          dataset {
            __typename
            id
            name
          }
        }
        operation
        args
        status
      }
    }
  ''';
  @override
  _AnalysisState createState() => _AnalysisState();
}

class _AnalysisState extends State<Analysis> {
  String _selectedId;
  PanelController _controller = PanelController();
  @override
  Widget build(BuildContext context) {
    return PreviewPanel(
        controller: _controller,
        main: QueryResultsList(
          query: widget.dataviews,
          variables: {'analysisId': widget.analysisId},
          getter: (v) => v['dataviews'],
          title: (v) {
            if (v['operation'] == 'CREATE') {
              return v['analysis']['dataset']['name'];
            }
            return v['operation'].toString().toLowerCase().capitalize();
          },
          subtitle: (v) {
            if (v['args'] == null) {
              return null;
            }
            var args = jsonDecode(v['args']) as Map;
            var s = args.entries.map((m) => "${m.key}: ${m.value}").join("\n");
            return Text(s, style: TextStyle(color: Colors.grey));
          },
          canDelete: (v) => v['operation'] != 'CREATE',
          below: (v) {
            final dataviewId = v['id'].toString();
            return Row(
                mainAxisAlignment: MainAxisAlignment.spaceAround,
                children: <Widget>[
                  FlatButton.icon(
                      icon: Icon(Icons.functions),
                      label: Text("statistics"),
                      onPressed: () => widget.nav
                          .push("statistics", s.Statistics(dataviewId), s.add)),
                  FlatButton.icon(
                      icon: Icon(Icons.equalizer),
                      label: Text("plots"),
                      onPressed: () => widget.nav.push("plots",
                          p.Plots(dataviewId), p.makeAdder(dataviewId))),
                  FlatButton.icon(
                      icon: Icon(Icons.scatter_plot),
                      label: Text("models"),
                      onPressed: () => widget.nav
                          .push("models", m.Models(dataviewId), m.add)),
                ]);
          },
          onTap: (v) {
            this.setState(() {
              _selectedId = v["id"].toString();
              _controller.animatePanelToPosition(1.0);
            });
          },
          selectedId: _selectedId,
        ),
        preview: _selectedId == null
            ? Expanded(
                child: Center(
                    child: Text("Tap a dataview for sample rows.",
                        style: TextStyle(fontSize: 20.0))))
            : DataPreview(id: _selectedId));
  }
}
