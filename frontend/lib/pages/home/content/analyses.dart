import '../components/content.dart';
import 'analysis.dart';
import 'package:flutter/material.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

class Analyses extends StatelessWidget {
  Analyses(this.projectId);
  final String projectId;
  final String query = '''
    query Analyses(\$projectId: ID!) {
      analyses(projectId: \$projectId) {
        __typename
        id
        createdAt
        updatedAt
        name
        dataset {
          __typename
          id
        }
        dataview {
          __typename
          id
        }
      }
    }
  ''';
  final String mutation = '''
    mutation CreateAnalysis(\$datasetId: ID!, \$name: String!) {
      createAnalysis(datasetId: \$datasetId, name: \$name) {
        __typename
        id
        createdAt
        updatedAt
        name
        dataset {
          __typename
          id
        }
        dataview {
          __typename
          id
        }
      }
    }
  ''';
  @override
  Widget build(BuildContext context) {
    return Content(
      listQuery: query,
      listQueryVariables: {'projectId': projectId},
      toTitleString: (v) => v['name'],
      onTap: (v, current) => current.push(Analysis(v['id']), v['name']),
      createName: 'Analysis',
      makeCreateForm: (setFormState, _v) =>
          CreateAnalysisForm(setFormState, projectId),
      createMutation: mutation,
      createFieldsToVariables: (fields) {
        return {'datasetId': fields['datasetId'], 'name': fields['name']};
      },
      onCreate: (v, current, _r) => current.push(Analysis(v['id']), v['name']),
    );
  }
}

class CreateAnalysisForm extends StatefulWidget {
  CreateAnalysisForm(this.setFormState, this.projectId);
  final void Function(Map<String, dynamic>, bool Function()) setFormState;
  final String projectId;
  final String datasets = '''
    query Datasets(\$projectId: ID!) {
      datasets(projectId: \$projectId) {
        __typename
        id
        createdAt
        updatedAt
        name
        status
      }
    }
  ''';
  @override
  _CreateAnalysisFormState createState() => _CreateAnalysisFormState();
}

class _CreateAnalysisFormState extends State<CreateAnalysisForm> {
  final _formKey = GlobalKey<FormBuilderState>();
  @override
  Widget build(BuildContext context) {
    return Query(
        options: QueryOptions(
          fetchPolicy: FetchPolicy.cacheAndNetwork,
          documentNode: gql(widget.datasets),
          variables: {'projectId': widget.projectId},
        ),
        builder: (QueryResult result,
            {VoidCallback refetch, FetchMore fetchMore}) {
          return FormBuilder(
              key: _formKey,
              autovalidateMode: AutovalidateMode.disabled,
              skipDisabled: true,
              onChanged: () {
                _formKey.currentState.save();
                widget.setFormState(Map.from(_formKey.currentState.value),
                    _formKey.currentState.validate);
              },
              child: Column(children: [
                FormBuilderTextField(
                  name: 'name',
                  validator: FormBuilderValidators.required(context),
                  decoration:
                      InputDecoration(hintText: 'name', labelText: 'name'),
                ),
                FormBuilderDropdown<String>(
                  name: 'datasetId',
                  validator: FormBuilderValidators.required(context),
                  decoration: InputDecoration(
                      hintText: 'dataset', labelText: 'dataset'),
                  items: (result.loading ? [] : result.data['datasets'])
                      .map<DropdownMenuItem<String>>((v) => DropdownMenuItem(
                          value: v['id'].toString(),
                          child: Text(v['name'].toString())))
                      .toList(),
                ),
              ]));
        });
  }
}
