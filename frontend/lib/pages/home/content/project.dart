import '../components/adaptive_buttons.dart';
import '../components/current.dart';
import 'analyses.dart';
import 'datasets.dart';
import 'permissions.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class Project extends StatelessWidget {
  Project(this.id);
  final String id;

  @override
  Widget build(BuildContext context) {
    final current = Provider.of<Current>(context);
    return AdaptiveButtons([
      AdaptiveButtonData(
          icon: Icons.storage,
          name: 'Datasets',
          onTap: () => current.push(Datasets(id), 'Datasets')),
      AdaptiveButtonData(
          icon: Icons.assessment,
          name: 'Analyses',
          onTap: () => current.push(Analyses(id), 'Analyses')),
      AdaptiveButtonData(
          icon: Icons.lock,
          name: 'Permissions',
          onTap: () => current.push(Permissions(id), 'Permissions')),
    ]);
  }
}
