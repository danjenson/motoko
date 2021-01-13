import '../../../common/dialogs.dart';
import '../../../common/tier.dart';
import '../components/current.dart';
import '../components/globals.dart' as globals;
import '../components/preview_panel.dart';
import '../components/searchable_list.dart';
import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:provider/provider.dart';
import 'package:sliding_up_panel/sliding_up_panel.dart';

class Content extends StatefulWidget {
  Content({
    @required this.listQuery,
    @required this.listQueryVariables,
    @required this.toTitleString,
    this.toPrimarySubtitleString,
    this.toSecondarySubtitleString,
    this.toButtons,
    this.canDelete,
    this.orderBy,
    this.onTap,
    this.onTapPreview,
    this.defaultPreviewString,
    this.createName,
    this.makeCreateForm,
    this.createMutation,
    this.createFieldsToVariables,
    this.onCreate,
    this.createOnLastFailureMessage,
    this.createOnLastWorkingMessage,
  });
  final String listQuery;
  final Map<String, dynamic> listQueryVariables;
  final String Function(dynamic v) toTitleString;
  final String Function(dynamic v) toPrimarySubtitleString;
  final String Function(dynamic v) toSecondarySubtitleString;
  final List<FlatButton> Function(dynamic v, Current current) toButtons;
  final bool Function(dynamic v, List<dynamic> results) canDelete;
  final Comparable Function(dynamic v) orderBy;
  final void Function(dynamic v, Current current) onTap;
  final Widget Function(String id) onTapPreview;
  final String defaultPreviewString;
  final String createName;
  final Widget Function(
      void Function(Map<String, dynamic>, bool Function()) setFormState,
      List<dynamic> values) makeCreateForm;
  final String createMutation;
  final Map<String, dynamic> Function(dynamic fields) createFieldsToVariables;
  final void Function(dynamic v, Current current, VoidCallback refetch)
      onCreate;
  final String createOnLastFailureMessage;
  final String createOnLastWorkingMessage;
  final String deleteNode = '''
    mutation DeleteNode(\$id: ID!) {
      deleteNode(id: \$id)
    }
  ''';
  final int minRefetchDelaySeconds = 2;
  final int maxRefetchDelaySeconds = 20;
  @override
  _ContentState createState() => _ContentState();
}

