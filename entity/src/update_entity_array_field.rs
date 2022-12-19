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

/// 更新元素
pub async fn update_entity_array_field(
    manage_id: &String,
    query_doc: Document,
    mut modify_doc: Document,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("upate_entity_array_field")),
    };

    modify_doc.insert(MODIFIER_FIELD_ID.to_string(), account_id.clone());
    modify_doc.insert(MODIFY_TIMESTAMP_FIELD_ID.to_string(), Utc::now().timestamp());

    // 更新
    let result = collection
        .update_one(
            query_doc.clone(),
            doc! {
                "$set": modify_doc,
            },
            None,
        )
        .await;

    // 结果
    match result {
        Ok(r) => match r.modified_count == 1 {
            true => Ok(operation_succeed("succeed")),
            false => Err(operation_failed(
                "update_entity_array_field",
                format!("更新了多个实体{}", query_doc),
            )),
        },
        Err(_e) => Err(operation_failed(
            "update_entity_array_field",
            format!("更新操作失败{}", query_doc),
        )),
    }
}
