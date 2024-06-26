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

use crate::get_entity_field;

/// 根据服务器的语言设置取得实体名
pub fn get_entity_name(entity_doc: &Document) -> Option<String> {
    let lang_code = configs::get_language_code();

    let name = match get_entity_field::get_entity_field(entity_doc, NAME_MAP_FIELD_ID.to_string()) {
        Some(b) => b,
        None => return None,
    };

    if let Some(n) = name.as_document() {
        if n.contains_key(lang_code.as_str()) {
            Some(n.get(lang_code.as_str()).unwrap().to_string())
        } else {
            // 如果没有，使用第一个名字

            let mut name_b: Bson = Bson::String("".to_string());
            // for x in n {
            //     name_b = x.1.clone();
            //     break;
            // }
            if let Some(first_name) = n.into_iter().next() {
                name_b = first_name.1.clone();
            }
            Some(name_b.as_str().unwrap().to_string())
        }
    } else {
        None
    }
}
