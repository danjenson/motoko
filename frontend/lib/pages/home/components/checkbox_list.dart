import 'package:flutter/material.dart';

class CheckboxList extends StatefulWidget {
  CheckboxList({this.items, this.onChanged, this.height = 250});
  final List<String> items;
  final void Function(List<String>) onChanged;
  final double height;
  @override
  _CheckboxListState createState() => _CheckboxListState();
}

class _CheckboxListState extends State<CheckboxList> {
  bool _selectAll = false;
  Map<String, bool> _checked;
  @override
  Widget build(BuildContext context) {
    var color = Theme.of(context).colorScheme.secondary;
    final updateParent = () {
      final selected = Map<String, bool>.from(_checked);
      selected.removeWhere((v, selected) => !selected);
      widget.onChanged(selected.keys.toList());
    };
    if (_checked == null) {
      setState(() {
        _checked =
            Map.fromIterable(widget.items, key: (v) => v, value: (v) => false);
      });
    }
    if (Set.from(_checked.keys).difference(Set.from(widget.items)).isNotEmpty) {
      setState(() {
        _checked.removeWhere((k, v) => !widget.items.contains(k));
        _selectAll = false;
      });
    }
    return Column(children: [
      CheckboxListTile(
          title: Text('Select all'),
          contentPadding: EdgeInsets.all(0),
          activeColor: Theme.of(context).colorScheme.secondary,
          controlAffinity: ListTileControlAffinity.leading,
          value: _selectAll,
          onChanged: (selectAll) {
            setState(() {
              _selectAll = selectAll;
              widget.items.forEach((v) => _checked[v] = selectAll);
            });
            updateParent();
          }),
      Container(
          constraints: BoxConstraints(maxHeight: 250),
          width: double.maxFinite,
          decoration:
              BoxDecoration(border: Border.all(color: color.withOpacity(0.8))),
          child: SingleChildScrollView(
              child: Column(
                  children: widget.items
                      .map((v) => CheckboxListTile(
                            title: Text(v, overflow: TextOverflow.ellipsis),
                            contentPadding: EdgeInsets.all(0),
                            activeColor: color,
                            controlAffinity: ListTileControlAffinity.leading,
                            value: _checked[v] == null ? false : _checked[v],
                            onChanged: (checked) {
                              setState(() {
                                _checked[v] = checked;
                                _selectAll = _checked.values.every((v) => v);
                              });
                              updateParent();
                            },
                          ))
                      .toList())))
    ]);
  }
}
