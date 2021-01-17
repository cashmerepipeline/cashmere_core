/*
Author: 闫刚 (yes7rose@sina.com)
core_ids.rs (c) 2020
Desc: 核心管理编号
Created:  2020-11-12T09:37:07.992Z
Modified: !date!
*/

pub mod view_rules;
pub mod results;
pub mod data;
pub mod field;
pub mod macros;
pub mod message;
pub mod ids;

use serde::{Serialize, Deserialize};

use field::PropertyField;
use crate::results::OperationResult;
use bson::Document;
use linked_hash_map::LinkedHashMap;

/// 管理实体
#[derive(Debug, Serialize, Deserialize)]
pub struct Manage {
    pub _id: String,
    pub id: i32,
    pub name: LinkedHashMap<String, String>,
    pub creator: String,
    pub create_timestamp: i64,
    pub modifier: String,
    pub modify_timestamp: i64,
    pub owner: String,
    pub groups: Vec<String>,

    pub schema: Vec<PropertyField>,
}

pub fn manage_from_document(manage_doc: Document) -> Result<Manage, OperationResult> {
    let _id = manage_doc.get_str("_id").unwrap();
    let id: i32 = manage_doc.get_str(&field::ids::ID_FIELD_ID.to_string()).unwrap().parse().unwrap();
    let name = bson::from_document(manage_doc.get_document(&field::ids::NAME_FIELD_ID.to_string()).unwrap().clone()).unwrap();
    let creator = manage_doc.get_str(&field::ids::CREATOR_FIELD_ID.to_string()).unwrap();
    let create_timestamp = manage_doc.get_i64(&field::ids::CREATE_TIMESTAMP_FIELD_ID.to_string()).unwrap();
    let modifier = manage_doc.get_str(&field::ids::MODIFIER_FIELD_ID.to_string()).unwrap();
    let modify_timestamp = manage_doc.get_i64(&field::ids::MODIFY_TIMESTAMP_FIELD_ID.to_string()).unwrap();
    let owner = manage_doc.get_str(&field::ids::OWNER_FIELD_ID.to_string()).unwrap();
    let groups: Vec<String> = manage_doc.get_array(&field::ids::GROUPS_FIELD_ID.to_string()).unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect();
    let schema = manage_doc.get_array(&field::ids::MANAGES_SCHEMA_FIELD_ID.to_string()).unwrap().iter().map(|x| {
        let field: PropertyField = PropertyField::from_bson(x.as_document().unwrap());
        field
    }).collect();

    Ok(Manage {
        _id: _id.to_string(),
        id: id,
        name: name,
        creator: creator.to_string(),
        create_timestamp,
        modifier: modifier.to_string(),
        modify_timestamp,
        owner: owner.to_string(),
        groups,
        schema: schema,
    }
    )
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
