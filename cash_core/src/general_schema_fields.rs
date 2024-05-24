use std::sync::Arc;

use dependencies_sync::indexmap::IndexMap;

use crate::SchemaField;

use manage_define::general_field_ids::*;

static mut GENERAL_PROPERTY_FIELDS: Option<Arc<Vec<SchemaField>>> = None;

pub fn general_schema_fields() -> &'static Vec<SchemaField> {
    unsafe {
        if GENERAL_PROPERTY_FIELDS.is_some() {
            GENERAL_PROPERTY_FIELDS.as_ref().unwrap()
        } else {
            let fields = _general_property_fields();
            GENERAL_PROPERTY_FIELDS.get_or_insert(Arc::new(fields));
            GENERAL_PROPERTY_FIELDS.as_ref().unwrap()
        }
    }
}

// 直接构建列表
fn _general_property_fields() -> Vec<SchemaField> {
    let mut id_name_map = IndexMap::new();
    id_name_map.insert("zh".to_string(), "编号".to_string());
    id_name_map.insert("en".to_string(), "id".to_string());
    let id_field = SchemaField {
        id: ID_FIELD_ID,
        name_map: id_name_map,
        data_type: "String".to_string(),
        removed: false,
    };

    let mut name_name_map = IndexMap::new();
    name_name_map.insert("zh".to_string(), "实体名".to_string());
    name_name_map.insert("en".to_string(), "name".to_string());
    let name_map_field = SchemaField {
        id: NAME_MAP_FIELD_ID,
        name_map: name_name_map,
        data_type: "Document".to_string(),
        removed: false,
    };

    let mut creator_name_map = IndexMap::new();
    creator_name_map.insert("zh".to_string(), "创建人".to_string());
    creator_name_map.insert("en".to_string(), "creator".to_string());
    let creator_field = SchemaField {
        id: CREATOR_FIELD_ID,
        name_map: creator_name_map,
        data_type: "String".to_string(),
        removed: false,
    };

    let mut create_timestamp_name_map = IndexMap::new();
    create_timestamp_name_map.insert("zh".to_string(), "创建时间戳".to_string());
    create_timestamp_name_map.insert("en".to_string(), "create_timestamp".to_string());
    let create_timestamp_field = SchemaField {
        id: CREATE_TIMESTAMP_FIELD_ID,
        name_map: create_timestamp_name_map,
        data_type: "Timestamp".to_string(),
        removed: false,
    };
    let mut modifier_name_map = IndexMap::new();
    modifier_name_map.insert("zh".to_string(), "修改人".to_string());
    modifier_name_map.insert("en".to_string(), "modifier".to_string());
    let modifier_field = SchemaField {
        id: MODIFIER_FIELD_ID,
        name_map: modifier_name_map,
        data_type: "Array".to_string(),
        removed: false,
    };
    let mut modify_timestamp_name_map = IndexMap::new();
    modify_timestamp_name_map.insert("zh".to_string(), "修改时间戳".to_string());
    modify_timestamp_name_map.insert("en".to_string(), "modify_timestamp".to_string());
    let modify_timestamp_field = SchemaField {
        id: MODIFY_TIMESTAMP_FIELD_ID,
        name_map: modify_timestamp_name_map,
        data_type: "Timestamp".to_string(),
        removed: false,
    };

    let mut owner_name_map = IndexMap::new();
    owner_name_map.insert("zh".to_string(), "主人".to_string());
    owner_name_map.insert("en".to_string(), "owner".to_string());
    let owner_field = SchemaField {
        id: OWNER_FIELD_ID,
        name_map: owner_name_map,
        data_type: "String".to_string(),
        removed: false,
    };

    let mut group_name_map = IndexMap::new();
    group_name_map.insert("zh".to_string(), "组".to_string());
    group_name_map.insert("en".to_string(), "group".to_string());
    let group_field = SchemaField {
        id: GROUPS_FIELD_ID,
        name_map: group_name_map,
        data_type: "Array".to_string(),
        removed: false,
    };

    let mut comments_name_map = IndexMap::new();
    comments_name_map.insert("zh".to_string(), "评论".to_string());
    comments_name_map.insert("en".to_string(), "comments".to_string());
    let comments_field = SchemaField {
        id: COMMENTS_FIELD_ID,
        name_map: comments_name_map,
        data_type: "Array".to_string(),
        removed: false,
    };

    let mut tags_name_map = IndexMap::new();
    tags_name_map.insert("zh".to_string(), "标签".to_string());
    tags_name_map.insert("en".to_string(), "tags".to_string());
    let tags_field = SchemaField {
        id: TAGS_FIELD_ID,
        name_map: tags_name_map,
        data_type: "Array".to_string(),
        removed: false,
    };

    let mut description_name_map = IndexMap::new();
    description_name_map.insert("zh".to_string(), "注释".to_string());
    description_name_map.insert("en".to_string(), "description".to_string());
    let description_field = SchemaField {
        id: DESCRIPTION_FIELD_ID,
        name_map: description_name_map,
        data_type: "String".to_string(),
        removed: false,
    };

    let mut entity_removed_name_map = IndexMap::new();
    entity_removed_name_map.insert("zh".to_string(), "已移除".to_string());
    entity_removed_name_map.insert("en".to_string(), "name".to_string());
    let entity_removed_field = SchemaField {
        id: REMOVED_FIELD_ID,
        name_map: entity_removed_name_map,
        data_type: "Boolean".to_string(),
        removed: false,
    };

    vec![
        id_field,
        name_map_field,
        owner_field,
        group_field,
        description_field,
        tags_field,
        comments_field,
        creator_field,
        create_timestamp_field,
        modifier_field,
        modify_timestamp_field,
        entity_removed_field,
    ]
}
