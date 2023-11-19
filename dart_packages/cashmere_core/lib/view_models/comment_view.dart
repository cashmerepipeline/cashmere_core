import 'package:cashmere_core/ids/field_ids.dart';
import 'package:cashmere_core/ids/general_field_ids.dart';
import 'package:cashmere_core/view_models/entity_view_base.dart';
import 'package:cashmere_core/view_models/set_general_fields.dart';

class CommentView extends EntityViewBase {
  final int targetManage;
  final String targetEntity;
  final String contents;

  CommentView(
    String id, {
    required this.targetManage,
    required this.targetEntity,
    required this.contents,
  }) : super(id);

  factory CommentView.fromMap(Map<String, dynamic> map) {
    final entity = CommentView(
      map[ID_FIELD_ID.toString()],
      targetManage: map[COMMENTS_TARGET_MANAGE_FIELD_ID.toString()],
      targetEntity: map[COMMENTS_TARGET_ENTITY_FIELD_ID.toString()],
      contents: map[COMMENTS_CONTENTS_FIELD_ID.toString()],
    );
    final result = setGeneralFields(entity, map);

    return result;
  }
}
