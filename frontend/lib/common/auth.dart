import 'package:flutter/material.dart';
import 'package:google_sign_in/google_sign_in.dart';
import 'storage.dart';

enum Status { Uninitialized, Authenticated, Authenticating, Unauthenticated }

class Auth extends ChangeNotifier {
  Auth({@required this.storage});
  final StorageInterface storage;
  final GoogleSignIn _googleSignIn = GoogleSignIn(scopes: [
    'email',
    'https://www.googleapis.com/auth/contacts.readonly',
  ]);
  Status _status = Status.Uninitialized;
  String _token;

  Status get status => _status;
  String get token => _token;

  void init() async {
    _token = await storage.getString('token');
    _status = _token == null ? Status.Authenticating : Status.Authenticated;
    notifyListeners();
  }

  signInWithGoogle() async {
    try {
      _status = Status.Authenticating;
      notifyListeners();
      final GoogleSignInAccount googleUser = await _googleSignIn.signIn();
      final GoogleSignInAuthentication googleAuth =
          await googleUser.authentication;
      // TODO(danj): exchange access/id tokens for JWT at server
      _token = googleAuth.idToken;
      await storage.putString(key: 'token', value: _token);
      _status = Status.Authenticated;
    } catch (e) {
      // TODO(danj): show error dialogue
      _status = Status.Unauthenticated;
      await storage.putString(key: 'token', value: 'dummy-token');
      _status = Status.Authenticated;
    }
    notifyListeners();
  }

  logout() async {
    await storage.clear();
    _token = null;
    _status = Status.Authenticating;
    notifyListeners();
  }
}
