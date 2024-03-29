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

pub async fn get_entities(
    collection_name: &String,
    filter: &Option<Document>,
) -> Result<Vec<Document>, OperationResult> {
    let collection = match database::get_collection_by_id(collection_name).await {
        Some(c) => c,
        None => return Err(collection_not_exists("get_entities")),
    };

    let cursor = collection.find(filter.clone(), None).await;

    let mut result: Vec<Document> = Vec::new();
    match cursor {
        Ok(mut r) => {
            while let Some(d) = r.next().await {
                match d {
                    Ok(dc) => result.push(dc),
                    _ => continue,
                }
            }
            Ok(result)
        }
        Err(_e) => Err(operation_failed(
            "get_entities",
            format!("获取失败{}", filter.clone().unwrap_or_default()),
        )),
    }
}
