import 'package:cashmere_core/core/name_map_from_map.dart';
import 'package:cashmere_core/ids/general_field_ids.dart';
import 'package:cashmere_core/view_models/entity_view_base.dart';

T setGeneralFields<T extends EntityViewBase>(T entity, Map<String, dynamic> map) {
  map[NAME_MAP_FIELD_ID.toString()] != null ? entity.nameMap = nameMapFromMap(map[NAME_MAP_FIELD_ID.toString()])! : null;
  map[CREATOR_FIELD_ID.toString()] != null ? entity.creator = map[CREATOR_FIELD_ID.toString()] : null;
  map[CREATE_TIMESTAMP_FIELD_ID.toString()] != null
      ? entity.createTimestamp = map[CREATE_TIMESTAMP_FIELD_ID.toString()]
      : null;
  map[MODIFIER_FIELD_ID.toString()] != null ? entity.modifier = map[MODIFIER_FIELD_ID.toString()] : null;
  map[MODIFY_TIMESTAMP_FIELD_ID.toString()] != null
      ? entity.modifyTimestamp = map[MODIFY_TIMESTAMP_FIELD_ID.toString()]
      : null;
  map[OWNER_FIELD_ID.toString()] != null ? entity.owner = map[OWNER_FIELD_ID.toString()] : null;

  if (map[GROUPS_FIELD_ID.toString()] != null) {
    final groups = <String>[];
    (map[GROUPS_FIELD_ID.toString()]).forEach((group) => groups.add(group as String));
    entity.groups = groups;
  }

  if (map[SPECS_FIELD_ID.toString()] != null) {
    final specs = <String>[];
    (map[SPECS_FIELD_ID.toString()]).forEach((spec) => specs.add(spec as String));
    entity.specs = specs;
  }

  if (map[COMMENTS_FIELD_ID.toString()] != null) {
    final comments = <String>[];
    (map[COMMENTS_FIELD_ID.toString()]).forEach((comment) => comments.add(comment as String));
    entity.comments = comments;
  }

  map[REMOVED_FIELD_ID.toString()] != null ? entity.removed = map[REMOVED_FIELD_ID.toString()] : null;

  return entity;
}