class _ContentState extends State<Content> {
  final _panelController = PanelController();
  String _selectedId;
  int _refetchDelaySeconds = 2;
  @override
  Widget build(BuildContext context) {
    final current = Provider.of<Current>(context, listen: false);
    return Query(
        options: QueryOptions(
            fetchPolicy: FetchPolicy.cacheAndNetwork,
            documentNode: gql(widget.listQuery),
            variables: widget.listQueryVariables),
        builder: (QueryResult result,
            {VoidCallback refetch, FetchMore fetchMore}) {
          var values = [];
          if (!result.loading) {
            if (result.hasException) {
              showErrorDialog(context, result.exception.toString());
            } else {
              values = (result.data as Map).values.first;
              if (values.isNotEmpty) {
                if (widget.orderBy != null) {
                  values.sort(
                      (a, b) => widget.orderBy(a).compareTo(widget.orderBy(b)));
                } else {
                  values.sort((a, b) {
                    var av = widget.toTitleString(a) +
                        (widget.toPrimarySubtitleString?.call(a) ?? '') +
                        (widget.toSecondarySubtitleString?.call(a) ?? '');
                    var bv = widget.toTitleString(b) +
                        (widget.toPrimarySubtitleString?.call(b) ?? '') +
                        (widget.toSecondarySubtitleString?.call(b) ?? '');
                    return av.compareTo(bv);
                  });
                }
                final lastStatus =
                    values[values.length - 1]['status'].toString();
                if (lastStatus == 'FAILED' &&
                    widget.createOnLastFailureMessage != null) {
                  globals.create = () => showErrorDialog(
                      context, widget.createOnLastFailureMessage);
                } else if (lastStatus != 'COMPLETED' &&
                    widget.createOnLastWorkingMessage != null) {
                  globals.create = () => showInfoDialog(
                      context, widget.createOnLastWorkingMessage);
                } else {
                  globals.create = () => showCreateDialog(
                      context: context,
                      name: widget.createName,
                      makeForm: (setFormState) =>
                          widget.makeCreateForm(setFormState, values),
                      mutation: widget.createMutation,
                      fieldsToVariables: widget.createFieldsToVariables,
                      onCreate: (v) => widget.onCreate(v, current, refetch));
                }
              } else {
                globals.create = () => showCreateDialog(
                    context: context,
                    name: widget.createName,
                    makeForm: (setFormState) =>
                        widget.makeCreateForm(setFormState, values),
                    mutation: widget.createMutation,
                    fieldsToVariables: widget.createFieldsToVariables,
                    onCreate: (v) => widget.onCreate(v, current, refetch));
              }
            }
          }
          var shouldRefetch = false;
          final List<Widget> items = values.map<Widget>((v) {
            final status = v['status'].toString();
            final isCompleted = status == 'COMPLETED';
            final isError = status == 'FAILED';
            final inProgress = ['QUEUED', 'RUNNING'].contains(status);
            shouldRefetch |= inProgress;
            Widget subtitle;
            if (widget.toPrimarySubtitleString != null &&
                widget.toSecondarySubtitleString != null &&
                widget.toPrimarySubtitleString(v).isNotEmpty &&
                widget.toSecondarySubtitleString(v).isNotEmpty) {
              subtitle = RichText(
                  text: TextSpan(
                      text: widget.toPrimarySubtitleString(v),
                      style: TextStyle(color: Colors.white),
                      children: <TextSpan>[
                    TextSpan(
                        text: widget.toSecondarySubtitleString(v),
                        style: TextStyle(color: Colors.grey))
                  ]));
            } else if (widget.toPrimarySubtitleString != null &&
                widget.toPrimarySubtitleString(v).isNotEmpty) {
              subtitle = Text(widget.toPrimarySubtitleString(v),
                  style: TextStyle(color: Colors.white));
            } else if (widget.toSecondarySubtitleString != null &&
                widget.toSecondarySubtitleString(v).isNotEmpty) {
              subtitle = Text(widget.toSecondarySubtitleString(v),
                  style: TextStyle(color: Colors.grey));
            }
            var children = <Widget>[
              ListTile(
                  title: Text(widget.toTitleString(v),
                      style: TextStyle(fontSize: 20)),
                  subtitle: subtitle,
                  selected: v['id'].toString() == _selectedId,
                  onTap: widget.onTap != null || widget.onTapPreview != null
                      ? () {
                          if (widget.onTap != null) {
                            widget.onTap(v, current);
                          }
                          if (widget.onTapPreview != null && isCompleted) {
                            setState(() {
                              _selectedId = v['id'].toString();
                              _panelController.animatePanelToPosition(1.0);
                            });
                          }
                        }
                      : null,
                  trailing:
                      Row(mainAxisSize: MainAxisSize.min, children: <Widget>[
                    Visibility(
                        visible: isError || inProgress,
                        child: isError
                            ? IconButton(
                                onPressed: () {
                                  final tier =
                                      Provider.of<Tier>(context, listen: false)
                                          .tier;
                                  if (tier == 'prod') {
                                    final msg = 'There was an error. Check that'
                                        ' the operation makes sense given the'
                                        ' provided data.';
                                    showErrorDialog(context, msg);
                                  } else {
                                    showErrorDialog(
                                        context, v['error']['message']);
                                  }
                                },
                                icon: Icon(Icons.error, size: 30))
                            : IconButton(
                                onPressed: () => {},
                                icon: Icon(Icons.cloud_upload, size: 30))),
                    Visibility(
                        visible: widget.canDelete?.call(v, values) ?? true,
                        child: IconButton(
                            onPressed: () async {
                              var closeProgress =
                                  showProgressDialog('Tidying up...');
                              final client = GraphQLProvider.of(context).value;
                              final mutOpts = MutationOptions(
                                fetchPolicy: FetchPolicy.networkOnly,
                                documentNode: gql(widget.deleteNode),
                                variables: {'id': v['id'].toString()},
                                onCompleted: (v) => refetch(),
                              );
                              var result = await client.mutate(mutOpts);
                              closeProgress();
                              if (result.hasException) {
                                showErrorDialog(
                                    context, result.exception.toString());
                              }
                            },
                            icon: Icon(Icons.delete_outline, size: 30)))
                  ]))
            ];
            if (widget.toButtons != null) {
              children.add(Row(
                  mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                  children: widget.toButtons(v, current)));
            }
            return Card(child: Column(children: children));
          }).toList();

          if (shouldRefetch && result.source == QueryResultSource.Network) {
            Future.delayed(Duration(milliseconds: _refetchDelaySeconds * 1000),
                () {
              // refetch every [min]->3->5->10->[max] seconds
              _refetchDelaySeconds = {
                    2: 3,
                    3: 5,
                    5: 10,
                    10: widget.maxRefetchDelaySeconds
                  }[_refetchDelaySeconds] ??
                  widget.maxRefetchDelaySeconds;
              refetch();
            });
          } else {
            _refetchDelaySeconds = widget.minRefetchDelaySeconds;
          }

          String searchableText(dynamic v) {
            var s = v.child.children[0].title.data;
            if (v.child.children[0].subtitle != null) {
              if (v.child.children[0].subtitle is Text) {
                s += '\n' + v.child.children[0].subtitle.data;
              }
              if (v.child.children[0].subtitle is RichText) {
                s += '\n' + v.child.children[0].subtitle.text.text;
              }
            }
            return s;
          }

          if (widget.onTapPreview != null) {
            if (widget.defaultPreviewString == null) {
              throw ('Default Preview String not provided!');
            }
            return PreviewPanel(
                controller: _panelController,
                main: SearchableList(
                    items: items,
                    searchableText: searchableText,
                    loading: result.loading,
                    bottomPadding: 150),
                preview: Expanded(
                    child: _selectedId == null
                        ? Center(
                            child: Text(widget.defaultPreviewString,
                                style: TextStyle(fontSize: 20)))
                        : widget.onTapPreview(_selectedId)));
          }
          return SearchableList(
              items: items,
              searchableText: searchableText,
              loading: result.loading);
        });
  }
}
