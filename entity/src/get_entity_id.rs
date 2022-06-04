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

use crate::get_entity_field;

/// 取得实体编号
pub fn get_entity_id(entity_doc: &Document) -> Option<String> {
    let id = match get_entity_field::get_entity_field(entity_doc, "_id") {
        Some(b) => b,
        None => return None,
    };

    id.as_str().map(|r| r.to_string())
}
