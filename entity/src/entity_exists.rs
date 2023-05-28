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

pub async fn entity_exists(manage_id: &String, query_doc: &Document) -> bool {
    // 检查
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return false,
    };

    let result = collection.find_one(query_doc.clone(), None).await;

    match result {
        Ok(Some(_r)) => true,
        Ok(None) => false,
        Err(_e) => false,
    }
}
