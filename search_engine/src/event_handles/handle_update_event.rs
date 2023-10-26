use cash_result::OperationResult;
use dependencies_sync::bson::Document;
use dependencies_sync::log;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::serde_json::{self, json};
use dependencies_sync::tantivy::schema;

use entity;

use crate::event_handles::commit_search_document::commit_search_document;
use crate::{get_manage_tantivy_index, get_manage_tantivy_schema};

pub async fn handle_update_event(manage_id: i32, entity_id: &String, updates: &Document) {
    log::info!("Updating entity {} with updates {:?}", entity_id, updates);

    let index = get_manage_tantivy_index(manage_id);
    let schema = get_manage_tantivy_schema(manage_id).unwrap();

    if let Ok(mut index_writer) = index.writer(15_000_000) {
        let update_document =
            match entity::get_entity_by_objectid(&manage_id.to_string(), entity_id).await {
                Ok(entity) => entity,
                Err(err) => {
                    log::error!(
                        "{}, {}: {}, {}",
                        t!("更新实体失败"),
                        t!("获取实体失败"),
                        entity_id,
                        err.details()
                    );
                    return;
                }
            };

        if let Err(err) = commit_search_document(
            &update_document,
            schema,
            manage_id,
            Some(entity_id.clone()),
            index_writer,
        ) {
            log::error!("{}: {}", t!("更新搜索文档失败"), err.details());
        };
    } else {
        log::error!("{}: {}", t!("获取writer失败"), manage_id);
    }
    
    index.reader().unwrap().reload();
}
