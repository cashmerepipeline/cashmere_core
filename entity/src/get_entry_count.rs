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

/// 取得实体数量
pub async fn get_entry_count(
    collection_name: &String,
    filter_doc: Document
) -> Result<u64, OperationResult> {
    if !database::collection_exists(collection_name).await {
        return Err(collection_not_exists("get_entity_by_id"));
    }

    let collection = match database::get_collection_by_id(collection_name).await {
        Some(c) => c,
        None => return Err(collection_not_exists("get_entity_by_id")),
    };

    let mut pipeline: Vec<Document> = vec![];
    pipeline.push(doc!{"$match": filter_doc});
    pipeline.push(doc!{"$count":"count"});
    let result = collection.aggregate(pipeline, None).await;

    // let result = collection.find(filter_doc, None).await;

    match result {
        // Ok(r) => Ok(r.count().await as u64),
        Ok(mut r) => {
            let d = r.next().await.unwrap().unwrap();
            let count = d.get_i32("count").unwrap();
            Ok(count as u64)
        }
        Err(_e) => Err(operation_failed(
            "get_entity_count",
            format!("获取失败{}", collection_name),
        )),
    }
}