use std::sync::Arc;

use dependencies_sync::linked_hash_map::LinkedHashMap;

use property_field::{FieldDataType, PropertyField};

use crate::general_field_ids::*;

static mut GENERAL_PROPERTY_FIELDS: Option<Arc<Vec<PropertyField>>> = None;

pub fn general_property_fields() -> &'static Vec<PropertyField> {
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
fn _general_property_fields() -> Vec<PropertyField> {
    let mut id_name_map = LinkedHashMap::new();
    id_name_map.insert("zh".to_string(), "编号".to_string());
    id_name_map.insert("en".to_string(), "id".to_string());
    let id_field = PropertyField {
        id: ID_FIELD_ID,
        name_map: id_name_map,
        data_type: FieldDataType::String,
        removed: false,
    };

    let mut name_name_map = LinkedHashMap::new();
    name_name_map.insert("zh".to_string(), "实体名".to_string());
    name_name_map.insert("en".to_string(), "name".to_string());
    let name_map_field = PropertyField {
        id: NAME_MAP_FIELD_ID,
        name_map: name_name_map,
        data_type: FieldDataType::List,
        removed: false,
    };

    let mut creator_name_map = LinkedHashMap::new();
    creator_name_map.insert("zh".to_string(), "创建人".to_string());
    creator_name_map.insert("en".to_string(), "creator".to_string());
    let creator_field = PropertyField {
        id: CREATOR_FIELD_ID,
        name_map: creator_name_map,
        data_type: FieldDataType::String,
        removed: false,
    };

    let mut create_timestamp_name_map = LinkedHashMap::new();
    create_timestamp_name_map.insert("zh".to_string(), "创建时间戳".to_string());
    create_timestamp_name_map.insert("en".to_string(), "create_timestamp".to_string());
    let create_timestamp_field = PropertyField {
        id: CREATE_TIMESTAMP_FIELD_ID,
        name_map: create_timestamp_name_map,
        data_type: FieldDataType::Int64,
        removed: false,
    };
    let mut modifier_name_map = LinkedHashMap::new();
    modifier_name_map.insert("zh".to_string(), "修改人".to_string());
    modifier_name_map.insert("en".to_string(), "modifier".to_string());
    let modifier_field = PropertyField {
        id: MODIFIER_FIELD_ID,
        name_map: modifier_name_map,
        data_type: FieldDataType::String,
        removed: false,
    };
    let mut modify_timestamp_name_map = LinkedHashMap::new();
    modify_timestamp_name_map.insert("zh".to_string(), "修改时间戳".to_string());
    modify_timestamp_name_map.insert("en".to_string(), "modify_timestamp".to_string());
    let modify_timestamp_field = PropertyField {
        id: MODIFY_TIMESTAMP_FIELD_ID,
        name_map: modify_timestamp_name_map,
        data_type: FieldDataType::Int64,
        removed: false,
    };

    let mut owner_name_map = LinkedHashMap::new();
    owner_name_map.insert("zh".to_string(), "主人".to_string());
    owner_name_map.insert("en".to_string(), "owner".to_string());
    let owner_field = PropertyField {
        id: OWNER_FIELD_ID,
        name_map: owner_name_map,
        data_type: FieldDataType::String,
        removed: false,
    };

    let mut group_name_map = LinkedHashMap::new();
    group_name_map.insert("zh".to_string(), "组".to_string());
    group_name_map.insert("en".to_string(), "group".to_string());
    let group_field = PropertyField {
        id: GROUPS_FIELD_ID,
        name_map: group_name_map,
        data_type: FieldDataType::List,
        removed: false,
    };

    let mut datas_name_map = LinkedHashMap::new();
    datas_name_map.insert("zh".to_string(), "数据".to_string());
    datas_name_map.insert("en".to_string(), "datas".to_string());
    let datas_field = PropertyField {
        id: DATAS_FIELD_ID,
        name_map: datas_name_map,
        data_type: FieldDataType::List,
        removed: false,
    };

    let mut datas_removed_name_map = LinkedHashMap::new();
    datas_removed_name_map.insert("zh".to_string(), "实体名".to_string());
    datas_removed_name_map.insert("en".to_string(), "name".to_string());
    let datas_removed_field = PropertyField {
        id: DATAS_REMOVED_FIELD_ID,
        name_map: datas_removed_name_map,
        data_type: FieldDataType::List,
        removed: false,
    };

    let mut comments_name_map = LinkedHashMap::new();
    comments_name_map.insert("zh".to_string(), "注释".to_string());
    comments_name_map.insert("en".to_string(), "name".to_string());
    let comments_field = PropertyField {
        id: COMMENTS_FIELD_ID,
        name_map: comments_name_map,
        data_type: FieldDataType::List,
        removed: false,
    };

    let mut entity_removed_name_map = LinkedHashMap::new();
    entity_removed_name_map.insert("zh".to_string(), "已移除".to_string());
    entity_removed_name_map.insert("en".to_string(), "name".to_string());
    let entity_removed_field = PropertyField {
        id: ENTITY_REMOVED_FIELD_ID,
        name_map: entity_removed_name_map,
        data_type: FieldDataType::Bool,
        removed: false,
    };

    vec![id_field, name_map_field, owner_field, group_field, datas_field, datas_removed_field, comments_field, creator_field, create_timestamp_field, modifier_field, modify_timestamp_field, entity_removed_field]
}
