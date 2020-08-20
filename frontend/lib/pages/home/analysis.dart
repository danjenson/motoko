import 'package:flutter/material.dart';
import 'adaptive_buttons.dart';
import 'data.dart' as d;
import 'plots.dart' as p;
import 'models.dart' as m;
import 'statistics.dart' as s;
import 'nav.dart';

class Analysis extends StatelessWidget {
  final List<AdaptiveButtonData> buttonData;
  Analysis({Nav nav, String analysisID})
      : buttonData = [
          AdaptiveButtonData(
              icon: Icons.save,
              name: 'data',
              onTap: () =>
                  nav.push('data', d.Data(analysisID: analysisID), d.add)),
          AdaptiveButtonData(
              icon: Icons.functions,
              name: 'statistics',
              onTap: () => nav.push('statistics',
                  s.Statistics(nav: nav, analysisID: analysisID), s.add)),
          AdaptiveButtonData(
              icon: Icons.equalizer,
              name: 'plots',
              onTap: () => nav.push(
                  'plots', p.Plots(nav: nav, analysisID: analysisID), p.add)),
          AdaptiveButtonData(
              icon: Icons.scatter_plot,
              name: 'models',
              onTap: () => nav.push(
                  'models', m.Models(nav: nav, analysisID: analysisID), m.add)),
        ];
  @override
  Widget build(BuildContext context) {
    return AdaptiveButtons(buttonData: buttonData);
  }
}
