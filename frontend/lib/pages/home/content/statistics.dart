import '../../../common/utils.dart';
import '../components/content.dart';
import 'dart:convert';
import 'forms/create_statistic.dart';
import 'package:flutter/material.dart';

class Statistics extends StatelessWidget {
  Statistics(this.dataviewId);
  final String dataviewId;
  final query = '''
    query Statistics(\$dataviewId: ID!) {
      statistics(dataviewId: \$dataviewId) {
        __typename
        id
        createdAt
        updatedAt
        type
        args
        status
        value
      }
    }
  ''';
  final mutation = '''
    mutation CreateStatistic(
      \$dataviewId: ID!,
      \$type: StatisticType!,
      \$args: JSON!,
    ) {
      createStatistic(
        dataviewId: \$dataviewId,
        type: \$type,
        args: \$args,
      ) {
        __typename
        id
        createdAt
        updatedAt
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
      toTitleString: (v) => v['type'].toString().toLowerCase().capitalize(),
      toPrimarySubtitleString: (v) {
        var a = jsonDecode(v['args']);
        return ['x', 'y']
            .where((v) => a.containsKey(v))
            .map((v) => '$v: ${a[v]}')
            .join('\n');
      },
      toSecondarySubtitleString: (v) {
        var value = '';
        if (v['status'] == 'COMPLETED') {
          var op = v['type'].toString();
          var m = v['value'];
          if (op == 'CORRELATION') {
            value = "\nvalue: ${m['correlation'].toStringAsFixed(2)}";
          } else if (op == 'SUMMARY') {
            value = '\n' +
                ['mean', 'median', 'mode', 'min', 'max', 'stddev']
                    .where((v) => m.containsKey(v))
                    .map((v) => '$v: ${m[v].toStringAsFixed(2)}')
                    .join('\n');
          }
        }
        return value;
      },
      createName: 'Statistic',
      makeCreateForm: (setFormState, _v) =>
          CreateStatisticForm(setFormState, dataviewId),
      createMutation: mutation,
      createFieldsToVariables: (fields) {
        var type = fields.remove('type').toUpperCase();
        return {
          'dataviewId': dataviewId,
          'type': type,
          'args': jsonEncode(fields),
        };
      },
      onCreate: (_v, _c, refetch) => refetch(),
    );
  }
}
