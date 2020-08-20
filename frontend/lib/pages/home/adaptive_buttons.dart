import 'package:flutter/material.dart';
import '../../common/utils.dart';

class AdaptiveButtonData {
  AdaptiveButtonData({@required this.icon, this.name, this.onTap});
  final IconData icon;
  final String name;
  final void Function() onTap;
}

class AdaptiveButtons extends StatelessWidget {
  AdaptiveButtons({@required this.buttonData});
  final List<AdaptiveButtonData> buttonData;
  @override
  Widget build(BuildContext context) {
    return Container(
        padding: EdgeInsets.all(20.0),
        child: Flex(
            direction: isPortrait(context) || isWeb()
                ? Axis.vertical
                : Axis.horizontal,
            children: buttonData
                .map((bd) => Expanded(
                        child: InkWell(
                      onTap: bd.onTap,
                      child: Card(
                          elevation: 3.0,
                          child: Center(
                              child: Column(
                                  mainAxisAlignment: MainAxisAlignment.center,
                                  children: <Widget>[
                                Icon(bd.icon, size: 50.0),
                                SizedBox(height: 5.0),
                                Text(bd.name, style: TextStyle(fontSize: 20.0))
                              ]))),
                    )))
                .toList()));
  }
}
