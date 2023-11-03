import 'package:cashmere_core/new_entity_calls/view_to_request.dart';
import 'package:cashmere_core/protocols/comment.pb.dart';
import 'package:cashmere_core/protocols/name.pb.dart';

class NewCommentView extends ViewToRequest<NewCommentRequest> {
  final Name name;
  final String targetManageId;
  final String targetEntityId;
  final String contents;

  NewCommentView(
      {required this.name, required this.targetManageId, required this.targetEntityId, required this.contents});

  @override
  NewCommentRequest toRequest() {
    return NewCommentRequest(
        name: name, targetManageId: targetManageId, targetEntityId: targetEntityId, contents: contents);
  }
}
