import 'current.dart';
import 'package:flutter/material.dart';
import 'package:flutter_breadcrumb/flutter_breadcrumb.dart';
import 'package:provider/provider.dart';

class ContentBreadcrumb extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final current = Provider.of<Current>(context);
    return BottomAppBar(
        child: Padding(
            padding: EdgeInsets.all(12),
            child: BreadCrumb(
                items: current.names
                    .asMap()
                    .map((idx, name) => MapEntry(
                        idx,
                        BreadCrumbItem(
                            onTap: () => current.to(idx),
                            content: Text(name.toLowerCase(),
                                style: TextStyle(
                                    color: Colors.white, fontSize: 25.0)))))
                    .values
                    .toList(),
                overflow: ScrollableOverflow(reverse: true),
                divider: Icon(Icons.keyboard_arrow_right, size: 35.0))));
  }
}
