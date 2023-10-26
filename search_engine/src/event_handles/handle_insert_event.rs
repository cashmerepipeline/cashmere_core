use cash_result::operation_failed;
use dependencies_sync::bson::{self, extjson, Bson};
use dependencies_sync::chrono::Utc;
use dependencies_sync::log;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::serde_json::{self, json};
use dependencies_sync::tantivy::time::Instant;
use dependencies_sync::tantivy::{self, doc, schema::*, TantivyDocument};
use manage_define::general_field_ids::{
    DESCRIPTIONS_FIELD_ID, ID_FIELD_ID, MODIFY_TIMESTAMP_FIELD_ID, NAME_MAP_FIELD_ID,
};

use crate::{get_manage_tantivy_index, get_manage_tantivy_schema};
use super::commit_search_document::commit_search_document;

pub fn handle_insert_event(manage_id: i32, new_entity_id: &String, full_document: &bson::Document) {
    // log::info!(
    //     "handle_insert_event: manage_id: {}, new_entity_id: {}, full_document: {:?}",
    //     manage_id,
    //     new_entity_id,
    //     full_document
    // );

    let index = get_manage_tantivy_index(manage_id);
    let schema = get_manage_tantivy_schema(manage_id).unwrap();

    if let Ok(mut writer) = index.writer(15_000_000) {
        if let Err(err) = commit_search_document(full_document, schema, manage_id, None, writer) {
            log::error!("{}: {}: {:?}", t!("更新搜索文档失败"), manage_id, err);
            return;
        }
    } else {
        log::error!("{}: {}", t!("获取writer失败"), manage_id);
        return;
    };

    index.reader().unwrap().reload();
}


