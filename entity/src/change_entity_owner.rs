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

/// 变更条目所属人
pub async fn change_entity_owner(
    manage_id: &String,
    entity_id: &String,
    new_owner: &String,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    let _new_value = Bson::from(new_owner);

    let query_doc = doc! {"_id":entity_id.clone()};
    let modify_doc = doc! {OWNER_FIELD_ID.to_string():new_owner.clone()};

    let result = crate::update_entity_field::update_entity_field(manage_id, query_doc, modify_doc, account_id).await;

    result
}
