import 'package:cashmere_core/protocols/name.pb.dart';

class NewCommentView {
  final Name name;
  final String targetManageId;
  final String targetEntityId;
  final String contents;

  NewCommentView(
      {required this.name, required this.targetManageId, required this.targetEntityId, required this.contents});
}
