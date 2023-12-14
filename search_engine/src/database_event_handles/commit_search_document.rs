use cash_result::{operation_failed, OperationResult};
use dependencies_sync::bson::{self};
use dependencies_sync::chrono::Utc;
use dependencies_sync::log;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::serde_json::{self, json};

use tantivy::{schema::*, Document as TantivyDocument};
use manage_define::general_field_ids::{
    DESCRIPTION_FIELD_ID, ID_FIELD_ID, MODIFY_TIMESTAMP_FIELD_ID, NAME_MAP_FIELD_ID,
};

use crate::get_tantivy_writer;

pub fn commit_search_document(
    update_document: &bson::Document,
    schema: std::sync::Arc<Schema>,
    manage_id: &str,
    // 这里的id为数据库的objectid
    object_id: Option<String>,
) -> Result<(), OperationResult> {
    let writer_arc = get_tantivy_writer();
    let writer = writer_arc.read();

    // zh: 如果存在，先删除旧的
    if let Some(obj_id) = object_id {
        writer.delete_term(Term::from_field_text(
            schema.get_field("_id").unwrap(),
            obj_id.as_str(),
        ));
    }

    let object_id = update_document.get_object_id("_id").unwrap().to_string();
    let id = update_document.get_str(ID_FIELD_ID.to_string()).unwrap();
    let name_map = update_document
        .get_document(NAME_MAP_FIELD_ID.to_string())
        .unwrap();
    let language_code = name_map.keys().next().unwrap().as_str();
    let name = name_map.values().next().unwrap().as_str().unwrap();
    let name_map_json = json!({
        language_code:name
    });
    let description = update_document
        .get_str(DESCRIPTION_FIELD_ID.to_string())
        .unwrap_or("");

    let modify_time =
        if let Ok(t) = update_document.get_timestamp(MODIFY_TIMESTAMP_FIELD_ID.to_string()) {
            t.time as i64
        } else {
            Utc::now().timestamp_millis()
        };
    let json_doc: serde_json::Value = json!({
            "mid": manage_id,
            "_id": object_id,
            ID_FIELD_ID.to_string(): id,
            NAME_MAP_FIELD_ID.to_string(): name_map_json,
            DESCRIPTION_FIELD_ID.to_string(): description,
            MODIFY_TIMESTAMP_FIELD_ID.to_string(): modify_time
    });
    // let doc = if let Ok(doc) = TantivyDocument::parse_json(&schema, json_doc.to_string().as_str()) {
    let doc = if let Ok(doc) = schema.parse_document(json_doc.to_string().as_str()) {
        doc
    } else {
        log::error!("{}: {}: {:?}", t!("转换为tdoc失败"), manage_id, json_doc);
        return Err(operation_failed(
            "update_search_document",
            t!("转换为tdoc失败"),
        ));
    };
    if let Err(e) = writer.add_document(doc) {
        log::error!("{}: {}: {:?}", t!("添加文档失败"), manage_id, e);
        return Err(operation_failed(
            "update_search_document",
            t!("添加文档失败"),
        ));
    };
    Ok(())
}
