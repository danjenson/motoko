import 'package:flutter/material.dart';
import 'dart:io' show Platform;

class Tier extends ChangeNotifier {
  final _tier = Platform.environment['TIER'] ?? 'local';
  final _endpoints = {
    'dev': 'https://dev.motoko.ai/graphql',
    'prod': 'https://motoko.ai/graphql',
  };
  String apiEndpoint() {
    // https://developer.android.com/studio/run/emulator-networking
    return _endpoints[_tier] ?? 'http://10.0.2.2:3000/graphql';
  }
}
