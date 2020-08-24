import 'package:flutter/material.dart';
import 'package:google_sign_in/google_sign_in.dart';
import 'storage.dart';
import 'error_dialog.dart';

enum Status { Uninitialized, Authenticated, Authenticating, Unauthenticated }

class Auth extends ChangeNotifier {
  Auth({@required this.storage});
  final StorageInterface storage;
  // https://developers.google.com/identity/protocols/oauth2/scopes
  final GoogleSignIn _googleSignIn = GoogleSignIn(scopes: [
    'email',
    'openid',
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

  signInWithGoogle(BuildContext context) async {
    try {
      _status = Status.Authenticating;
      notifyListeners();
      final GoogleSignInAccount googleUser = await _googleSignIn.signIn();
      final GoogleSignInAuthentication googleAuth =
          await googleUser.authentication;
      // TODO(danj): exchange access and ID token for JWT
      _token = googleAuth.idToken;
      await storage.putString(key: 'token', value: _token);
      _status = Status.Authenticated;
    } catch (e) {
      errorDialog(context: context, message: e.toString());
      _status = Status.Unauthenticated;
    }
    notifyListeners();
  }

  logout() async {
    _googleSignIn.signOut();
    await storage.clear();
    _token = null;
    _status = Status.Authenticating;
    notifyListeners();
  }
}
