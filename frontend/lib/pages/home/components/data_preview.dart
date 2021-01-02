import '../../../common/dialogs.dart';
import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

class DataPreview extends StatelessWidget {
  DataPreview(this.id);
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
            variables: {'id': id}),
        builder: (QueryResult result,
            {VoidCallback refetch, FetchMore fetchMore}) {
          if (result.hasException) {
            showErrorDialog(context, result.exception.toString());
          } else if (!result.loading) {
            schema = result.data['node']['schema'] ?? [];
            sampleRows = result.data['node']['sampleRows'];
          }
          return result.loading
              ? Center(child: CircularProgressIndicator())
              : Scrollbar(
                  child: SingleChildScrollView(
                      scrollDirection: Axis.vertical,
                      child: SingleChildScrollView(
                        scrollDirection: Axis.horizontal,
                        child: DataTable(
                          dividerThickness: 0,
                          columns: schema
                              .map<DataColumn>((columnDataType) => DataColumn(
                                  label: Text(
                                      columnDataType['columnName'].toString(),
                                      style: TextStyle(
                                          fontStyle: FontStyle.italic))))
                              .toList(),
                          rows: sampleRows
                              .map((row) => DataRow(
                                  cells: schema
                                      .map<DataCell>((columnDataType) =>
                                          DataCell(Text(row[
                                                  columnDataType['columnName']
                                                      .toString()]
                                              .toString())))
                                      .toList()))
                              .toList(),
                        ),
                      )));
        });
  }
}
