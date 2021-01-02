import '../components/searchable_list.dart';
import 'package:flutter/material.dart';

class Models extends StatelessWidget {
  Models(this.dataviewId);
  final String dataviewId;
  final List<String> modelIds = ["model 1", "model 2", "model 3"];
  @override
  Widget build(BuildContext context) {
    return SearchableList(
        items: modelIds
            .map((modelId) =>
                Card(elevation: 3.0, child: ListTile(title: Text(modelId))))
            .toList(),
        searchableText: (item) => item.child.title.data);
  }
}
