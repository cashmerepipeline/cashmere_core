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

/// 取得实体编号
pub fn get_entity_id(entity_doc: &Document) -> Option<String> {
    let id = match get_entity_field::get_entity_field(entity_doc, "_id") {
        Some(b) => b,
        None => return None,
    };

    id.as_str().map(|r| r.to_string())
}
