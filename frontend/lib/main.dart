import 'common/accent_color.dart';
import 'common/auth.dart';
import 'common/gql.dart';
import 'common/storage.dart';
import 'common/theme.dart';
import 'common/tier.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import 'pages/home/home_page.dart';
import 'pages/login/login_page.dart';
import 'pages/privacy/privacy_page.dart';
import 'pages/splash/splash_page.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await SystemChrome.setPreferredOrientations([DeviceOrientation.portraitUp]);
  runApp(App());
}

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MultiProvider(
        providers: [
          ChangeNotifierProvider<Tier>(create: (_) => Tier()),
          Provider<Storage>(create: (_) {
            var storage = Storage();
            storage.init();
            return storage;
          }),
        ],
        child: Consumer<Tier>(builder: (context, tier, _) {
          return Consumer<Storage>(builder: (context, storage, _) {
            return MultiProvider(
                providers: [
                  ChangeNotifierProvider<Auth>(create: (_) {
                    var auth = Auth(tier, storage);
                    auth.init();
                    return auth;
                  }),
                  ChangeNotifierProvider<AccentColor>(create: (_) {
                    var accentColor = AccentColor(storage);
                    accentColor.init();
                    return accentColor;
                  }),
                ],
                child: CustomGraphQLProvider(
                    Consumer<AccentColor>(builder: (context, accentColor, _) {
                  return MaterialApp(
                    title: 'motoko',
                    initialRoute: '/',
                    routes: {
                      '/': (context) => CurrentPage(),
                      '/privacy': (context) => PrivacyPage(),
                    },
                    themeMode: ThemeMode.dark,
                    darkTheme: makeTheme(accentColor.value),
                  );
                })));
          });
        }));
  }
}

class CurrentPage extends StatelessWidget {
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
