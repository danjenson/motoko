import 'package:flutter/material.dart';
import 'package:flutter/gestures.dart';
import 'package:provider/provider.dart';
import 'nav.dart';
import 'nav_breadcrumb.dart';
import 'profile_drawer.dart';
import 'projects.dart' as p;

class HomePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(create: (_) {
      var nav = Nav();
      nav.push('projects', p.Projects(nav: nav), p.add);
      return nav;
    }, child: Consumer<Nav>(builder: (context, nav, _) {
      return WillPopScope(
          // override default back button action to use nav
          onWillPop: () async {
            nav.back();
            return false;
          },
          child: Scaffold(
              appBar: AppBar(
                centerTitle: true,
                title: RichText(
                    text: TextSpan(
                        text: 'motoko',
                        recognizer: TapGestureRecognizer()
                          ..onTap = () {
                            nav.to(0);
                          },
                        style: TextStyle(
                            color: Theme.of(context).colorScheme.secondary,
                            fontFamily: 'Brushstrike-TTF',
                            fontSize: 35.0))),
                elevation: 5.0,
              ),
              // refocus body when tapping outside inputs
              body: GestureDetector(
                  behavior: HitTestBehavior.opaque,
                  onTap: () {
                    FocusScope.of(context).requestFocus(new FocusNode());
                  },
                  child: nav.body),
              drawer: ProfileDrawer(),
              bottomNavigationBar: NavBreadcrumb(nav: nav),
              floatingActionButton: nav.add != null
                  ? FloatingActionButton(
                      onPressed: () => nav.add(context),
                      tooltip: 'add',
                      child: Icon(Icons.add),
                    )
                  : SizedBox.shrink()));
    }));
  }
}
