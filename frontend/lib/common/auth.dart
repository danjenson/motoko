import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:google_sign_in/google_sign_in.dart';
import 'storage.dart';
import 'tier.dart';
import 'error_dialog.dart';
import 'utils.dart';

enum Status { Uninitialized, Authenticated, Authenticating, Unauthenticated }

class Auth extends ChangeNotifier {
  final Tier tier;
  final Storage storage;
  Auth(this.tier, this.storage);
  // https://developers.google.com/identity/protocols/oauth2/scopes
  final GoogleSignIn _googleSignIn = GoogleSignIn(scopes: [
    'email',
    'openid',
    'profile',
  ]);
  final _login = '''
    mutation Login(\$provider: Provider!, \$token: String!) {
      login(provider: \$provider, token: \$token) {
        accessToken
        accessTokenExpiresAt
        refreshToken
        refreshTokenExpiresAt
      }
    }
  ''';
  final _refresh = '''
    mutation Refresh(\$token: String!) {
      refresh(token: \$token) {
        accessToken
        accessTokenExpiresAt
        refreshToken
        refreshTokenExpiresAt
      }
    }
  ''';
  Status _status = Status.Uninitialized;
  String _accessToken;
  String _refreshToken;
  int _accessTokenExpiresAt;
  int _refreshTokenExpiresAt;

  String get accessToken => _accessToken;
  String get refreshToken => _refreshToken;
  Status get status => _status;

  void init() async {
    _accessToken = await storage.getString('accessToken');
    _accessTokenExpiresAt =
        int.tryParse((await storage.getString('accessTokenExpiresAt') ?? ''));
    _refreshToken = await storage.getString('refreshToken');
    _refreshTokenExpiresAt =
        int.tryParse((await storage.getString('refreshTokenExpiresAt') ?? ''));
    if (_accessToken == null ||
        _accessTokenExpiresAt == null ||
        _refreshToken == null ||
        _refreshTokenExpiresAt == null ||
        _refreshTokenExpiresAt < timestamp()) {
      _status = Status.Authenticating;
      clearCredentials();
      notifyListeners();
      return;
    }
    _status = Status.Authenticated;
    notifyListeners();
  }

  bool accessTokenHasExpired() {
    return _accessTokenExpiresAt < timestamp();
  }

  bool refreshTokenHasExpired() {
    return _refreshTokenExpiresAt < timestamp();
  }

  refreshTokens(BuildContext context) async {
    final queryOpts = QueryOptions(
        fetchPolicy: FetchPolicy.networkOnly,
        documentNode: gql(_refresh),
        variables: {'token': _refreshToken});
    try {
      final res = await client().query(queryOpts);
      final creds = res.data['refresh'];
      _accessToken = creds['accessToken'];
      _accessTokenExpiresAt = creds['accessTokenExpiresAt'];
      _refreshToken = creds['refreshToken'];
      _refreshTokenExpiresAt = creds['refreshToken'];
    } catch (e) {
      errorDialog(context: context, message: e.toString());
      _status = Status.Unauthenticated;
      notifyListeners();
    }
  }

  GraphQLClient client() {
    return GraphQLClient(
        cache:
            NormalizedInMemoryCache(dataIdFromObject: typenameDataIdFromObject),
        link: HttpLink(uri: tier.apiEndpoint()));
  }

  void clearCredentials() async {
    await storage.delete('accessToken');
    await storage.delete('accessTokenExpiresAt');
    await storage.delete('refreshToken');
    await storage.delete('refreshTokenExpiresAt');
  }

  signInWithGoogle(BuildContext context) async {
    try {
      _status = Status.Authenticating;
      notifyListeners();
      final GoogleSignInAccount googleUser = await _googleSignIn.signIn();
      final GoogleSignInAuthentication googleAuth =
          await googleUser.authentication;
      final queryOpts = QueryOptions(
          fetchPolicy: FetchPolicy.networkOnly,
          documentNode: gql(_login),
          variables: {'provider': 'GOOGLE', 'token': googleAuth.idToken});
      final res = await client().query(queryOpts);
      if (!res.loading) {
        final creds = res.data['login'];
        _accessToken = creds['accessToken'];
        _refreshToken = creds['refreshToken'];
        _accessTokenExpiresAt = DateTime.parse(creds['accessTokenExpiresAt'])
                .millisecondsSinceEpoch ~/
            1000;
        _refreshTokenExpiresAt = DateTime.parse(creds['refreshTokenExpiresAt'])
                .millisecondsSinceEpoch ~/
            1000;
        await storage.putString(key: 'accessToken', value: _accessToken);
        await storage.putString(
            key: 'accessTokenExpiresAt',
            value: _accessTokenExpiresAt.toString());
        await storage.putString(key: 'refreshToken', value: _refreshToken);
        await storage.putString(
            key: 'refreshTokenExpiresAt',
            value: _refreshTokenExpiresAt.toString());
        _status = Status.Authenticated;
        notifyListeners();
      }
    } catch (e) {
      errorDialog(context: context, message: e.toString());
      _status = Status.Unauthenticated;
      notifyListeners();
    }
  }

  logout() async {
    clearCredentials();
    _googleSignIn.signOut();
    _status = Status.Authenticating;
    notifyListeners();
  }
}
