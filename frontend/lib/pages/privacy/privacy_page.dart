import 'package:flutter/material.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'package:flutter_markdown/flutter_markdown.dart';

class PrivacyPage extends StatefulWidget {
  @override
  _PrivacyPageState createState() => _PrivacyPageState();
}

class _PrivacyPageState extends State<PrivacyPage> {
  var contents = '';
  loadContents() async {
    var value = await rootBundle.loadString('legal/privacy/policy.md');
    setState(() {
      contents = value;
    });
  }

  @override
  Widget build(BuildContext context) {
    loadContents();
    var theme = Theme.of(context);
    var textTheme = theme.textTheme;
    var h1 = TextStyle(fontSize: 40, color: theme.colorScheme.secondary);
    var h2 = TextStyle(fontSize: 25, color: theme.colorScheme.secondary);
    var mdTheme = theme.copyWith(
        textTheme: textTheme.copyWith(headline5: h1, headline6: h2));
    return Scaffold(
        backgroundColor: Colors.black,
        body: Center(
            child: Container(
                constraints: BoxConstraints(minWidth: 200, maxWidth: 1000),
                child: Markdown(
                    data: contents,
                    styleSheet: MarkdownStyleSheet.fromTheme(mdTheme).copyWith(
                      textScaleFactor: 1.5,
                      blockSpacing: 20,
                    )))));
  }
}
