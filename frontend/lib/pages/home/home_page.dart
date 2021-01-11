import 'components/content_breadcrumb.dart';
import 'components/current.dart';
import 'components/globals.dart' as globals;
import 'components/profile_drawer.dart';
import 'content/projects.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class HomePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final color = Theme.of(context).colorScheme.secondary;
    return MultiProvider(
        providers: [
          ChangeNotifierProvider<Current>(
              create: (_) => Current(Projects(), 'Projects')),
        ],
        child: Consumer<Current>(builder: (context, current, _) {
          return WillPopScope(
            // override default back button action to use nav
            onWillPop: () async {
              current.back();
              return false;
            },
            child: Scaffold(
                key: globals.homeKey,
                appBar: AppBar(
                  leading: Visibility(
                      visible: current.canGoBack,
                      child: IconButton(
                          icon: Icon(Icons.arrow_back_ios),
                          color: color,
                          onPressed: () => current.back())),
                  centerTitle: true,
                  title: RichText(
                      text: TextSpan(
                          text: 'motoko',
                          recognizer: TapGestureRecognizer()
                            ..onTap = () => current.to(0),
                          style: TextStyle(
                              color: color,
                              fontFamily: 'Brushstrike-TTF',
                              fontSize: 35.0))),
                ),
                // refocus body when tapping outside inputs
                body: GestureDetector(
                    behavior: HitTestBehavior.opaque,
                    onTap: () {
                      FocusScope.of(context).requestFocus(new FocusNode());
                    },
                    child: current.content),
                endDrawer: ProfileDrawer(),
                bottomNavigationBar: ContentBreadcrumb(),
                floatingActionButton: Visibility(
                    visible: current.hasCreateButton,
                    child: FloatingActionButton(
                        onPressed: () => globals.create(),
                        tooltip: 'create',
                        child: Icon(Icons.add)))),
          );
        }));
  }
}
