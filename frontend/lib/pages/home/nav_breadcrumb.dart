import 'package:flutter/material.dart';
import 'package:flutter_breadcrumb/flutter_breadcrumb.dart';
import 'nav.dart';

class NavBreadcrumb extends StatelessWidget {
  NavBreadcrumb({@required this.nav});
  final Nav nav;
  @override
  Widget build(BuildContext context) {
    return BottomAppBar(
        elevation: 5.0,
        color: Theme.of(context).primaryColor,
        child: Padding(
            padding: EdgeInsets.all(12),
            child: BreadCrumb(
                items: nav.names
                    .asMap()
                    .map((idx, name) => MapEntry(
                        idx,
                        BreadCrumbItem(
                            onTap: () => nav.to(idx),
                            content: Text(name,
                                style: TextStyle(
                                    color: Colors.white, fontSize: 25.0)))))
                    .values
                    .toList(),
                overflow: ScrollableOverflow(reverse: true),
                divider: Icon(Icons.keyboard_arrow_right,
                    size: 35.0,
                    color: Theme.of(context).colorScheme.secondary))));
  }
}
