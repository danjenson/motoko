import '../../../../common/dialogs.dart';
import 'create_dataview_filter.dart';
import 'create_dataview_select.dart';
import 'create_dataview_sort.dart';
import 'create_dataview_summarize.dart';
import 'package:flutter/material.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

class CreateDataviewForm extends StatefulWidget {
  CreateDataviewForm(this.setFormState, this.analysisId);
  final void Function(Map<String, dynamic>, bool Function()) setFormState;
  final String analysisId;
  final String schema = '''
    query Node(\$id: ID!) {
      node(id: \$id) {
        __typename
        id
        ... on Analysis {
          dataview {
            __typename
            id
            schema {
              columnName
              dataType
            }
          }
        }
      }
    }
  ''';
  @override
  _CreateDataviewFormState createState() => _CreateDataviewFormState();
}

class _CreateDataviewFormState extends State<CreateDataviewForm> {
  final _formKey = GlobalKey<FormBuilderState>();
  String _opType;
  @override
  Widget build(BuildContext context) {
    return Query(
        options: QueryOptions(
            fetchPolicy: FetchPolicy.cacheAndNetwork,
            documentNode: gql(widget.schema),
            variables: {'id': widget.analysisId}),
        builder: (QueryResult result,
            {VoidCallback refetch, FetchMore fetchMore}) {
          var schema = [];
          if (result.hasException) {
            showErrorDialog(context, result.exception.toString());
          } else if (!result.loading) {
            schema = result.data['node']['dataview']['schema'] ?? [];
          }
          void setFormState(
              Map<String, dynamic> fields, bool Function() validate) {
            fields['operation'] = _opType.toString().toUpperCase();
            widget.setFormState(fields, validate);
          }

          return FormBuilder(
              key: _formKey,
              autovalidateMode: AutovalidateMode.disabled,
              skipDisabled: true,
              child: Column(children: [
                FormBuilderDropdown<String>(
                  name: 'operation',
                  focusColor: Theme.of(context).colorScheme.secondary,
                  decoration: InputDecoration(
                      hintText: 'operation', labelText: 'operation'),
                  validator: FormBuilderValidators.required(context),
                  items: [
                    'Filter',
                    // TODO(danj): add Mutate
                    'Select',
                    'Sort',
                    'Summarize',
                  ]
                      .map((v) => DropdownMenuItem(value: v, child: Text(v)))
                      .toList(),
                  onChanged: (v) => setState(() {
                    _opType = v;
                  }),
                ),
                Visibility(
                    visible: _opType == 'Filter',
                    child: CreateDataviewFilterForm(setFormState, schema)),
                Visibility(
                    visible: _opType == 'Select',
                    child: CreateDataviewSelectForm(setFormState, schema)),
                Visibility(
                    visible: _opType == 'Sort',
                    child: CreateDataviewSortForm(setFormState, schema)),
                Visibility(
                    visible: _opType == 'Summarize',
                    child: CreateDataviewSummarizeForm(setFormState, schema)),
              ]));
        });
  }
}
