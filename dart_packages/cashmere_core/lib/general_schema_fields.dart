import 'package:cashmere_core/protocols/manage_schema.pb.dart';

import 'ids/general_field_ids.dart';

final List<SchemaField> generalSchemaFields = [
// 编号
  SchemaField(id: ID_FIELD_ID, nameMap: {"zh": "编号", "en": "id"}, dataType: "String", removed: false),
// 名字
  SchemaField(id: NAME_MAP_FIELD_ID, nameMap: {"zh": "实体名", "en": "name"}, dataType: "String", removed: false),
// 创建人
  SchemaField(id: CREATOR_FIELD_ID, nameMap: {"zh": "创建人", "en": "creator"}, dataType: "String", removed: false),
// 创建时间
  SchemaField(
      id: CREATE_TIMESTAMP_FIELD_ID,
      nameMap: {"zh": "创建时间戳", "en": "create_timestamp"},
      dataType: "Timestamp",
      removed: false),
// 修改人
  SchemaField(id: MODIFIER_FIELD_ID, nameMap: {"zh": "修改人", "en": "modifier"}, dataType: "String", removed: false),
// 修改时间
  SchemaField(
      id: MODIFY_TIMESTAMP_FIELD_ID,
      nameMap: {"zh": "修改时间戳", "en": "modify_timestamp"},
      dataType: "Timestamp",
      removed: false),
// 主人
  SchemaField(id: OWNER_FIELD_ID, nameMap: {"zh": "主人", "en": "owner"}, dataType: "String", removed: false),
// 组
  SchemaField(id: GROUPS_FIELD_ID, nameMap: {"zh": "组", "en": "group"}, dataType: "Array", removed: false),
// 相关数据
  SchemaField(id: SPECS_FIELD_ID, nameMap: {"zh": "规格", "en": "datas"}, dataType: "Array", removed: false),
// 注释和评论
  SchemaField(id: COMMENTS_FIELD_ID, nameMap: {"zh": "评论", "en": "id"}, dataType: "String", removed: false),
// 删除标记
  SchemaField(id: REMOVED_FIELD_ID, nameMap: {"zh": "删除标记", "en": "removed"}, dataType: "Boolean", removed: false)
];
