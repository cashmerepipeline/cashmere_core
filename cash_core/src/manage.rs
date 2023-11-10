use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use serde::{Deserialize, Serialize};

use dependencies_sync::bson::{self, Document, Timestamp};
use dependencies_sync::linked_hash_map::LinkedHashMap;

use cash_result::OperationResult;
use manage_define::general_property_fields::general_property_fields;
use property_field::PropertyField;

/// 管理实体
#[derive(Debug, Serialize, Deserialize)]
pub struct Manage {
    pub object_id: String,
    pub id: i32,
    pub name_map: LinkedHashMap<String, String>,
    pub creator: String,
    pub create_timestamp: Timestamp,
    pub modifier: String,
    pub modify_timestamp: Timestamp,

    pub owner: String,
    pub groups: Vec<String>,

    pub schema: Vec<PropertyField>,

    // 能被搜索
    pub is_searchable: bool,
    // 需要登录才能访问管理的实体
    pub login_required: bool,
}

/// bson文档-->管理实体
pub fn manage_from_document(manage_doc: Document) -> Result<Manage, OperationResult> {
    let object_id = manage_doc.get_str("_id").unwrap();
    let id: i32 = manage_doc
        .get_str(&ID_FIELD_ID.to_string())
        .unwrap()
        .parse()
        .unwrap();

    let name_map = bson::from_document(
        manage_doc
            .get_document(&NAME_MAP_FIELD_ID.to_string())
            .unwrap()
            .clone(),
    )
    .unwrap();

    let creator = manage_doc.get_str(&CREATOR_FIELD_ID.to_string()).unwrap();
    let create_timestamp = manage_doc
        .get_timestamp(CREATE_TIMESTAMP_FIELD_ID.to_string())
        .unwrap();

    let modifier = manage_doc.get_str(&MODIFIER_FIELD_ID.to_string()).unwrap();
    let modify_timestamp = manage_doc
        .get_timestamp(MODIFY_TIMESTAMP_FIELD_ID.to_string())
        .unwrap();

    let owner = manage_doc.get_str(&OWNER_FIELD_ID.to_string()).unwrap();
    let groups: Vec<String> = manage_doc
        .get_array(&GROUPS_FIELD_ID.to_string())
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().to_string())
        .collect();

    let mut schema = general_property_fields().clone();
    manage_doc
        .get_array(&MANAGES_SCHEMA_FIELD_ID.to_string())
        .unwrap()
        .iter()
        .for_each(|x| {
            let field: PropertyField = PropertyField::from_document(x.as_document().unwrap());
            schema.push(field);
        });

    let is_searchable = manage_doc
        .get_bool(&IS_SEARCHABLE_FIELD_ID.to_string())
        .unwrap();

    let login_required = manage_doc
        .get_bool(&LOGIN_REQUIRED_FIELD_ID.to_string())
        .unwrap();

    Ok(Manage {
        object_id: object_id.to_string(),
        id,
        name_map,
        creator: creator.to_string(),
        create_timestamp,
        modifier: modifier.to_string(),
        modify_timestamp,
        owner: owner.to_string(),
        groups,
        schema,
        is_searchable,
        login_required,
    })
}
