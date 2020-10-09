import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import '../../common/auth.dart';
import '../../common/accent_color.dart';
import 'dart:math' as math;

class ProfileDrawer extends StatelessWidget {
  final double size = 25.0;
  final TextStyle textStyle = TextStyle(fontSize: 25.0);
  final query = '''
    query {
      me {
        __typename
        id
        name
        displayName
      }
    }
  ''';
  @override
  Widget build(BuildContext context) {
    // TODO(danj): loading screen
    return Query(
        options: QueryOptions(
          fetchPolicy: FetchPolicy.cacheAndNetwork,
          documentNode: gql(query),
        ),
        builder: (QueryResult result,
            {VoidCallback refetch, FetchMore fetchMore}) {
          return Drawer(
            child: Column(
              children: <Widget>[
                DrawerHeader(
                    child: Center(
                        child: Column(children: <Widget>[
                  Icon(Icons.account_circle,
                      size: 100.0,
                      color: Theme.of(context).colorScheme.secondary),
                  SizedBox(height: 10.0),
                  Text(
                      result.data != null && result.data['me'] != null
                          ? result.data['me']['name']
                          : "",
                      style: TextStyle(
                          fontSize: 20.0,
                          color: Theme.of(context).colorScheme.secondary)),
                ]))),
                Spacer(),
                SwitchListTile(
                  activeColor:
                      Provider.of<AccentColor>(context, listen: false).second,
                  inactiveTrackColor:
                      Provider.of<AccentColor>(context, listen: false)
                          .first
                          .withOpacity(0.5),
                  inactiveThumbColor:
                      Provider.of<AccentColor>(context, listen: false).first,
                  value:
                      !Provider.of<AccentColor>(context, listen: false).isFirst,
                  title: Text('Accent', style: textStyle),
                  onChanged: (value) =>
                      Provider.of<AccentColor>(context, listen: false).flip(),
                  secondary: Icon(Icons.brightness_4),
                ),
                ListTile(
                    onTap: () =>
                        Provider.of<Auth>(context, listen: false).logout(),
                    leading: Transform(
                      alignment: Alignment.center,
                      transform: Matrix4.rotationY(math.pi),
                      child: Icon(Icons.exit_to_app, size: size),
                    ),
                    title: Text('Logout', style: textStyle)),
                SizedBox(height: 10.0),
              ],
            ),
          );
        });
  }
}
