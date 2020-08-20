import 'package:flutter/material.dart';
import 'package:photo_view/photo_view.dart';
import '../../common/platform_svg.dart';
import 'preview_panel.dart';

class Plot extends StatelessWidget {
  Plot({@required this.plotID});
  final String plotID;
  final List<String> items = ['field 1', 'field 2', 'field 3'];
  @override
  Widget build(BuildContext context) {
    return PreviewPanel(
      main: ListView(
          padding: EdgeInsets.all(20),
          children: items
              .map((opID) =>
                  Card(elevation: 3.0, child: ListTile(title: Text(opID))))
              .toList()),
      title: 'view plot',
      preview: Expanded(
          child: ClipRect(
              child: PhotoView.customChild(
                  backgroundDecoration:
                      BoxDecoration(color: Colors.transparent),
                  child: PlatformSvg.asset('images/test.svg')))),
    );
  }
}
