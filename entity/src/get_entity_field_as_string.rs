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

use crate::get_entity_field;

/// 取得实体属性为String
pub fn get_entity_field_as_string(
    entity_doc: &Document,
    field_name: impl Into<String>,
) -> Option<String> {
    let b = match get_entity_field::get_entity_field(entity_doc, field_name) {
        Some(r) => r,
        None => return None,
    };

    b.as_str().map(|result| result.to_string())
}
