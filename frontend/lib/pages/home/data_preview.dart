import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import '../../common/error_dialog.dart';

class DataPreview extends StatelessWidget {
  DataPreview({@required this.id});
  final String id;
  final String query = '''
  query DataNode(\$id: ID!) {
    node(id: \$id) {
      __typename
      id
      ... on Dataset {
        schema {
          columnName
          dataType
        }
        sampleRows
      }
      ... on Dataview {
        schema {
          columnName
          dataType
        }
        sampleRows
      }
    }
  }
  ''';
  List<dynamic> schema = [];
  List<dynamic> sampleRows = [];
  @override
  Widget build(BuildContext context) {
    return Query(
        options: QueryOptions(
            fetchPolicy: FetchPolicy.cacheFirst,
            documentNode: gql(query),
            variables: {"id": id}),
        builder: (QueryResult result,
            {VoidCallback refetch, FetchMore fetchMore}) {
          var loading =
              result.loading && result.source == QueryResultSource.Loading;
          if (result.hasException) {
            return ErrorDialog(result.exception.toString());
          } else if (!loading) {
            schema = result.data["node"]["schema"];
            sampleRows = result.data["node"]["sampleRows"];
          }
          return loading
              ? Expanded(child: Center(child: CircularProgressIndicator()))
              : Expanded(
                  child: Scrollbar(
                      child: SingleChildScrollView(
                          scrollDirection: Axis.vertical,
                          child: SingleChildScrollView(
                            scrollDirection: Axis.horizontal,
                            child: DataTable(
                              columns: schema
                                  .map<DataColumn>((columnDataType) =>
                                      DataColumn(
                                          label: Text(
                                              columnDataType["columnName"]
                                                  .toString(),
                                              style: TextStyle(
                                                  fontStyle:
                                                      FontStyle.italic))))
                                  .toList(),
                              rows: sampleRows
                                  .map((row) => DataRow(
                                      cells: schema
                                          .map<DataCell>((columnDataType) =>
                                              DataCell(Text(row[columnDataType[
                                                          "columnName"]
                                                      .toString()]
                                                  .toString())))
                                          .toList()))
                                  .toList(),
                            ),
                          ))));
        });
  }
}
