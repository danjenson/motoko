import 'package:flutter/material.dart';

typedef String Getter(dynamic w);

class SearchableList extends StatefulWidget {
  final List<Widget> items;
  final Getter getter;
  final bool loading;
  SearchableList({this.items, this.getter, this.loading = false}) {
    items.sort((w1, w2) => getter(w1).compareTo(getter(w2)));
  }
  @override
  _SearchableListState createState() => _SearchableListState();
}

class _SearchableListState extends State<SearchableList> {
  List<Widget> filtered = [];
  bool hasFiltered = false;
  @override
  Widget build(BuildContext context) {
    return Column(children: <Widget>[
      Container(
          child: TextField(
              onChanged: (search) {
                setState(() {
                  filtered = widget.items
                      .where((item) =>
                          search.isEmpty ||
                          widget.getter(item).contains(search))
                      .toList();
                  hasFiltered = true;
                });
              },
              style: TextStyle(fontSize: 20),
              decoration: InputDecoration(
                  hintText: "Search", prefixIcon: Icon(Icons.search))),
          padding: EdgeInsets.fromLTRB(25, 10, 25, 10)),
      widget.loading
          ? Expanded(child: Center(child: CircularProgressIndicator()))
          : Expanded(
              child: ListView(
                  padding: EdgeInsets.fromLTRB(20, 0, 20, 20),
                  children: hasFiltered ? filtered : widget.items)),
    ]);
  }
}
