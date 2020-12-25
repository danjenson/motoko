import '../../common/error_dialog.dart';
import '../../common/form_dialog.dart';
import '../../common/globals.dart' as globals;
import '../../common/types.dart';
import '../../common/utils.dart';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';
import 'package:flutter_svg/svg.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:photo_view/photo_view.dart';
import 'package:sliding_up_panel/sliding_up_panel.dart';
import 'preview_panel.dart';
import 'query_results_list.dart';

class Plots extends StatefulWidget {
  Plots(this.dataviewId);
  final String dataviewId;
  final plots = '''
    query Plots(\$dataviewId: ID!) {
      plots(dataviewId: \$dataviewId) {
        __typename
        id
        createdAt
        updatedAt
        name
        type
        args
        status
      }
    }
  ''';
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
  _PlotsState createState() => _PlotsState();
}

class _PlotsState extends State<Plots> {
  String _selectedId;
  String _uri;
  PanelController _controller = PanelController();
  @override
  Widget build(BuildContext context) {
    return PreviewPanel(
        controller: _controller,
        main: QueryResultsList(
          query: widget.plots,
          variables: {'dataviewId': widget.dataviewId},
          getter: (v) => v['plots'],
          title: (v) => v['name'],
          subtitle: (v) {
            if (v['args'] == null) {
              return null;
            }
            var args = jsonDecode(v['args']) as Map;
            var sub = 'type: ${v['type'].toLowerCase()}\n' +
                ['title', 'x', 'y', 'color', 'shape']
                    .where((v) => args.containsKey(v))
                    .map((v) => '$v: ${args[v]}')
                    .join('\n');
            return Text(sub, style: TextStyle(color: Colors.grey));
          },
          onTap: (v) async {
            if (v['status'] == 'COMPLETED') {
              final client = GraphQLProvider.of(context).value;
              final queryOpts = QueryOptions(
                  fetchPolicy: FetchPolicy.cacheFirst,
                  documentNode: gql(widget.plotUri),
                  variables: {'id': v['id']});
              final res = await client.query(queryOpts);
              if (res.hasException) {
                errorDialog(
                    context: context, message: res.exception.toString());
              }
              this.setState(() {
                _selectedId = v['id'];
                _uri = res.data['node']['uri'];
                _controller.animatePanelToPosition(1.0);
              });
            }
          },
          selectedId: _selectedId,
        ),
        preview: _uri == null
            ? Expanded(
                child: Center(
                    child: Text('Tap a plot to view.',
                        style: TextStyle(fontSize: 20.0))))
            : Expanded(
                child: ClipRect(
                    child: PhotoView.customChild(
                        backgroundDecoration:
                            BoxDecoration(color: Colors.transparent),
                        child: SvgPicture.network(_uri)))));
  }
}

class NewPlotForm extends StatefulWidget {
  NewPlotForm({@required this.client, @required this.dataviewId});
  final GraphQLClient client;
  final String dataviewId;
  final schema = '''
    query Node(\$id: ID!) {
      node(id: \$id) {
        __typename
        id
        ... on Dataview {
          schema {
            columnName
            dataType
          }
        }
      }
    }
  ''';
  final createPlot = '''
    mutation CreatePlot(
      \$dataviewId: ID!,
      \$name: String!,
      \$type: PlotType!,
      \$args: JSON!,
    ) {
      createPlot(
        dataviewId: \$dataviewId,
        name: \$name,
        type: \$type,
        args: \$args,
      ) {
        __typename
        id
        createdAt
        updatedAt
        name
        type
        args
        status
      }
    }
  ''';
  @override
  _NewPlotFormState createState() => _NewPlotFormState();
}

