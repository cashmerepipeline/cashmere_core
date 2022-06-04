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

/// 取得实体数据 所属人
pub fn get_entity_owner(entity_doc: &Document) -> Option<String> {
    match entity_doc.get_str("owner") {
        Ok(r) => Some(r.to_string()),
        Err(_e) => None,
    }
}
