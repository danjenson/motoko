import 'package:flutter/material.dart';
import 'adaptive_buttons.dart';
import 'datasets.dart' as d;
import 'analyses.dart' as a;
import 'permissions.dart' as p;
import 'nav.dart';

class Project extends StatelessWidget {
  final List<AdaptiveButtonData> buttonData;
  Project({Nav nav, String projectId})
      : buttonData = [
          AdaptiveButtonData(
              icon: Icons.storage,
              name: 'datasets',
              onTap: () => nav.push('datasets',
                  d.Datasets(projectId: projectId), d.makeAdder(projectId))),
          AdaptiveButtonData(
              icon: Icons.assessment,
              name: 'analyses',
              onTap: () => nav.push(
                  'analyses',
                  a.Analyses(nav: nav, projectId: projectId),
                  a.makeAdder(projectId))),
          AdaptiveButtonData(
              icon: Icons.lock,
              name: 'permissions',
              onTap: () => nav.push(
                  'permissions', p.Permissions(projectId: projectId), p.add)),
        ];
  @override
  Widget build(BuildContext context) {
    return AdaptiveButtons(buttonData: buttonData);
  }
}