class _NewPlotFormState extends State<NewPlotForm> {
  final _formKey = GlobalKey<FormBuilderState>();
  String _plotType = '';
  @override
  Widget build(BuildContext context) {
    return Query(
        options: QueryOptions(
          fetchPolicy: FetchPolicy.cacheAndNetwork,
          documentNode: gql(widget.schema),
          variables: {'id': widget.dataviewId},
        ),
        builder: (QueryResult result,
            {VoidCallback refetch, FetchMore fetchMore}) {
          var schema = [];
          var loading =
              result.loading && result.source == QueryResultSource.Loading;
          if (result.hasException) {
            return ErrorDialog(result.exception.toString());
          } else if (!loading) {
            schema = result.data['node']['schema'] ?? [];
          }
          return Padding(
              padding: EdgeInsets.all(10),
              child: SingleChildScrollView(
                child: Column(children: <Widget>[
                  FormBuilder(
                      key: _formKey,
                      autovalidateMode: AutovalidateMode.disabled,
                      skipDisabled: true,
                      child: Column(children: <Widget>[
                        FormBuilderTextField(
                          name: 'title',
                          validator: FormBuilderValidators.required(context),
                          decoration: InputDecoration(
                              hintText: 'title', labelText: 'title'),
                        ),
                        FormBuilderDropdown<String>(
                          name: 'type',
                          focusColor: Theme.of(context).colorScheme.secondary,
                          decoration: InputDecoration(
                              hintText: 'type', labelText: 'type'),
                          validator: FormBuilderValidators.required(context),
                          items: [
                            'Bar',
                            'Histogram',
                            'Line',
                            'Scatter',
                            'Smooth'
                          ]
                              .map((v) =>
                                  DropdownMenuItem(value: v, child: Text(v)))
                              .toList(),
                          onChanged: (v) => setState(() {
                            _plotType = v;
                          }),
                        ),
                        FormBuilderDropdown<String>(
                            name: 'x',
                            decoration: InputDecoration(
                                hintText: 'x-axis', labelText: 'x-axis'),
                            validator: FormBuilderValidators.required(context),
                            items: schema
                                .where((v) {
                                  if (['Histogram', 'Line', 'Scatter', 'Smooth']
                                      .contains(_plotType)) {
                                    return isNumericDataType(v['dataType']);
                                  }
                                  if (_plotType == 'Bar') {
                                    return isCategoricalDataType(v['dataType']);
                                  }
                                  return false;
                                })
                                .map((v) => DropdownMenuItem(
                                    value: v['columnName'].toString(),
                                    child: Text(v['columnName'])))
                                .toList()),
                        Visibility(
                            visible: !['Bar', 'Histogram'].contains(_plotType),
                            child: FormBuilderDropdown<String>(
                                name: 'y',
                                enabled:
                                    !['Bar', 'Histogram'].contains(_plotType),
                                decoration: InputDecoration(
                                    hintText: 'y-axis', labelText: 'y-axis'),
                                validator:
                                    FormBuilderValidators.required(context),
                                items: schema
                                    .where(
                                        (v) => isNumericDataType(v['dataType']))
                                    .map((v) => DropdownMenuItem(
                                        value: v['columnName'].toString(),
                                        child: Text(v['columnName'])))
                                    .toList())),
                        Visibility(
                            visible: _plotType != 'Histogram',
                            child: FormBuilderDropdown<String>(
                                name: 'color',
                                enabled: _plotType != 'Histogram',
                                decoration: InputDecoration(
                                    hintText: '[color]', labelText: '[color]'),
                                allowClear: true,
                                items: schema
                                    .where((v) =>
                                        isCategoricalDataType(v['dataType']))
                                    .map((v) => DropdownMenuItem(
                                        value: v['columnName'].toString(),
                                        child: Text(v['columnName'])))
                                    .toList())),
                        Visibility(
                            visible: ['Scatter', 'Smooth'].contains(_plotType),
                            child: FormBuilderDropdown<String>(
                                name: 'shape',
                                enabled:
                                    ['Scatter', 'Smooth'].contains(_plotType),
                                decoration: InputDecoration(
                                    hintText: '[shape]', labelText: '[shape]'),
                                allowClear: true,
                                items: schema
                                    .where((v) =>
                                        isCategoricalDataType(v['dataType']))
                                    .map((v) => DropdownMenuItem(
                                        value: v['columnName'].toString(),
                                        child: Text(v['columnName'])))
                                    .toList())),
                      ])),
                  SizedBox(height: 10),
                  Row(
                      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                      children: <Widget>[
                        FlatButton(
                            child: Text(
                              'CANCEL',
                              style: TextStyle(
                                  color:
                                      Theme.of(context).colorScheme.secondary),
                            ),
                            onPressed: () {
                              Navigator.of(context).pop();
                            }),
                        FlatButton(
                            child: Text(
                              'CREATE',
                              style: TextStyle(
                                  color:
                                      Theme.of(context).colorScheme.secondary),
                            ),
                            onPressed: () async {
                              if (_formKey.currentState.saveAndValidate()) {
                                var args =
                                    Map.from(_formKey.currentState.value);
                                args.removeWhere((k, v) => v == null);
                                var type = args
                                    .remove('type')
                                    .toString()
                                    .toUpperCase();
                                final queryOpts = QueryOptions(
                                    fetchPolicy: FetchPolicy.networkOnly,
                                    documentNode: gql(widget.createPlot),
                                    variables: {
                                      'dataviewId': widget.dataviewId,
                                      'name':
                                          _formKey.currentState.value['title'],
                                      'type': type,
                                      'args': jsonEncode(args),
                                    });
                                final res =
                                    await widget.client.query(queryOpts);
                                if (res.hasException) {
                                  errorDialog(
                                      context: context,
                                      message: res.exception.toString());
                                }
                                Navigator.of(context).pop();
                                globals.refetch();
                              }
                            })
                      ])
                ]),
              ));
        });
  }
}

Adder makeAdder(String dataviewId) {
  void add(BuildContext context) {
    final client = GraphQLProvider.of(context).value;
    final form = NewPlotForm(client: client, dataviewId: dataviewId);
    formDialog(context: context, title: 'New Plot', form: form);
  }

  return add;
}
