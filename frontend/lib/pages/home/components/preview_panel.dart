import 'package:flutter/material.dart';
import 'package:sliding_up_panel/sliding_up_panel.dart';

class PreviewPanel extends StatelessWidget {
  PreviewPanel({@required this.main, @required this.preview, this.controller});
  final Widget main;
  final Widget preview;
  final PanelController controller;
  @override
  Widget build(BuildContext context) {
    final color = Theme.of(context).colorScheme.secondary;
    return SlidingUpPanel(
        controller: controller,
        margin: EdgeInsets.fromLTRB(10, 0, 10, 0),
        minHeight: 35,
        backdropEnabled: true,
        color: Theme.of(context).cardColor,
        borderRadius: BorderRadius.only(
            topLeft: Radius.circular(10), topRight: Radius.circular(10)),
        border: Border.all(color: color.withOpacity(0.65)),
        body: main,
        panel: GestureDetector(
            onTap: () {
              if (controller.isPanelOpen) {
                controller.animatePanelToPosition(0.0);
              } else if (controller.isPanelClosed) {
                controller.animatePanelToPosition(1.0);
              }
            },
            behavior: HitTestBehavior.opaque,
            child: Container(
                padding: EdgeInsets.all(10),
                child: Column(
                  children: <Widget>[
                    Container(height: 3.0, width: 80, color: color),
                    SizedBox(height: 20),
                    preview,
                  ],
                ))));
  }
}
