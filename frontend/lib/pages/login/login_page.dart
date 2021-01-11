import '../../common/auth.dart';
import 'package:flutter/material.dart';
import 'package:flutter_signin_button/flutter_signin_button.dart';

class LoginPage extends StatelessWidget {
  final Auth auth;
  LoginPage({@required this.auth});
  @override
  Widget build(BuildContext context) {
    return Scaffold(
        backgroundColor: Colors.black,
        body: Center(
            child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: <Widget>[
              Text('motoko',
                  style: TextStyle(
                      color: Theme.of(context).colorScheme.secondary,
                      fontFamily: 'Brushstrike-TTF',
                      fontSize: 75.0)),
              SizedBox(height: 50),
              Transform.scale(
                  scale: 1.15,
                  child: SignInButton(Buttons.Apple,
                      onPressed: () => auth.signInWithApple(context))),
              SizedBox(height: 5),
              Transform.scale(
                  scale: 1.15,
                  child: SignInButton(Buttons.Google,
                      onPressed: () => auth.signInWithGoogle(context))),
            ])));
  }
}
