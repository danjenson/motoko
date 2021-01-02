import '../../../common/utils.dart';
import 'package:flutter/material.dart';

class AdaptiveButtons extends StatelessWidget {
  AdaptiveButtons(this.adaptiveButtons);
  final List<AdaptiveButtonData> adaptiveButtons;
  @override
  Widget build(BuildContext context) {
    return Container(
        padding: EdgeInsets.all(20.0),
        child: Flex(
            direction: isPortrait(context) || isWeb()
                ? Axis.vertical
                : Axis.horizontal,
            children: adaptiveButtons
                .map((ab) => Expanded(
                        child: InkWell(
                      onTap: ab.onTap,
                      child: Card(
                          child: Center(
                              child: Column(
                                  mainAxisAlignment: MainAxisAlignment.center,
                                  children: <Widget>[
                            Icon(ab.icon, size: 50.0),
                            SizedBox(height: 5.0),
                            Text(ab.name, style: TextStyle(fontSize: 20.0))
                          ]))),
                    )))
                .toList()));
  }
}

class AdaptiveButtonData {
  AdaptiveButtonData({@required this.icon, this.name, this.onTap});
  final IconData icon;
  final String name;
  final void Function() onTap;
}
