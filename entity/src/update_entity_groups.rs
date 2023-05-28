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

    crate::add_entity_to_array_field::add_entity_to_array_field(manage_id, query_doc, modify_doc, account_id).await
}
