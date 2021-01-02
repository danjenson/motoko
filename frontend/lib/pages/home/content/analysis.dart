import '../../../common/utils.dart';
import '../components/content.dart';
import '../components/data_preview.dart';
import 'forms/create_dataview.dart';
import 'models.dart';
import 'plots.dart';
import 'statistics.dart';
import 'dart:convert';
import 'package:flutter/material.dart';

class Analysis extends StatelessWidget {
  Analysis(this.id);
  final String id;
  final query = '''
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
        nRows
      }
    }
  ''';
  final mutation = '''
    mutation CreateDataview(
      \$analysisId: ID!,
      \$operation: Operation!,
      \$args: JSON!,
    ) {
      createDataview(
        analysisId: \$analysisId,
        operation: \$operation,
        args: \$args,
      ) {
        __typename
        id
        createdAt
        updatedAt
        operation
        args
        status
      }
    }
  ''';
  @override
  Widget build(BuildContext context) {
    return Content(
      listQuery: query,
      listQueryVariables: {'analysisId': id},
      toTitleString: (v) {
        if (v['operation'] == 'CREATE') {
          return v['analysis']['dataset']['name'];
        }
        return v['operation'].toString().toLowerCase().capitalize();
      },
      toSecondarySubtitleString: (v) {
        var s = '';
        if (v['args'] != null) {
          var args = jsonDecode(v['args']) as Map;
          var sep = '\n   ';
          if (v['operation'] == 'SELECT') {
            var cols = args['columns'].join(sep);
            s = 'columns:$sep$cols\n';
          } else if (v['operation'] == 'FILTER') {
            var filters = args['filters']
                .map((v) => '${v['column']} ${v['comparator']} ${v['value']}')
                .join(sep);
            s = 'filters:$sep$filters\n';
          } else if (v['operation'] == 'SORT') {
            var sorts = args['sorts']
                .map((v) => '${v['column']} ${v['order'].toLowerCase()}')
                .join(sep);
            s = 'sort by:$sep$sorts\n';
          } else if (v['operation'] == 'SUMMARIZE') {
            var summaries = args['summaries']
                .map(
                    (v) => '${v['column']} by ${v['summarizer'].toLowerCase()}')
                .join(sep);
            var groupBys = args['groupBys'].join(sep);
            s = 'summarize:$sep$summaries\ngrouping by:$sep$groupBys\n';
          } else {
            s = args.entries.map((m) => '${m.key}: ${m.value}').join('\n') +
                '\n';
          }
        }
        if (v['nRows'] == null) {
          return s;
        }
        return '${s}rows: ${nCompact(v['nRows'])}';
      },
      toButtons: (v, current) => [
        FlatButton.icon(
            icon: Icon(Icons.functions),
            label: Text("statistics"),
            onPressed: () => current.push(Statistics(v['id']), 'Statistics')),
        FlatButton.icon(
            icon: Icon(Icons.equalizer),
            label: Text("plots"),
            onPressed: () => current.push(Plots(v['id']), 'Plots')),
        FlatButton.icon(
            icon: Icon(Icons.scatter_plot),
            label: Text("models"),
            onPressed: () => current.push(Models(v['id']), 'Models')),
      ],
      canDelete: (v, _) => v['operation'] != 'CREATE',
      orderBy: (v) => DateTime.parse(v['createdAt']),
      onTapPreview: (id) => DataPreview(id),
      defaultPreviewString: 'Tap a dataview.',
      createName: 'Dataview',
      makeCreateForm: (setFormState, _v) =>
          CreateDataviewForm(setFormState, id),
      createMutation: mutation,
      createFieldsToVariables: (fields) {
        var op = fields.remove('operation').toString().toUpperCase();
        return {'analysisId': id, 'operation': op, 'args': jsonEncode(fields)};
      },
      onCreate: (_v, _c, refetch) => refetch(),
      createOnLastFailureMessage:
          'Must clear previous failure before creating a new dataview.',
      createOnLastWorkingMessage: 'Must wait for previous operation to finish.',
    );
  }
}
