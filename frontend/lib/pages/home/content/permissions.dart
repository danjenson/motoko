import '../../../common/dialogs.dart';
import '../../../common/utils.dart';
import '../components/content.dart';
import 'package:flutter/material.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

class Permissions extends StatelessWidget {
  Permissions(this.projectId);
  final String projectId;
  final String name = 'Permissions';
  final query = '''
      query Roles(\$projectId: ID!) {
        roles(projectId: \$projectId) {
          __typename
          id
          role
          user {
            __typename
            id
            name
            displayName
          }
        }
      }
  ''';
  final String mutation = '''
    mutation CreateRole(
      \$projectId: ID!,
      \$userId: ID!,
      \$role: Role!,
    ) {
      createRole(
        projectId: \$projectId,
        userId: \$userId,
        role: \$role,
      ) {
        __typename
        id
        role
        user {
          __typename
          id
          name
          displayName
        }
      }
    }
  ''';
  @override
  Widget build(BuildContext context) {
    return Content(
      listQuery: query,
      listQueryVariables: {'projectId': projectId},
      toTitleString: (v) => v['role'].toString().toLowerCase().capitalize(),
      toSecondarySubtitleString: (v) =>
          v['user']['displayName'] + '\n@' + v['user']['name'],
      canDelete: (v, results) {
        final admins = results.where((v) => v['role'] == 'ADMIN');
        if (admins.length == 1 && v['role'] == 'ADMIN') {
          return false;
        }
        return true;
      },
      createName: 'Role',
      makeCreateForm: (setFormState, values) =>
          CreateRoleForm(setFormState, projectId, values),
      createMutation: mutation,
      createFieldsToVariables: (fields) {
        var args = Map.from(fields);
        return {
          'name': args['name'],
          'role': args['role'].toUpperCase(),
        };
      },
      onCreate: (_v, _c, refetch) => refetch(),
    );
  }
}

class CreateRoleForm extends StatefulWidget {
  CreateRoleForm(this.setFormState, this.projectId, this.values);
  final void Function(Map<String, dynamic>, bool Function()) setFormState;
  final String projectId;
  final List<dynamic> values;
  final String users = '''
    query Users() {
      users() {
        __typename
        id
        name
        displayName
      }
    }
  ''';
  @override
  _CreateRoleFormState createState() => _CreateRoleFormState();
}

class _CreateRoleFormState extends State<CreateRoleForm> {
  final _formKey = GlobalKey<FormBuilderState>();
  @override
  Widget build(BuildContext context) {
    return Query(
        options: QueryOptions(
          fetchPolicy: FetchPolicy.cacheAndNetwork,
          documentNode: gql(widget.users),
        ),
        builder: (QueryResult result,
            {VoidCallback refetch, FetchMore fetchMore}) {
          final usersWithRoles = widget.values
              .map<String>((v) => v['user']['name'].toString())
              .toList();
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
                FormBuilderDropdown<String>(
                  name: 'user',
                  validator: FormBuilderValidators.required(context),
                  decoration:
                      InputDecoration(hintText: 'user', labelText: 'user'),
                  items: (result.loading ? [] : result.data['users'])
                      .where(
                          (v) => !usersWithRoles.contains(v['name'].toString()))
                      .map<DropdownMenuItem<String>>((v) => DropdownMenuItem(
                          value: v['name'].toString(),
                          child: Text(v['name'].toString())))
                      .toList(),
                ),
                FormBuilderDropdown<String>(
                  name: 'role',
                  validator: FormBuilderValidators.required(context),
                  decoration:
                      InputDecoration(hintText: 'role', labelText: 'role'),
                  items: ['Admin', 'Editor', 'Viewer']
                      .map((v) => DropdownMenuItem(value: v, child: Text(v)))
                      .toList(),
                ),
              ]));
        });
  }
}
