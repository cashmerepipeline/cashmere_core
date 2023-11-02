use dependencies_sync::bson::Document;
use dependencies_sync::log;
use dependencies_sync::rust_i18n::{self, t};

use crate::collection_event_handles::commit_search_document::commit_search_document;
use crate::{get_manage_tantivy_index, get_manage_tantivy_schema};

pub fn handle_update_event(
    manage_id: i32,
    object_id: &String,
    updates: &Document,
    full_document: &Document,
) {
    log::info!(
        "{}: {}-{} {:?}",
        t!("更新实体索引"),
        manage_id,
        object_id,
        updates
    );

    let _index = get_manage_tantivy_index(manage_id);
    let schema = get_manage_tantivy_schema(manage_id).unwrap();

    // 如果字段全部不在模式表中，则返回
    let field_names = schema
        .fields()
        .map(|(_, x)| x.name().to_string())
        .collect::<Vec<String>>();

    let mut need_update = false;
    for (key, _) in updates.iter() {
        if field_names.contains(key) {
            need_update = true;
        }
    }
    if !need_update {
        return;
    }


    if let Err(err) =
        commit_search_document(full_document, schema, manage_id, Some(object_id.clone()))
    {
        log::error!("{}: {}", t!("更新搜索文档失败"), err.details());
    };
}
