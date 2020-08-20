import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'utils.dart';

class Storage implements StorageInterface {
  final _storage = isWeb() ? WebStorage() : MobileStorage();

  Future<bool> init() => _storage.init();
  Future<bool> hasKey(String key) => _storage.hasKey(key);
  Future<bool> putString({String key, String value}) =>
      _storage.putString(key: key, value: value);
  Future<String> getString(String key) => _storage.getString(key);
  Future<bool> putBool({String key, bool value}) =>
      _storage.putBool(key: key, value: value);
  Future<bool> getBool(String key) => _storage.getBool(key);
  Future<bool> clear() => _storage.clear();
}

abstract class StorageInterface {
  Future<bool> init();
  Future<bool> hasKey(String key);
  Future<bool> putString({String key, String value});
  Future<String> getString(String key);
  Future<bool> putBool({String key, bool value});
  Future<bool> getBool(String key);
  Future<void> clear();
}

// TODO(danj): find something more secure for web
class WebStorage implements StorageInterface {
  SharedPreferences _storage;

  Future<bool> init() async {
    _storage = await SharedPreferences.getInstance();
    return true;
  }

  Future<bool> hasKey(String key) async => _storage.containsKey(key);

  Future<bool> putString({String key, String value}) async =>
      _storage.setString(key, value);

  Future<String> getString(String key) async {
    try {
      return _storage.getString(key);
    } catch (_) {
      return '';
    }
  }

  Future<bool> putBool({String key, bool value}) async =>
      _storage.setBool(key, value);

  Future<bool> getBool(String key) async {
    try {
      return _storage.getBool(key);
    } catch (_) {
      return false;
    }
  }

  Future<bool> clear() async => _storage.clear();
}

class MobileStorage implements StorageInterface {
  final FlutterSecureStorage _storage = FlutterSecureStorage();

  Future<bool> init() async => true;

  Future<bool> hasKey(String key) async {
    var v = await _storage.read(key: key);
    return v != null;
  }

  Future<bool> putString({String key, String value}) async {
    await _storage.write(key: key, value: value);
    return true;
  }

  Future<String> getString(String key) async => _storage.read(key: key) ?? '';

  Future<bool> putBool({String key, bool value}) async {
    await _storage.write(key: key, value: value.toString());
    return true;
  }

  Future<bool> getBool(String key) async {
    var v = await _storage.read(key: key);
    return v == 'true';
  }

  Future<bool> clear() async {
    await _storage.deleteAll();
    return true;
  }
}
