import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'common/accent_color.dart';
import 'common/auth.dart';
import 'common/gql.dart';
import 'common/profile.dart';
import 'common/storage.dart';
import 'pages/home/home_page.dart';
import 'pages/login/login_page.dart';
import 'pages/splash/splash_page.dart';

void main() {
  runApp(App());
}

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Provider(create: (_) {
      var storage = Storage();
      storage.init();
      return storage;
    }, child: Consumer<Storage>(builder: (context, storage, _) {
      return ChangeNotifierProvider(
          create: (_) {
            var accentColor = AccentColor(storage: storage);
            accentColor.init();
            return accentColor;
          },
          child: Consumer<AccentColor>(
              builder: (context, accentColor, _) => wrapWithGraphQL(MaterialApp(
                    title: 'motoko',
                    home: MultiProvider(providers: [
                      ChangeNotifierProvider<Auth>(create: (_) {
                        var auth = Auth(storage: storage);
                        auth.init();
                        return auth;
                      }),
                      ChangeNotifierProvider<Profile>(create: (_) => Profile()),
                    ], child: LandingPage()),
                    themeMode: ThemeMode.dark,
                    darkTheme: ThemeData(
                      appBarTheme: AppBarTheme(
                          iconTheme: IconThemeData(color: accentColor.value)),
                      iconTheme: IconThemeData(color: accentColor.value),
                      accentColor: accentColor.value,
                      inputDecorationTheme: InputDecorationTheme(
                        focusedBorder: UnderlineInputBorder(
                            borderSide: BorderSide(color: accentColor.value)),
                      ),
                      colorScheme:
                          ColorScheme.dark(secondary: accentColor.value),
                      visualDensity: VisualDensity.adaptivePlatformDensity,
                    ),
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
