import 'package:flutter/material.dart';

class CreateDataviewSortForm extends StatefulWidget {
  CreateDataviewSortForm(this.setFormState, this.schema)
      : cols = schema.map((v) => v['columnName'].toString()).toSet();
  final void Function(Map<String, dynamic>, bool Function()) setFormState;
  final List<dynamic> schema;
  final Set<String> cols;
  @override
  _CreateDataviewSortFormState createState() => _CreateDataviewSortFormState();
}

class _CreateDataviewSortFormState extends State<CreateDataviewSortForm> {
  var _formKey = GlobalKey<FormState>();
  var _scrollController = ScrollController();
  List<Sort> _sorts = [];
  @override
  Widget build(BuildContext context) {
    final color = Theme.of(context).colorScheme.secondary;
    final defaultSort =
        () => Sort(widget.schema[0]['columnName'].toString(), Order.ASCENDING);
    if (_sorts.isEmpty) {
      setState(() => _sorts = [defaultSort()]);
    }
    var usedCols = Set.from(_sorts.map((v) => v.column).toList());
    var unusedCols = widget.cols.difference(usedCols);
    return Container(
        constraints: BoxConstraints(maxHeight: 250),
        width: double.maxFinite,
        decoration:
            BoxDecoration(border: Border.all(color: color.withOpacity(0.8))),
        child: SingleChildScrollView(
            controller: _scrollController,
            reverse: true,
            child: Form(
                key: _formKey,
                autovalidateMode: AutovalidateMode.onUserInteraction,
                onChanged: () {
                  widget.setFormState(
                      {'sorts': _sorts}, _formKey.currentState.validate);
                },
                child: Column(
                    children: _sorts
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
                                          value: _sorts[idx].column,
                                          onChanged: (String v) => setState(
                                              () => _sorts[idx].column = v),
                                          items: widget.schema
                                              .map((v) =>
                                                  DropdownMenuItem<String>(
                                                      value: v['columnName']
                                                          .toString(),
                                                      child: Text(
                                                          v['columnName']
                                                              .toString())))
                                              .toList()),
                                    ),
                                    IntrinsicWidth(
                                        child: DropdownButtonFormField<Order>(
                                      value: _sorts[idx].order,
                                      items: Order.values
                                          .map((v) => DropdownMenuItem<Order>(
                                              value: v,
                                              child: Text(v.toShortString())))
                                          .toList(),
                                      onChanged: (Order v) =>
                                          setState(() => _sorts[idx].order = v),
                                    )),
                                    IntrinsicWidth(
                                        child: IconButton(
                                            icon: (idx == _sorts.length - 1) &&
                                                    unusedCols.isNotEmpty
                                                ? Icon(Icons.add)
                                                : Icon(Icons.remove),
                                            onPressed: () {
                                              setState(() {
                                                if (idx == _sorts.length - 1) {
                                                  if (unusedCols.isNotEmpty) {
                                                    _sorts.add(Sort(
                                                        unusedCols.first,
                                                        Order.ASCENDING));
                                                    _scrollController
                                                        .jumpTo(0.0);
                                                  }
                                                } else {
                                                  _sorts.removeAt(idx);
                                                }
                                              });
                                            })),
                                  ]),
                                ]))))
                        .values
                        .toList()))));
  }
}

enum Order {
  ASCENDING,
  DESCENDING,
}

extension ParseToString on Order {
  String toShortString() {
    return (this == Order.ASCENDING) ? 'Asc' : 'Desc';
  }

  String toFullString() {
    return this.toString().split('.').last;
  }
}

class Sort {
  Sort(this.column, this.order);
  String column;
  Order order;
  @override
  String toString() {
    return '$column ${order.toShortString()}';
  }

  Sort.fromJson(Map<String, dynamic> json)
      : column = json['column'],
        order =
            json['order'] == 'ASCENDING' ? Order.ASCENDING : Order.DESCENDING;
  Map<String, dynamic> toJson() {
    return {
      'column': column,
      'order': order.toFullString(),
    };
  }
}
