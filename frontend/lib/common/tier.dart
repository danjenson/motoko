import 'package:flutter/material.dart';
import 'dart:io' show Platform;

class Tier extends ChangeNotifier {
  final _tier = Platform.environment['TIER'] ?? 'dev';
  final _endpoints = {
    'dev': 'https://dev.motoko.ai/graphql',
    'prod': 'https://motoko.ai/graphql',
  };
  String apiEndpoint() {
    // https://developer.android.com/studio/run/emulator-networking
    // NOTE: use the following for a local server
    // return _endpoints[_tier] ?? 'http://10.0.2.2:3000/graphql';
    return _endpoints[_tier];
  }
}
