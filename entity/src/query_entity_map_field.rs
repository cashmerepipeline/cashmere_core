use std::collections::BTreeMap;

use dependencies_sync::chrono::Utc;
// use dependencies_sync::tokio::stream::StreamExt;
use dependencies_sync::futures::stream::StreamExt;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::mongodb::options::{FindOneAndUpdateOptions, FindOneOptions, UpdateOptions};
use dependencies_sync::mongodb::{bson, bson::doc, bson::Bson, bson::Document, Collection};
use serde::Deserialize;

use cash_result::*;
use database::get_database;
use manage_define::general_field_ids::*;

use crate::utils::{add_modify_update_fields, get_timestamp_update_doc};

/// --------------------------
/// 取得实体的Map属性字段
/// --------------------------
pub async fn query_entity_map_field(
    manage_id: &str,
    query_doc: &Document,
    map_field: &str,
) -> Result<Document, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists(manage_id, "query_entity_map_field")),
    };

    let mut project_doc = Document::new();
    for f in query_doc.keys() {
        project_doc.insert(f, 1);
    }
    project_doc.insert(map_field, 1);

    let find_options = FindOneOptions::builder()
        .projection(Some(project_doc))
        .build();

    // 更新
    let result = collection.find_one(query_doc.clone(), find_options).await;

    // 结果
    match result {
        Ok(r) => match r {
            Some(d) => Ok(d),
            None => Err(operation_failed(
                "query_entity_map_field",
                format!(
                    "查询{}--{}, 发生错误, 请检查数据正确性。",
                    manage_id, query_doc,
                ),
            )),
        },
        Err(_e) => Err(operation_failed(
            "query_entity_map_field",
            format!("更新操作失败{}", query_doc),
        )),
    }
}
