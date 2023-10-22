use std::collections::BTreeMap;

use dependencies_sync::chrono::Utc;
// use dependencies_sync::tokio::stream::StreamExt;
use dependencies_sync::futures::stream::StreamExt;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};
use dependencies_sync::mongodb::{bson, bson::doc, bson::Bson, bson::Document, Collection};
use serde::Deserialize;

use cash_result::*;
use database::get_cashmere_database;
use manage_define::general_field_ids::*;

/// 取得实体数量
pub async fn get_entity_count(
    collection_name: &String,
    filter_doc: Document,
) -> Result<u64, OperationResult> {
    if !database::collection_exists(collection_name).await {
        return Err(collection_not_exists("get_entity_by_id"));
    }

    let collection = match database::get_collection_by_id(collection_name).await {
        Some(c) => c,
        None => return Err(collection_not_exists("get_entity_by_id")),
    };

    let result = collection.count_documents(filter_doc, None).await;

    // let result = collection.find(filter_doc, None).await;

    match result {
        // Ok(r) => Ok(r.count().await as u64),
        Ok( r) => Ok(r),
        Err(_e) => Err(operation_failed(
            "get_entry_count",
            format!("获取实体数量失败{}", collection_name),
        )),
    }
}
