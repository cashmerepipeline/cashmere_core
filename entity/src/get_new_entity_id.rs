use std::collections::BTreeMap;

use dependencies_sync::chrono::Utc;
// use dependencies_sync::tokio::stream::StreamExt;
use dependencies_sync::futures::stream::StreamExt;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::log;
use dependencies_sync::mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};
use dependencies_sync::mongodb::{bson, bson::doc, bson::Bson, bson::Document, Collection};
use dependencies_sync::rust_i18n::{self, t};
use serde::Deserialize;

use cash_result::*;
use database::get_database;
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
        Ok(r) => {
            if let Some(r) = r {
                if let Ok(r) = r.get_i64("id_count") {
                    Some(r)
                } else {
                    log::error!("{}: {}", t!("提取新实体编号失败"), manage_id);
                    None
                }
            } else {
                log::error!("{}: {}", t!("新实体编号数据错误"), manage_id);
                None
            }
        }
        Err(_e) => {
            log::error!("{}: {}", t!("取得新实体编号失败"), manage_id);
            None
        }
    }
}
