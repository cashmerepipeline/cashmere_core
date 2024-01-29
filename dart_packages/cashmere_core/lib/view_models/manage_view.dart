import 'package:bson/bson.dart';
import 'package:cashmere_core/core/name_map_from_map.dart';
import 'package:cashmere_core/ids/general_field_ids.dart';

/// 管理视图
class ManageView {
  final String id;
  late final Map<String, String> nameMap;
  late final String creator;
  late final Timestamp createTimestamp;
  late final String modifier;
  late final Timestamp modifyTimestamp;
  late final String owner;
  late final List<String> groups;
  late final List<String>? description;

  ManageView(this.id);

  factory ManageView.fromMap(Map<String, dynamic> map) {
    ManageView v = ManageView(map[ID_FIELD_ID.toString()]);

    map[NAME_MAP_FIELD_ID.toString()] != null ? v.nameMap = nameMapFromMap(map[NAME_MAP_FIELD_ID.toString()]) : null;
    map[CREATOR_FIELD_ID.toString()] != null ? v.creator = map[CREATOR_FIELD_ID.toString()] : null;
    map[CREATE_TIMESTAMP_FIELD_ID.toString()] != null
        ? v.createTimestamp = map[CREATE_TIMESTAMP_FIELD_ID.toString()]
        : null;
    map[MODIFIER_FIELD_ID.toString()] != null ? v.modifier = map[MODIFIER_FIELD_ID.toString()] : null;
    map[MODIFY_TIMESTAMP_FIELD_ID.toString()] != null
        ? v.modifyTimestamp = map[MODIFY_TIMESTAMP_FIELD_ID.toString()]
        : null;
    map[OWNER_FIELD_ID.toString()] != null ? v.owner = map[OWNER_FIELD_ID.toString()] : null;

    if (map[GROUPS_FIELD_ID.toString()] != null) {
      final groups = <String>[];
      (map[GROUPS_FIELD_ID.toString()]).forEach((group) => groups.add(group as String));
      v.groups = groups;
    }

    map[DESCRIPTION_FIELD_ID.toString()] != null ? v.description = map[DESCRIPTION_FIELD_ID.toString()] : "";

    return v;
  }
}
