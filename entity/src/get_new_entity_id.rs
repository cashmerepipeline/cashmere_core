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

use crate::utils::get_timestamp_update_doc;

/// 取得新连续id
pub async fn get_new_entity_id(manage_id: &str, account_id: &str) -> Option<i64> {
    let ids_collection = database::get_ids_collection().await;

    let update_doc = doc! {
     "$inc": {"id_count":1},
     "$set": { MODIFIER_FIELD_ID.to_string(): account_id},
     "$currentDate": {
         MODIFY_TIMESTAMP_FIELD_ID.to_string(): { "$type": "timestamp" }
      },
    };

    let result = ids_collection
        .find_one_and_update(
            doc! {
                "_id": manage_id
            },
            update_doc,
            Some(FindOneAndUpdateOptions::builder().upsert(true).build()),
        )
        .await;

    match result {
        Ok(r) => Some(r.unwrap().get_i32("id_count").unwrap() as i64),
        Err(_e) => None,
    }
}
