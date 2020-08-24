import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'common/accent_color.dart';
import 'common/auth.dart';
import 'common/gql.dart';
import 'common/profile.dart';
import 'common/storage.dart';
import 'common/theme.dart';
import 'pages/home/home_page.dart';
import 'pages/login/login_page.dart';
import 'pages/privacy/privacy_page.dart';
import 'pages/splash/splash_page.dart';

void main() {
  runApp(App());
}

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    // provide storage
    return Provider(create: (_) {
      var storage = Storage();
      storage.init();
      return storage;
    }, child: Consumer<Storage>(builder: (context, storage, _) {
      // provide accent color
      return ChangeNotifierProvider(
          create: (_) {
            var accentColor = AccentColor(storage: storage);
            accentColor.init();
            return accentColor;
          },
          child: Consumer<AccentColor>(
              builder: (context, accentColor, _) => wrapWithGraphQL(MaterialApp(
                    title: 'motoko',
                    initialRoute: '/',
                    routes: {
                      '/': (context) => MultiProvider(
                            providers: [
                              // provide auth
                              ChangeNotifierProvider<Auth>(create: (_) {
                                var auth = Auth(storage: storage);
                                auth.init();
                                return auth;
                              }),
                              // provide profile
                              ChangeNotifierProvider<Profile>(
                                  create: (_) => Profile()),
                            ],
                            child: LandingPage(),
                          ),
                      '/privacy': (context) => PrivacyPage(),
                    },
                    themeMode: ThemeMode.dark,
                    darkTheme: theme(accentColor.value),
                  ))));
    }));
  }
}

class LandingPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Consumer<Auth>(builder: (context, Auth auth, _) {
      switch (auth.status) {
        case Status.Uninitialized:
          return SplashPage();
        case Status.Authenticated:
          return HomePage();
        default:
          return LoginPage(auth: auth);
      }
    });
  }
}
