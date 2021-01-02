import '../../../../common/dialogs.dart';
import '../../../../common/utils.dart';
import 'package:flutter/material.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

class CreateStatisticForm extends StatefulWidget {
  CreateStatisticForm(this.setFormState, this.dataviewId);
  final void Function(Map<String, dynamic>, bool Function()) setFormState;
  final String dataviewId;
  final String schema = '''
    query Node(\$id: ID!) {
      node(id: \$id) {
        __typename
        id
        ... on Dataview {
          schema {
            columnName
            dataType
          }
        }
      }
    }
  ''';
  @override
  _CreateStatisticFormState createState() => _CreateStatisticFormState();
}

class _CreateStatisticFormState extends State<CreateStatisticForm> {
  final _formKey = GlobalKey<FormBuilderState>();
  String _statType;
  @override
  Widget build(BuildContext context) {
    return Query(
        options: QueryOptions(
          fetchPolicy: FetchPolicy.cacheAndNetwork,
          documentNode: gql(widget.schema),
          variables: {'id': widget.dataviewId},
        ),
        builder: (QueryResult result,
            {VoidCallback refetch, FetchMore fetchMore}) {
          var schema = [];
          if (result.hasException) {
            showErrorDialog(context, result.exception.toString());
          } else if (!result.loading) {
            schema = result.data['node']['schema'] ?? [];
          }
          return FormBuilder(
              key: _formKey,
              autovalidateMode: AutovalidateMode.disabled,
              skipDisabled: true,
              onChanged: () {
                _formKey.currentState.save();
                widget.setFormState(Map.from(_formKey.currentState.value),
                    _formKey.currentState.validate);
              },
              child: Column(children: <Widget>[
                FormBuilderDropdown<String>(
                  name: 'type',
                  focusColor: Theme.of(context).colorScheme.secondary,
                  decoration:
                      InputDecoration(hintText: 'type', labelText: 'type'),
                  validator: FormBuilderValidators.required(context),
                  items: [
                    'Correlation',
                    'Summary',
                  ]
                      .map((v) => DropdownMenuItem(value: v, child: Text(v)))
                      .toList(),
                  onChanged: (v) => setState(() {
                    _statType = v;
                  }),
                ),
                FormBuilderDropdown<String>(
                    name: 'x',
                    decoration: InputDecoration(hintText: 'x', labelText: 'x'),
                    validator: FormBuilderValidators.required(context),
                    items: schema
                        .where((v) {
                          if (['Correlation', 'Summary'].contains(_statType)) {
                            return isNumericDataType(v['dataType']);
                          }
                          return false;
                        })
                        .map((v) => DropdownMenuItem(
                            value: v['columnName'].toString(),
                            child: Text(v['columnName'])))
                        .toList()),
                Visibility(
                    visible: ['Correlation'].contains(_statType),
                    child: FormBuilderDropdown<String>(
                        name: 'y',
                        enabled: ['Correlation'].contains(_statType),
                        decoration:
                            InputDecoration(hintText: 'y', labelText: 'y'),
                        validator: FormBuilderValidators.required(context),
                        items: schema
                            .where((v) => isNumericDataType(v['dataType']))
                            .map((v) => DropdownMenuItem(
                                value: v['columnName'].toString(),
                                child: Text(v['columnName'])))
                            .toList())),
              ]));
        });
  }
}
