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
use crate::utils::add_modify_update_fields;
use crate::update_entity_field::update_entity_field;

/// 变更条目所属人
pub async fn change_entity_owner(
    manage_id: &str,
    entity_id: &str,
    new_owner: &str,
    account_id: &str,
) -> Result<OperationResult, OperationResult> {
    let _new_value = Bson::from(new_owner);

    let query_doc = doc! {ID_FIELD_ID.to_string():entity_id};
    let mut modify_doc = doc! {OWNER_FIELD_ID.to_string():new_owner};

    update_entity_field(
        manage_id,
        query_doc,
        &mut modify_doc,
        account_id,
    )
    .await
}
