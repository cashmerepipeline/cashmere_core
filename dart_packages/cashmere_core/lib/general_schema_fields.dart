import 'package:cashmere_core/manage/schema_field.dart';

import 'ids/general_field_ids.dart';

final List<SchemaField> generalSchemaFields = [
// 编号
  SchemaField(ID_FIELD_ID, {"zh": "编号", "en": "id"}, "String", false),
// 名字
  SchemaField(NAME_MAP_FIELD_ID, {"zh": "实体名", "en": "name"}, "String", false),
// 创建人
  SchemaField(CREATOR_FIELD_ID, {"zh": "创建人", "en": "creator"}, "String", false),
// 创建时间
  SchemaField(CREATE_TIMESTAMP_FIELD_ID, {"zh": "创建时间戳", "en": "create_timestamp"}, "String", false),
// 修改人
  SchemaField(MODIFIER_FIELD_ID, {"zh": "修改人", "en": "modifier"}, "String", false),
// 修改时间
  SchemaField(MODIFY_TIMESTAMP_FIELD_ID, {"zh": "修改时间戳", "en": "modify_timestamp"}, "String", false),
// 主人
  SchemaField(OWNER_FIELD_ID, {"zh": "主人", "en": "owner"}, "String", false),
// 组
  SchemaField(GROUPS_FIELD_ID, {"zh": "组", "en": "group"}, "String", false),
// 相关数据
  SchemaField(SPECS_FIELD_ID, {"zh": "规格", "en": "datas"}, "String", false),
// 注释和评论
  SchemaField(COMMENTS_FIELD_ID, {"zh": "评论", "en": "id"}, "String", false),
// 删除标记
  SchemaField(REMOVED_FIELD_ID, {"zh": "删除标记", "en": "removed"}, "Bool", false)
];
