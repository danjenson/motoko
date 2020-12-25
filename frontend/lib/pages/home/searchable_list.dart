import 'package:flutter/material.dart';
import '../../common/types.dart';

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
  var _controller = TextEditingController();
  List<Widget> filtered = [];
  bool hasFiltered = false;
  @override
  Widget build(BuildContext context) {
    return Column(children: <Widget>[
      Container(
          child: TextField(
              controller: _controller,
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
              style: TextStyle(fontSize: 25, color: Colors.white),
              decoration: InputDecoration(
                prefixIcon: Icon(Icons.search, size: 30),
                suffixIcon: Visibility(
                    visible: hasFiltered,
                    child: IconButton(
                      onPressed: () {
                        _controller.clear();
                        setState(() {
                          hasFiltered = false;
                        });
                      },
                      icon: Icon(Icons.clear, size: 30),
                    )),
                hintText: "Search",
              )),
          padding: EdgeInsets.fromLTRB(15, 10, 15, 10)),
      widget.loading
          ? Expanded(child: Center(child: CircularProgressIndicator()))
          : Expanded(
              child: ListView(
                  padding: EdgeInsets.fromLTRB(10, 0, 10, 225),
                  children: hasFiltered ? filtered : widget.items)),
    ]);
  }
}
