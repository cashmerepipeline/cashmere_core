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

/// 取得新连续id
pub async fn get_new_entity_id(manage_id: &String, account_id: &String) -> Option<i64> {
    let ids_collection = database::get_ids_collection().await;
    let result = ids_collection
        .find_one_and_update(
            doc! {
                "_id": manage_id.clone()
            },
            doc! {
                "$inc": {"id_count":1},
                "$set": {
                    MODIFIER_FIELD_ID.to_string(): account_id.clone(),
                    MODIFY_TIMESTAMP_FIELD_ID.to_string(): Utc::now().timestamp()
                }
            },
            Some(FindOneAndUpdateOptions::builder().upsert(true).build()),
        )
        .await;

    match result {
        Ok(r) => Some(r.unwrap().get_i32("id_count").unwrap() as i64),
        Err(_e) => None,
    }
}
