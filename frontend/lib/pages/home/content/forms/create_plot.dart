import '../../../../common/dialogs.dart';
import '../../../../common/utils.dart';
import 'package:flutter/material.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

class CreatePlotForm extends StatefulWidget {
  CreatePlotForm(this.setFormState, this.dataviewId);
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
  _CreatePlotFormState createState() => _CreatePlotFormState();
}

class _CreatePlotFormState extends State<CreatePlotForm> {
  final _formKey = GlobalKey<FormBuilderState>();
  String _plotType;
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
                FormBuilderTextField(
                  name: 'title',
                  validator: FormBuilderValidators.required(context),
                  decoration:
                      InputDecoration(hintText: 'title', labelText: 'title'),
                ),
                FormBuilderDropdown<String>(
                  name: 'type',
                  decoration:
                      InputDecoration(hintText: 'type', labelText: 'type'),
                  validator: FormBuilderValidators.required(context),
                  onChanged: (v) => setState(() {
                    _plotType = v;
                  }),
                  items: ['Bar', 'Histogram', 'Line', 'Scatter', 'Smooth']
                      .map((v) => DropdownMenuItem(value: v, child: Text(v)))
                      .toList(),
                ),
                FormBuilderDropdown<String>(
                    name: 'x',
                    decoration: InputDecoration(
                        hintText: 'x-axis', labelText: 'x-axis'),
                    validator: FormBuilderValidators.required(context),
                    items: schema
                        .where((v) {
                          if (['Histogram', 'Line', 'Scatter', 'Smooth']
                              .contains(_plotType)) {
                            return isNumericDataType(v['dataType']);
                          } else if (_plotType == 'Bar') {
                            return isCategoricalDataType(v['dataType']);
                          }
                          return false;
                        })
                        .map((v) => DropdownMenuItem(
                            value: v['columnName'].toString(),
                            child: Text(v['columnName'])))
                        .toList()),
                Visibility(
                    visible: !['Bar', 'Histogram'].contains(_plotType),
                    child: FormBuilderDropdown<String>(
                        name: 'y',
                        enabled: !['Bar', 'Histogram'].contains(_plotType),
                        decoration: InputDecoration(
                            hintText: 'y-axis', labelText: 'y-axis'),
                        validator: FormBuilderValidators.required(context),
                        items: schema
                            .where((v) => isNumericDataType(v['dataType']))
                            .map((v) => DropdownMenuItem(
                                value: v['columnName'].toString(),
                                child: Text(v['columnName'])))
                            .toList())),
                Visibility(
                    visible: _plotType != 'Histogram',
                    child: FormBuilderDropdown<String>(
                        name: 'color',
                        enabled: _plotType != 'Histogram',
                        decoration: InputDecoration(
                            hintText: '[color]', labelText: '[color]'),
                        allowClear: true,
                        items: schema
                            .where((v) => isCategoricalDataType(v['dataType']))
                            .map((v) => DropdownMenuItem(
                                value: v['columnName'].toString(),
                                child: Text(v['columnName'])))
                            .toList())),
                Visibility(
                    visible: ['Scatter', 'Smooth'].contains(_plotType),
                    child: FormBuilderDropdown<String>(
                        name: 'shape',
                        enabled: ['Scatter', 'Smooth'].contains(_plotType),
                        decoration: InputDecoration(
                            hintText: '[shape]', labelText: '[shape]'),
                        allowClear: true,
                        items: schema
                            .where((v) => isCategoricalDataType(v['dataType']))
                            .map((v) => DropdownMenuItem(
                                value: v['columnName'].toString(),
                                child: Text(v['columnName'])))
                            .toList())),
              ]));
        });
  }
}
