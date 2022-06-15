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

/// 取得 实体
pub async fn get_entity_by_id(
    collection_name: &String,
    id: &String,
) -> Result<Document, OperationResult> {
    if !database::collection_exists(collection_name).await {
        return Err(collection_not_exists("get_entity_by_id"));
    }

    let collection = match database::get_collection_by_id(collection_name).await {
        Some(c) => c,
        None => return Err(collection_not_exists("get_entity_by_id")),
    };

    let result = collection
        .find_one(
            doc! {
                "_id": id
            },
            None,
        )
        .await;

    match result {
        Ok(r) => match r {
            Some(d) => Ok(d),
            None => Err(operation_failed(
                "get_entity_by_id",
                format!("无数据 {}", id),
            )),
        },
        Err(_e) => Err(operation_failed(
            "get_entity_by_id",
            format!("获取失败{}", id),
        )),
    }
}