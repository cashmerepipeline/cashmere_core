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
