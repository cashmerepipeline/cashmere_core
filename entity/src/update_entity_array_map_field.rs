use std::collections::BTreeMap;

use dependencies_sync::chrono::Utc;
// use dependencies_sync::tokio::stream::StreamExt;
use dependencies_sync::futures::stream::StreamExt;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::mongodb::{bson, bson::Bson, bson::doc, bson::Document, Collection};
use dependencies_sync::mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};
use serde::Deserialize;

use cash_result::*;
use database::get_cashmere_database;
use manage_define::general_field_ids::*;

use crate::utils::get_timestamp_update_doc;

/// 更新map元素
pub async fn update_entity_array_map_field(
    manage_id: &String,
    query_doc: Document,
    modify_doc: Document,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("upate_entity_array_field")),
    };


    let pipeline_docs = vec![
        doc! { "$set": modify_doc.clone()},
        doc! {"$set": {MODIFIER_FIELD_ID.to_string(): account_id.clone()}},
        get_timestamp_update_doc(),
    ];

    // 更新
    let result = collection
        .update_one(
            query_doc.clone(),
                pipeline_docs,
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
