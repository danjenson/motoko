import '../../../../common/dialogs.dart';
import '../../components/checkbox_list.dart';
import 'package:flutter/material.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

class CreateModelForm extends StatefulWidget {
  CreateModelForm(this.setFormState, this.dataviewId);
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
  _CreateModelFormState createState() => _CreateModelFormState();
}

class _CreateModelFormState extends State<CreateModelForm> {
  final _formKey = GlobalKey<FormBuilderState>();
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
                  name: 'name',
                  validator: FormBuilderValidators.required(context),
                  decoration:
                      InputDecoration(hintText: 'name', labelText: 'name'),
                ),
                FormBuilderDropdown<String>(
                  name: 'target',
                  focusColor: Theme.of(context).colorScheme.secondary,
                  allowClear: true,
                  decoration: InputDecoration(
                      hintText: '[target]', labelText: '[target]'),
                  items: schema
                      .map((v) => DropdownMenuItem(
                          value: v['columnName'].toString(),
                          child: Text(v['columnName'])))
                      .toList(),
                  onChanged: (v) {
                    if (_formKey.currentState.value['features'] != null) {
                      _formKey.currentState.value['features']
                          .removeWhere((x) => x == v);
                    }
                  },
                ),
                FormBuilderField(
                    name: 'features',
                    validator: FormBuilderValidators.required(context),
                    builder: (FormFieldState<dynamic> field) {
                      return CheckboxList(
                        items: schema
                            .where((v) =>
                                v['columnName'].toString() !=
                                _formKey.currentState.value['target'])
                            .map((v) => v['columnName'].toString())
                            .toList(),
                        onChanged: (selected) {
                          field.didChange(selected);
                        },
                      );
                    }),
              ]));
        });
  }
}
