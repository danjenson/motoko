import 'package:flutter/material.dart';

class SplashPage extends StatelessWidget {
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
            ])));
  }
}
