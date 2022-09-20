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

pub async fn entity_exists(manage_id: &String, query_doc: Document) -> bool {
    // 检查
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return false,
    };

    let result = collection.find_one(query_doc, None).await;
    print!("{:?}", result);

    match result {
        Ok(Some(_r)) => true,
        Ok(None) => false,
        Err(_e) => false,
    }
}
