use std::collections::BTreeMap;

use chrono::Utc;
// use tokio::stream::StreamExt;
use futures::stream::StreamExt;
use linked_hash_map::LinkedHashMap;
use mongodb::{bson, bson::Bson, bson::doc, bson::Document, Collection};
use mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};
use serde::Deserialize;

use cash_result::*;
use database::get_cashmere_database;
use manage_define::general_field_ids::*;

/// 插入实体, 返回插入的实体的_id
pub async fn insert_entity(
    manage_id: &String,
    entity_doc: &mut Document,
    account_id: &String,
    group_id: &String,
) -> Result<String, OperationResult> {
    // 检查
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("insert_entity")),
    };

    if entity_doc.contains_key(&ID_FIELD_ID.to_string()) {
        let v = entity_doc
            .get_str(&ID_FIELD_ID.to_string())
            .unwrap()
            .to_string();
        entity_doc.insert("_id", v);
    }

    // 创建标记
    entity_doc.insert(CREATOR_FIELD_ID.to_string(), account_id.clone());
    entity_doc.insert(MODIFIER_FIELD_ID.to_string(), account_id.clone());
    entity_doc.insert(OWNER_FIELD_ID.to_string(), account_id.clone());
    entity_doc.insert(GROUPS_FIELD_ID.to_string(), vec![group_id.clone()]);
    entity_doc.insert(
        MODIFY_TIMESTAMP_FIELD_ID.to_string(),
        Utc::now().timestamp(),
    );
    entity_doc.insert(
        CREATE_TIMESTAMP_FIELD_ID.to_string(),
        Utc::now().timestamp(),
    );

    // 插入
    let result = collection.insert_one(entity_doc.clone(), None).await;

    // 结果
    match result {
        Ok(r) => Ok(r.inserted_id.as_str().unwrap().to_string()),
        Err(_e) => Err(operation_failed(
            "insert_entity",
            format!("插入实体失败 {}", entity_doc),
        )),
    }
}
