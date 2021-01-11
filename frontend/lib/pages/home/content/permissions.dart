import '../../../common/utils.dart';
import '../components/content.dart';
import 'forms/create_role.dart';
import 'package:flutter/material.dart';

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
          'projectId': projectId,
          'userId': args['user'],
          'role': args['role'].toUpperCase(),
        };
      },
      onCreate: (_v, _c, refetch) => refetch(),
    );
  }
}
