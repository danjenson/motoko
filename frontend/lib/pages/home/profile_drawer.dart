import '../../common/accent_color.dart';
import '../../common/auth.dart';
import 'dart:math' as math;
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:provider/provider.dart';

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
    var accentColor = Provider.of<AccentColor>(context, listen: false);
    var auth = Provider.of<Auth>(context, listen: false);
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
                  Transform(
                      alignment: Alignment.center,
                      transform: Matrix4.rotationY(math.pi),
                      child: Icon(Icons.sports_motorsports_sharp,
                          size: 100.0,
                          color: Theme.of(context).colorScheme.secondary)),
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
                GestureDetector(
                    child: ListTile(
                        onTap: () {
                          Clipboard.setData(
                              ClipboardData(text: auth.accessToken));
                        },
                        leading: Transform(
                          alignment: Alignment.center,
                          transform: Matrix4.rotationY(math.pi),
                          child: Icon(Icons.content_copy, size: size),
                        ),
                        title: Text('Access Key', style: textStyle))),
                SwitchListTile(
                  activeColor: accentColor.second,
                  inactiveTrackColor: accentColor.first.withOpacity(0.5),
                  inactiveThumbColor: accentColor.first,
                  value: !accentColor.isFirst,
                  title: Text('Accent', style: textStyle),
                  onChanged: (value) => accentColor.flip(),
                  secondary: Icon(Icons.brightness_4),
                ),
                ListTile(
                    onTap: () => auth.logout(),
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
