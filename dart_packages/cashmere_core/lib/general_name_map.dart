import 'ids/general_field_ids.dart';

// 通用属性名字映射
final Map<int, Map<String, String>> generalSchemaFieldsNameMap = {
// 编号
  ID_FIELD_ID: {"zh": "编号", "en": "id"},
// 名字
  NAME_MAP_FIELD_ID: {"zh": "实体名", "en": "name"},
// 创建人
  CREATOR_FIELD_ID: {"zh": "创建人", "en": "creator"},
// 创建时间
  CREATE_TIMESTAMP_FIELD_ID: {"zh": "创建时间戳", "en": "create_timestamp"},
// 修改人
  MODIFIER_FIELD_ID: {"zh": "修改人", "en": "modifier"},
// 修改时间
  MODIFY_TIMESTAMP_FIELD_ID: {"zh": "修改时间戳", "en": "modify_timestamp"},
// 主人
  OWNER_FIELD_ID: {"zh": "主人", "en": "owner"},
// 组
  GROUPS_FIELD_ID: {"zh": "组", "en": "group"},
// 相关数据
  SPECS_FIELD_ID: {"zh": "规格", "en": "datas"},
// 注释和评论
  COMMENTS_FIELD_ID: {"zh": "评论", "en": "id"},
// 删除标记
  REMOVED_FIELD_ID: {"zh": "删除标记", "en": "removed"}
};
