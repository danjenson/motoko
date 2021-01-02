import '../../../common/utils.dart';
import '../components/content.dart';
import '../components/data_preview.dart';
import 'package:flutter/material.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';

class Datasets extends StatelessWidget {
  Datasets(this.projectId);
  final String projectId;
  final String name = 'Datasets';
  final query = '''
    query Datasets(\$projectId: ID!) {
      datasets(projectId: \$projectId) {
        __typename
        id
        createdAt
        updatedAt
        name
        status
        nRows
      }
    }
  ''';
  final mutation = '''
    mutation CreateDataset(\$projectId: ID!, \$name: String!, \$uri: String!) {
      createDataset(projectId: \$projectId, name: \$name, uri: \$uri) {
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
  Widget build(BuildContext context) {
    return Content(
      listQuery: query,
      listQueryVariables: {'projectId': projectId},
      toTitleString: (v) => v['name'],
      toSecondarySubtitleString: (v) {
        if (v['nRows'] == null) {
          return '';
        }
        return 'rows: ${nCompact(v['nRows'])}';
      },
      onTapPreview: (id) => DataPreview(id),
      defaultPreviewString: 'Tap a dataset.',
      createName: 'Dataset',
      makeCreateForm: (setFormState, _v) =>
          CreateDatasetForm(setFormState, projectId),
      createMutation: mutation,
      createFieldsToVariables: (fields) {
        fields['projectId'] = projectId;
        return fields;
      },
      onCreate: (_v, _c, refetch) => refetch(),
    );
  }
}

class CreateDatasetForm extends StatefulWidget {
  CreateDatasetForm(this.setFormState, this.projectId);
  final void Function(Map<String, dynamic>, bool Function()) setFormState;
  final String projectId;
  @override
  _CreateDatasetFormState createState() => _CreateDatasetFormState();
}

class _CreateDatasetFormState extends State<CreateDatasetForm> {
  final _formKey = GlobalKey<FormBuilderState>();
  @override
  Widget build(BuildContext context) {
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
          decoration: InputDecoration(hintText: 'name', labelText: 'name'),
        ),
        FormBuilderTextField(
          name: 'uri',
          validator: FormBuilderValidators.required(context),
          decoration: InputDecoration(hintText: 'uri', labelText: 'uri'),
        )
      ]),
    );
  }
}
