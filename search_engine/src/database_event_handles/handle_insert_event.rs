use dependencies_sync::bson;
use dependencies_sync::log;
use dependencies_sync::rust_i18n::{self, t};

use super::commit_search_document::commit_search_document;

use crate::get_tantivy_schema;

pub fn handle_insert_event(manage_id: i32, full_document: &bson::Document) {
    log::warn!("{}: {}", t!("开始插入查询"), manage_id);

    let schema = get_tantivy_schema();

    if let Err(err) = commit_search_document(full_document, schema, manage_id, None) {
        log::error!("{}: {}: {:?}", t!("更新搜索文档失败"), manage_id, err);
    }
}
