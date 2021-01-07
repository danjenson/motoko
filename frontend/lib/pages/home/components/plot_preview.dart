import '../../../common/dialogs.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_svg/svg.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:photo_view/photo_view.dart';

class PlotPreview extends StatelessWidget {
  PlotPreview(this.id);
  final String id;
  final plotUri = '''
    query Node(\$id: ID!) {
      node(id: \$id) {
        __typename
        id
        ... on Plot {
          uri
        }
      }
    }
  ''';
  @override
  Widget build(BuildContext context) {
    return Query(
        options: QueryOptions(
            fetchPolicy: FetchPolicy.cacheFirst,
            documentNode: gql(plotUri),
            variables: {'id': id}),
        builder: (QueryResult result,
            {VoidCallback refetch, FetchMore fetchMore}) {
          var uri;
          if (result.hasException) {
            showErrorDialog(context, result.exception.toString());
          } else if (!result.loading) {
            uri = result.data['node']['uri'];
          }
          return result.loading
              ? Center(child: CircularProgressIndicator())
              : GestureDetector(
                  onTap: () {
                    Clipboard.setData(ClipboardData(text: uri));
                    var close =
                        showProgressDialog('Copied to clipboard', false);
                    Future.delayed(Duration(milliseconds: 1000), close);
                  },
                  child: ClipRect(
                      child: PhotoView.customChild(
                          backgroundDecoration:
                              BoxDecoration(color: Colors.transparent),
                          child: SvgPicture.network(uri))));
        });
  }
}
