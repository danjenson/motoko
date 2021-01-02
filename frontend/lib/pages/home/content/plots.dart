import '../components/content.dart';
import '../components/plot_preview.dart';
import 'dart:convert';
import 'forms/create_plot.dart';
import 'package:flutter/material.dart';

class Plots extends StatelessWidget {
  Plots(this.dataviewId);
  final String dataviewId;
  final query = '''
    query Plots(\$dataviewId: ID!) {
      plots(dataviewId: \$dataviewId) {
        __typename
        id
        createdAt
        updatedAt
        name
        type
        args
        status
      }
    }
  ''';
  final mutation = '''
    mutation CreatePlot(
      \$dataviewId: ID!,
      \$name: String!,
      \$type: PlotType!,
      \$args: JSON!,
    ) {
      createPlot(
        dataviewId: \$dataviewId,
        name: \$name,
        type: \$type,
        args: \$args,
      ) {
        __typename
        id
        createdAt
        updatedAt
        name
        type
        args
        status
      }
    }
  ''';
  @override
  Widget build(BuildContext context) {
    return Content(
      listQuery: query,
      listQueryVariables: {'dataviewId': dataviewId},
      toTitleString: (v) => v['name'],
      toSecondarySubtitleString: (v) {
        if (v['args'] == null) {
          return '';
        }
        var args = jsonDecode(v['args']) as Map;
        return 'type: ${v['type'].toLowerCase()}\n' +
            ['title', 'x', 'y', 'color', 'shape']
                .where((v) => args.containsKey(v))
                .map((v) => '$v: ${args[v]}')
                .join('\n');
      },
      onTapPreview: (id) => PlotPreview(id),
      defaultPreviewString: 'Tap a plot.',
      createName: 'Plot',
      makeCreateForm: (setFormState, _v) =>
          CreatePlotForm(setFormState, dataviewId),
      createMutation: mutation,
      createFieldsToVariables: (fields) {
        var type = fields.remove('type').toUpperCase();
        return {
          'dataviewId': dataviewId,
          'name': fields['title'],
          'type': type,
          'args': jsonEncode(fields),
        };
      },
      onCreate: (_v, _c, refetch) => refetch(),
    );
  }
}
