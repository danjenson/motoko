import '../../../common/accent_color.dart';
import '../../../common/auth.dart';
import '../../../common/dialogs.dart';
import 'dart:math' as math;
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:provider/provider.dart';
import 'package:url_launcher/url_launcher.dart';

class ProfileDrawer extends StatelessWidget {
  final double size = 25.0;
  final TextStyle textStyle = TextStyle(fontSize: 20.0);
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
          fetchPolicy: FetchPolicy.cacheFirst,
          documentNode: gql(query),
        ),
        builder: (QueryResult result,
            {VoidCallback refetch, FetchMore fetchMore}) {
          final color = Theme.of(context).colorScheme.secondary;
          return GestureDetector(
              onTap: () => Navigator.pop(context),
              child: Drawer(
                  elevation: 0,
                  child: Container(
                    margin: EdgeInsets.only(bottom: 40, top: 50, right: 15),
                    decoration: BoxDecoration(
                        borderRadius: BorderRadius.circular(10),
                        border: Border.all(color: color)),
                    child: Column(
                      children: <Widget>[
                        DrawerHeader(
                            decoration: BoxDecoration(
                                border: Border(bottom: BorderSide.none)),
                            child: Center(
                                child: Column(children: <Widget>[
                              Transform(
                                  alignment: Alignment.center,
                                  transform: Matrix4.rotationY(math.pi),
                                  child: Icon(Icons.sports_motorsports_sharp,
                                      size: 75.0, color: color)),
                              SizedBox(height: 5.0),
                              Text(
                                  result.data != null &&
                                          result.data['me'] != null
                                      ? result.data['me']['displayName']
                                      : "",
                                  style:
                                      TextStyle(fontSize: 20.0, color: color)),
                              SizedBox(height: 5.0),
                              Text(
                                  result.data != null &&
                                          result.data['me'] != null
                                      ? '@' + result.data['me']['name']
                                      : "",
                                  style:
                                      TextStyle(fontSize: 15.0, color: color)),
                            ]))),
                        Spacer(),
                        ListTile(
                            onTap: () async {
                              const url =
                                  'https://github.com/danjenson/motoko-issues/issues';
                              if (await canLaunch(url)) {
                                await launch(url);
                              } else {
                                throw 'Could not launch $url';
                              }
                            },
                            leading:
                                Icon(Icons.bug_report_outlined, size: size),
                            title: Text('Bug Reports & Feature Requests',
                                style: textStyle)),
                        GestureDetector(
                            child: ListTile(
                                onTap: () {
                                  Clipboard.setData(
                                      ClipboardData(text: auth.accessToken));
                                  var close = showProgressDialog(
                                      'Copied to clipboard', false);
                                  Future.delayed(
                                      Duration(milliseconds: 1000), close);
                                },
                                leading: Transform(
                                  alignment: Alignment.center,
                                  transform: Matrix4.rotationY(math.pi),
                                  child: Icon(Icons.content_copy, size: size),
                                ),
                                title: Text('Access Key', style: textStyle))),
                        SwitchListTile(
                          activeColor: accentColor.second,
                          inactiveTrackColor:
                              accentColor.first.withOpacity(0.5),
                          inactiveThumbColor: accentColor.first,
                          value: !accentColor.isFirst,
                          title: Text('Accent', style: textStyle),
                          onChanged: (value) => accentColor.flip(),
                          secondary: Icon(Icons.brightness_4, size: size),
                        ),
                        ListTile(
                            onTap: () => auth.logout(),
                            leading: Transform(
                              alignment: Alignment.center,
                              transform: Matrix4.rotationY(math.pi),
                              child: Icon(Icons.exit_to_app, size: size),
                            ),
                            title: Text('Logout', style: textStyle)),
                        SizedBox(height: 15.0),
                      ],
                    ),
                  )));
        });
  }
}
