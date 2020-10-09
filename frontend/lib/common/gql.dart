import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../common/auth.dart';
import '../common/tier.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

class GraphQL extends StatelessWidget {
  GraphQL(this.child);
  final Widget child;
  @override
  Widget build(BuildContext context) {
    var auth = Provider.of<Auth>(context);
    var authLink = AuthLink(getToken: () async {
      if (auth.refreshTokenHasExpired()) {
        await auth.logout();
        return "";
      }
      if (auth.accessTokenHasExpired()) {
        await auth.refreshTokens(context);
      }
      var accessToken = auth.accessToken;
      return "Bearer $accessToken";
    });
    var errorLink = ErrorLink(errorHandler: (ErrorResponse res) {
      // log out if you get an unauthorized response code
      if (res.fetchResult.statusCode == 401) {
        auth.logout();
      }
    });
    var apiEndpoint = Provider.of<Tier>(context).apiEndpoint();
    var httpLink = HttpLink(uri: apiEndpoint);
    var link = authLink.concat(errorLink).concat(httpLink);
    return GraphQLProvider(
        client: ValueNotifier(GraphQLClient(
            cache: NormalizedInMemoryCache(
                // requires `id` and `__typename` fields or returns null
                dataIdFromObject: typenameDataIdFromObject),
            link: link)),
        child: CacheProvider(child: child));
  }
}
