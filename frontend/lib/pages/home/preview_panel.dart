import 'package:flutter/material.dart';
import 'package:sliding_up_panel/sliding_up_panel.dart';

class PreviewPanel extends StatelessWidget {
  PreviewPanel(
      {@required this.main,
      @required this.preview,
      this.title = 'preview',
      this.controller});
  final Widget main;
  final Widget preview;
  final String title;
  final PanelController controller;
  @override
  Widget build(BuildContext context) {
    return SlidingUpPanel(
        controller: controller,
        minHeight: 45,
        backdropEnabled: true,
        color: Theme.of(context).cardColor,
        borderRadius: BorderRadius.only(
            topLeft: Radius.circular(20), topRight: Radius.circular(20)),
        body: main,
        panel: Container(
            padding: EdgeInsets.all(10),
            child: Column(
              children: <Widget>[
                Container(
                    height: 3.0,
                    width: 80,
                    color: Theme.of(context).colorScheme.secondary),
                SizedBox(height: 3),
                Text(title,
                    style: TextStyle(color: Colors.white, fontSize: 18.0)),
                SizedBox(height: 8),
                preview,
              ],
            )));
  }
}
