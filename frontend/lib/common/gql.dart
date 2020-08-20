import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

Widget wrapWithGraphQL(Widget app) {
  return GraphQLProvider(
      client: ValueNotifier(GraphQLClient(
          cache: NormalizedInMemoryCache(
              dataIdFromObject: typenameDataIdFromObject),
          link: AuthLink(
                  getToken: () async =>
                      'bearer 22d2891cb1566d7c21f897ebd7d927dd99ac4c9a')
              .concat(HttpLink(uri: 'https://api.github.com/graphql')))),
      child: CacheProvider(child: app));
}
