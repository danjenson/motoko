import 'package:flutter/material.dart';
import 'adaptive_buttons.dart';
import 'datasets.dart' as d;
import 'analyses.dart' as a;
import 'permissions.dart' as p;
import 'nav.dart';

class Project extends StatelessWidget {
  final List<AdaptiveButtonData> buttonData;
  Project({Nav nav, String projectID})
      : buttonData = [
          AdaptiveButtonData(
              icon: Icons.storage,
              name: 'datasets',
              onTap: () => nav.push('datasets',
                  d.Datasets(projectID: projectID), d.makeAdder(projectID))),
          AdaptiveButtonData(
              icon: Icons.assessment,
              name: 'analyses',
              onTap: () => nav.push('analyses',
                  a.Analyses(nav: nav, projectID: projectID), a.add)),
          AdaptiveButtonData(
              icon: Icons.lock,
              name: 'permissions',
              onTap: () => nav.push(
                  'permissions', p.Permissions(projectID: projectID), p.add)),
        ];
  @override
  Widget build(BuildContext context) {
    return AdaptiveButtons(buttonData: buttonData);
  }
}
