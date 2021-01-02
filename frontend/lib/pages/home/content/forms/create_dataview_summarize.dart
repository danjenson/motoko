import 'package:flutter/material.dart';

class CreateDataviewSummarizeForm extends StatefulWidget {
  CreateDataviewSummarizeForm(this.setFormState, this.schema)
      : cols = schema.map((v) => v['columnName'].toString()).toSet();
  final void Function(Map<String, dynamic>, bool Function()) setFormState;
  final List<dynamic> schema;
  final Set<String> cols;
  @override
  _CreateDataviewSummarizeForm createState() => _CreateDataviewSummarizeForm();
}

class _CreateDataviewSummarizeForm extends State<CreateDataviewSummarizeForm> {
  var _formKey = GlobalKey<FormState>();
  var _scrollSummaries = ScrollController();
  var _scrollGroupBys = ScrollController();
  List<Summary> _summaries = [];
  List<String> _groupBys = [];
  @override
  Widget build(BuildContext context) {
    final color = Theme.of(context).colorScheme.secondary;
    final firstCol = widget.schema[0]['columnName'].toString();
    final secondCol = widget.schema[1]['columnName'].toString();
    final defaultSummary = () => Summary(firstCol, Summarizer.SUM);
    if (_summaries.isEmpty) {
      setState(() {
        _summaries = [defaultSummary()];
        _groupBys = [secondCol];
      });
    }
    var usedCols = Set.from(_summaries.map((v) => v.column).toList());
    usedCols.addAll(_groupBys);
    var unusedCols = widget.cols.difference(usedCols);
    return Form(
      key: _formKey,
      autovalidateMode: AutovalidateMode.onUserInteraction,
      onChanged: () {
        widget.setFormState({'summaries': _summaries, 'groupBys': _groupBys},
            _formKey.currentState.validate);
      },
      child: Column(children: [
        Container(
            constraints: BoxConstraints(maxHeight: 250),
            width: double.maxFinite,
            decoration: BoxDecoration(
                border: Border.all(color: color.withOpacity(0.8))),
            child: SingleChildScrollView(
                controller: _scrollSummaries,
                reverse: true,
                child: Column(
                  children: _summaries
                      .asMap()
                      .map((idx, v) => MapEntry(
                          idx,
                          Container(
                              decoration: BoxDecoration(
                                  border: Border(
                                      bottom: BorderSide(
                                          color: color.withOpacity(0.8)))),
                              child: Column(children: [
                                Row(children: [
                                  Expanded(
                                    child: DropdownButtonFormField<String>(
                                        isExpanded: true,
                                        hint: Text('Select column'),
                                        value: _summaries[idx].column,
                                        onChanged: (String v) => setState(
                                            () => _summaries[idx].column = v),
                                        items: widget.schema
                                            .map((v) =>
                                                DropdownMenuItem<String>(
                                                    value: v['columnName']
                                                        .toString(),
                                                    child: Text(v['columnName']
                                                        .toString())))
                                            .toList()),
                                  ),
                                  IntrinsicWidth(
                                      child:
                                          DropdownButtonFormField<Summarizer>(
                                    value: _summaries[idx].summarizer,
                                    items: Summarizer.values
                                        .map((v) =>
                                            DropdownMenuItem<Summarizer>(
                                                value: v,
                                                child: Text(v.toAbbrev())))
                                        .toList(),
                                    onChanged: (Summarizer v) => setState(
                                        () => _summaries[idx].summarizer = v),
                                  )),
                                  IntrinsicWidth(
                                      child: IconButton(
                                          icon: idx == _summaries.length - 1
                                              ? Icon(Icons.add)
                                              : Icon(Icons.remove),
                                          onPressed: () {
                                            setState(() {
                                              if (idx ==
                                                  _summaries.length - 1) {
                                                _summaries
                                                    .add(defaultSummary());
                                                _scrollSummaries.jumpTo(0.0);
                                              } else {
                                                _summaries.removeAt(idx);
                                              }
                                            });
                                          })),
                                ]),
                              ]))))
                      .values
                      .toList(),
                ))),
        SizedBox(height: 10),
        Text('grouping by'),
        SizedBox(height: 10),
        Container(
            constraints: BoxConstraints(maxHeight: 250),
            width: double.maxFinite,
            decoration: BoxDecoration(
                border: Border.all(color: color.withOpacity(0.8))),
            child: SingleChildScrollView(
                controller: _scrollGroupBys,
                reverse: true,
                child: Column(
                  children: _groupBys
                      .asMap()
                      .map(
                        (idx, v) => MapEntry(
                          idx,
                          Container(
                            decoration: BoxDecoration(
                                border: Border(
                                    bottom: BorderSide(
                                        color: color.withOpacity(0.8)))),
                            child: Row(children: [
                              Expanded(
                                  child: DropdownButtonFormField<String>(
                                      isExpanded: true,
                                      hint: Text('Select column'),
                                      value: _groupBys[idx],
                                      onChanged: (String v) =>
                                          setState(() => _groupBys[idx] = v),
                                      items: widget.cols
                                          .map((v) => DropdownMenuItem<String>(
                                              value: v, child: Text(v)))
                                          .toList())),
                              IntrinsicWidth(
                                  child: IconButton(
                                      icon: (idx == _groupBys.length - 1) &&
                                              unusedCols.isNotEmpty
                                          ? Icon(Icons.add)
                                          : Icon(Icons.remove),
                                      onPressed: () {
                                        setState(() {
                                          if (idx == _groupBys.length - 1) {
                                            if (unusedCols.isNotEmpty) {
                                              _groupBys.add(unusedCols.first);
                                              _scrollGroupBys.jumpTo(0.0);
                                            }
                                          } else {
                                            _groupBys.removeAt(idx);
                                          }
                                        });
                                      })),
                            ]),
                          ),
                        ),
                      )
                      .values
                      .toList(),
                )))
      ]),
    );
  }
}

enum Summarizer {
  COUNT,
  MEAN,
  MEDIAN,
  MODE,
  MIN,
  MAX,
  SUM,
  STDDEV,
}

extension ParseToString on Summarizer {
  String toShortString() {
    return this.toString().split('.').last;
  }

  String toAbbrev() {
    return {
      Summarizer.COUNT: 'n',
      Summarizer.MEAN: 'avg',
      Summarizer.MEDIAN: 'med',
      Summarizer.MODE: 'mod',
      Summarizer.MIN: 'min',
      Summarizer.MAX: 'max',
      Summarizer.SUM: 'sum',
      Summarizer.STDDEV: 'std',
    }[this];
  }
}

class Summary {
  Summary(this.column, this.summarizer);
  String column;
  Summarizer summarizer;
  @override
  String toString() {
    return '$summarizer($column)';
  }

  Summary.fromJson(Map<String, dynamic> json)
      : column = json['column'],
        summarizer = {
          'COUNT': Summarizer.COUNT,
          'MEAN': Summarizer.MEAN,
          'MEDIAN': Summarizer.MEDIAN,
          'MODE': Summarizer.MODE,
          'MIN': Summarizer.MIN,
          'MAX': Summarizer.MAX,
          'SUM': Summarizer.SUM,
          'STDDEV': Summarizer.STDDEV,
        }[json['summarizer']];
  Map<String, dynamic> toJson() {
    return {
      'column': column,
      'summarizer': summarizer.toShortString(),
    };
  }
}
