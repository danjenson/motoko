import '../components/content.dart';
import 'dart:convert';
import 'forms/create_model.dart';
import 'package:flutter/material.dart';

class Models extends StatelessWidget {
  Models(this.dataviewId);
  final String dataviewId;
  final query = '''
    query Models(\$dataviewId: ID!) {
      models(dataviewId: \$dataviewId) {
        __typename
        id
        createdAt
        updatedAt
        name
        target
        features
        args
        status
        evaluation
        decisions
        error
      }
    }
  ''';
  final mutation = '''
    mutation CreateModel(
      \$dataviewId: ID!,
      \$name: String!,
      \$target: String,
      \$features: [String!]!,
      \$args: JSON,
    ) {
      createModel(
        dataviewId: \$dataviewId,
        name: \$name,
        target: \$target,
        features: \$features,
        args: \$args,
      ) {
        __typename
        id
        createdAt
        updatedAt
        name
        target
        features
        args
        status
        evaluation
        decisions
        error
      }
    }
  ''';
  @override
  Widget build(BuildContext context) {
    return Content(
      listQuery: query,
      listQueryVariables: {'dataviewId': dataviewId},
      toTitleString: (v) => v['name'],
      toPrimarySubtitleString: (v) {
        if (v['target'] != null) {
          return 'target: ${v['target']}';
        }
        return 'target: none';
      },
      toSecondarySubtitleString: (v) {
        if (v['status'] == 'COMPLETED') {
          final e = v['evaluation'];
          final nss = e['normalized silhouette score'];
          if (nss != null) {
            return '\nnormalized silhouette score: ${nss.toStringAsFixed(2)}';
          }
          final accuracy = e['accuracy'];
          if (accuracy != null) {
            return '\naccuracy: ${accuracy.toStringAsFixed(2)}';
          }
          final rsq = e['R^2'];
          if (rsq != null) {
            return '\nR^2: ${rsq.toStringAsFixed(2)}';
          }
        }
        return '';
      },
      createName: 'Model',
      makeCreateForm: (setFormState, _v) =>
          CreateModelForm(setFormState, dataviewId),
      createMutation: mutation,
      createFieldsToVariables: (fields) {
        return {
          'dataviewId': dataviewId,
          'name': fields.remove('name'),
          'target': fields.remove('target'),
          'features': fields.remove('features'),
          'args': jsonEncode(fields),
        };
      },
      onCreate: (_v, _c, refetch) => refetch(),
    );
  }
}
