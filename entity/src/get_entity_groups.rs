use std::collections::BTreeMap;

use dependencies_sync::chrono::Utc;
// use dependencies_sync::tokio::stream::StreamExt;
use dependencies_sync::futures::stream::StreamExt;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::mongodb::{bson, bson::Bson, bson::doc, bson::Document, Collection};
use dependencies_sync::mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};
use serde::Deserialize;

use cash_result::*;
use database::get_database;
use manage_define::general_field_ids::*;

/// 取得实体数据 所属组
pub fn get_entity_groups(entity_doc: &Document) -> Option<Vec<String>> {
    match entity_doc.get_array("groups") {
        Ok(r) => Some(r.iter().map(|x| x.as_str().unwrap().to_string()).collect()),
        Err(_e) => None,
    }
}
