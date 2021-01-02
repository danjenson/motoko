import '../../components/checkbox_list.dart';
import 'package:flutter/material.dart';

class CreateDataviewSelectForm extends StatelessWidget {
  CreateDataviewSelectForm(this.setFormState, this.schema);
  final void Function(Map<String, dynamic>, bool Function()) setFormState;
  final List<dynamic> schema;
  @override
  Widget build(BuildContext context) {
    return CheckboxList(
      items: schema.map((v) => v['columnName'].toString()).toList(),
      onChanged: (selected) {
        setFormState({'columns': selected}, () => selected.isNotEmpty);
      },
    );
  }
}
