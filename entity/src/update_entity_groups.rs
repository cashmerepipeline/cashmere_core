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

/// 变更条目所属组
pub async fn update_entity_groups(
    manage_id: &String,
    entity_id: &String,
    new_groups: &Vec<String>,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let _collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("update_entity_groups")),
    };

    let query_doc = doc! {
        "_id": entity_id
    };

    let modify_doc = doc! {
        GROUPS_FIELD_ID.to_string(): { "$each":new_groups}
    };

    crate::push_entity_array_field::push_entity_array_field(manage_id, query_doc, modify_doc, account_id).await
}
