import '../components/content.dart';
import '../content/project.dart';
import 'package:flutter/material.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';

class Projects extends StatelessWidget {
  final String query = '''
    query {
      projects {
        __typename
        id
        createdAt
        updatedAt
        name
      }
    }
  ''';
  final String mutation = '''
    mutation CreateProject(\$name: String!) {
      createProject(name: \$name) {
        __typename
        id
        createdAt
        updatedAt
        name
      }
    }
  ''';
  @override
  Widget build(BuildContext context) {
    return Content(
        listQuery: query,
        listQueryVariables: {},
        toTitleString: (v) => v['name'],
        onTap: (v, current) => current.push(
            Project(v['id']), v['name'].toString(), hasCreateButton: false),
        createName: 'Project',
        makeCreateForm: (setFormState, _v) => CreateProjectForm(setFormState),
        createMutation: mutation,
        createFieldsToVariables: (fields) => fields,
        onCreate: (v, current, _r) => current.push(
            Project(v['id']), v['name'].toString(),
            hasCreateButton: false));
  }
}

class CreateProjectForm extends StatefulWidget {
  CreateProjectForm(this.setFormState);
  final void Function(Map<String, dynamic>, bool Function()) setFormState;
  @override
  _CreateProjectFormState createState() => _CreateProjectFormState();
}

class _CreateProjectFormState extends State<CreateProjectForm> {
  final _formKey = GlobalKey<FormBuilderState>();
  @override
  Widget build(BuildContext context) {
    return FormBuilder(
      key: _formKey,
      autovalidateMode: AutovalidateMode.disabled,
      skipDisabled: true,
      onChanged: () {
        _formKey.currentState.save();
        widget.setFormState(
            Map<String, dynamic>.from(_formKey.currentState.value),
            _formKey.currentState.validate);
      },
      child: FormBuilderTextField(
        name: 'name',
        validator: FormBuilderValidators.required(context),
        decoration: InputDecoration(hintText: 'name', labelText: 'name'),
      ),
    );
  }
}
