import '../../../../common/utils.dart';
import 'package:flutter/material.dart';

class CreateDataviewFilterForm extends StatefulWidget {
  CreateDataviewFilterForm(this.setFormState, this.schema)
      : isNumericCol = Map.fromIterable(schema,
            key: (v) => v['columnName'].toString(),
            value: (v) => isNumericDataType(v['dataType'].toString()));
  final void Function(Map<String, dynamic>, bool Function()) setFormState;
  final List<dynamic> schema;
  final Map<String, bool> isNumericCol;
  @override
  _CreateDataviewFilterFormState createState() =>
      _CreateDataviewFilterFormState();
}

class _CreateDataviewFilterFormState extends State<CreateDataviewFilterForm> {
  var _formKey = GlobalKey<FormState>();
  var _scrollController = ScrollController();
  List<Filter> _filters = [];
  List<TextEditingController> _textControllers = [TextEditingController()];
  @override
  Widget build(BuildContext context) {
    final color = Theme.of(context).colorScheme.secondary;
    final defaultFilter =
        () => Filter(widget.schema[0]['columnName'].toString(), '=', null);
    if (_filters.isEmpty) {
      setState(() => _filters = [defaultFilter()]);
    }
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
                      {'filters': _filters}, _formKey.currentState.validate);
                },
                child: Column(
                    children: _filters
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
                                          value: _filters[idx].column,
                                          onChanged: (String v) => setState(
                                              () => _filters[idx].column = v),
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
                                        child: DropdownButtonFormField<String>(
                                      value: _filters[idx].comparator,
                                      items: ['=', '>', '>=', '<', '<=']
                                          .map((v) => DropdownMenuItem<String>(
                                              value: v, child: Text(v)))
                                          .toList(),
                                      onChanged: (String v) => setState(
                                          () => _filters[idx].comparator = v),
                                    )),
                                  ]),
                                  Row(children: [
                                    Expanded(
                                        child: TextFormField(
                                            keyboardType: widget.isNumericCol[
                                                    _filters[idx].column]
                                                ? TextInputType.number
                                                : TextInputType.text,
                                            validator: (v) {
                                              if (v == null || v.isEmpty) {
                                                return 'value required';
                                              } else if (widget.isNumericCol[
                                                      _filters[idx].column] &&
                                                  !isNumeric(v)) {
                                                return 'must be numeric';
                                              }
                                              return null;
                                            },
                                            controller: _textControllers[idx],
                                            decoration: InputDecoration(
                                                border: InputBorder.none,
                                                focusedBorder: InputBorder.none,
                                                enabledBorder: InputBorder.none,
                                                errorBorder: InputBorder.none,
                                                disabledBorder:
                                                    InputBorder.none,
                                                contentPadding:
                                                    EdgeInsets.fromLTRB(
                                                        15, 0, 10, 0),
                                                hintText: 'Enter value'),
                                            onChanged: (v) => setState(() =>
                                                _filters[idx].value = v))),
                                    IconButton(
                                        icon: idx == _filters.length - 1
                                            ? Icon(Icons.add)
                                            : Icon(Icons.remove),
                                        onPressed: () {
                                          setState(() {
                                            if (idx == _filters.length - 1) {
                                              _filters.add(defaultFilter());
                                              _textControllers
                                                  .add(TextEditingController());
                                              _scrollController.jumpTo(0.0);
                                            } else {
                                              _filters.removeAt(idx);
                                              _textControllers.removeAt(idx);
                                            }
                                          });
                                        }),
                                  ]),
                                ]))))
                        .values
                        .toList()))));
  }
}

class Filter {
  Filter(this.column, this.comparator, this.value);
  String column;
  String comparator;
  String value;
  @override
  String toString() {
    return '$column $comparator $value';
  }

  Filter.fromJson(Map<String, dynamic> json)
      : column = json['column'],
        comparator = json['comparator'],
        value = json['value'];
  Map<String, dynamic> toJson() {
    return {
      'column': column,
      'comparator': comparator,
      'value': value,
    };
  }
}
