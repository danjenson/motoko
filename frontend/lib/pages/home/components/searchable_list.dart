import 'package:flutter/material.dart';

class SearchableList extends StatefulWidget {
  SearchableList(
      {@required this.items,
      @required this.searchableText,
      this.loading = false,
      this.bottomPadding = 0});
  final List<Widget> items;
  final String Function(dynamic v) searchableText;
  final bool loading;
  final double bottomPadding;
  @override
  _SearchableListState createState() => _SearchableListState();
}

class _SearchableListState extends State<SearchableList> {
  var _controller = TextEditingController();
  List<Widget> filtered = [];
  bool hasFiltered = false;
  @override
  Widget build(BuildContext context) {
    return Padding(
        padding: EdgeInsets.fromLTRB(10, 0, 10, 0),
        child: Column(children: <Widget>[
          Container(
              child: TextField(
                  controller: _controller,
                  autofocus: false,
                  onChanged: (search) {
                    setState(() {
                      filtered = widget.items
                          .where((item) =>
                              search.isEmpty ||
                              widget
                                  .searchableText(item)
                                  .toLowerCase()
                                  .contains(search.toLowerCase()))
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
                          icon: Icon(Icons.clear),
                        )),
                    hintText: 'Search',
                  )),
              padding: EdgeInsets.fromLTRB(15, 10, 15, 10)),
          widget.loading
              ? Expanded(child: Center(child: CircularProgressIndicator()))
              : Expanded(
                  child: ListView(
                      padding: EdgeInsets.fromLTRB(
                          10, 0, 10, 85 + widget.bottomPadding),
                      children: hasFiltered ? filtered : widget.items)),
        ]));
  }
}
