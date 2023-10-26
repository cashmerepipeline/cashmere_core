use cash_result::{operation_failed, OperationResult};
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

pub fn commit_search_document(
    update_document: &bson::Document,
    schema: std::sync::Arc<Schema>,
    manage_id: i32,
    // 这里的id为数据库的objectid
    entity_id: Option<String>,
    mut writer: tantivy::IndexWriter,
) -> Result<(), OperationResult> {
    // zh: 如果存在，先删除旧的
    if entity_id.is_some() {
        writer.delete_term(Term::from_field_text(
            schema.get_field("_id").unwrap(),
            entity_id.unwrap().as_str(),
        ));
    }

    let database_id = update_document.get_object_id("_id").unwrap().to_string();
    let id = update_document.get_str(ID_FIELD_ID.to_string()).unwrap();
    let name_map = update_document
        .get_document(NAME_MAP_FIELD_ID.to_string())
        .unwrap();
    let language_code = name_map.keys().next().unwrap().as_str();
    let name = name_map.values().next().unwrap().as_str().unwrap();
    let mut name_map_json = json!({
        language_code:name
    });
    let description = update_document
        .get_str(DESCRIPTIONS_FIELD_ID.to_string())
        .unwrap_or("");

    let modify_time =
        if let Ok(t) = update_document.get_timestamp(MODIFY_TIMESTAMP_FIELD_ID.to_string()) {
            t.time as i64
        } else {
            Utc::now().timestamp_millis()
        };
    let json_doc: serde_json::Value = json!({
            "_id": database_id,
            ID_FIELD_ID.to_string(): id,
            NAME_MAP_FIELD_ID.to_string(): name_map_json,
            DESCRIPTIONS_FIELD_ID.to_string(): description,
            MODIFY_TIMESTAMP_FIELD_ID.to_string(): modify_time
    });
    let doc = if let Ok(doc) = TantivyDocument::parse_json(&schema, json_doc.to_string().as_str()) {
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
    if let Err(e) = writer.commit() {
        log::error!("{}: {}: {:?}", t!("提交失败"), manage_id, e);
        return Err(operation_failed("update_search_document", t!("提交失败")));
    };

    Ok(())
}
