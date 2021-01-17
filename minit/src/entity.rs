/*
Author: 闫刚 (yes7rose@sina.com)
entity.rs (c) 2020
Desc: 同步实体操作
Created:  2020-11-23T11:51:44.495Z
Modified: !date!
*/

use cash_core::results::*;
use cash_core::ids;
use mongodb::{bson::Document};
use mongodb::sync::{Collection};
use chrono::{Utc};

/// 插入实体
pub fn insert_entity(
    collection: &Collection,
    entity_doc: &mut Document,
    account_id: &String,
    group_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 创建标记
    entity_doc.insert(ids::CREATOR_FIELD_ID.to_string(), account_id.clone());
    entity_doc.insert(ids::MODIFIER_FIELD_ID.to_string(), account_id.clone());
    entity_doc.insert(ids::OWNER_FIELD_ID.to_string(), account_id.clone());
    entity_doc.insert(ids::GROUPS_FIELD_ID.to_string(), vec![group_id.clone()]);
    entity_doc.insert(ids::CREATE_TIMESTAMP_FIELD_ID.to_string(), Utc::now().timestamp());
    entity_doc.insert(ids::MODIFIER_FIELD_ID.to_string(), Utc::now().timestamp());

    // 插入
    let result = collection.insert_one(entity_doc.clone(), None);

    // 结果
    match result {
        _ => Ok(operation_succeed("ok")),
        Err(_e) => Err(operation_failed(
            "insert_entity",
            format!("插入实体失败 {}", entity_doc),
        )),
    }
}