import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../../common/profile.dart';
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
          // navigate back with back button
          onWillPop: () async {
            nav.back();
            return false;
          },
          child: Scaffold(
              appBar: AppBar(
                centerTitle: true,
                title: Text('motoko',
                    style: TextStyle(
                        color: Theme.of(context).colorScheme.secondary,
                        fontFamily: 'Brushstrike-TTF',
                        fontSize: 35.0)),
                elevation: 5.0,
              ),
              // refocus body when tapping outside inputs
              body: GestureDetector(
                  behavior: HitTestBehavior.opaque,
                  onTap: () {
                    FocusScope.of(context).requestFocus(new FocusNode());
                  },
                  child: nav.body),
              drawer: Consumer<Profile>(
                  builder: (context, profile, child) =>
                      ProfileDrawer(profile: profile)),
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
