use std::collections::BTreeMap;

use dependencies_sync::chrono::Utc;
// use dependencies_sync::tokio::stream::StreamExt;
use dependencies_sync::futures::stream::StreamExt;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};
use dependencies_sync::mongodb::{bson, bson::doc, bson::Bson, bson::Document, Collection};
use serde::Deserialize;

use cash_result::*;
use database::get_database;
use manage_define::general_field_ids::*;

use crate::utils::{add_modify_update_fields, get_timestamp_update_doc};

/// --------------------------
/// 插入或者更新实体的一个Map属性字段
/// --------------------------
pub async fn insert_entity_map_field(
    manage_id: &str,
    query_doc: Document,
    modify_doc: Document,
    account_id: &str,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists(manage_id, "update_entity")),
    };

    let mut _modify_doc = doc! { "$set":modify_doc.clone()};
    let  _modify_doc = add_modify_update_fields(account_id,  &mut _modify_doc);

    // 更新
    let result = collection
        .update_one(
            query_doc.clone(),
            _modify_doc,
            UpdateOptions::builder().upsert(true).build(),
        )
        .await;

    // 结果
    match result {
        Ok(r) => match r.modified_count == 1 {
            true => Ok(operation_succeed("ok")),
            false => Err(operation_failed(
                "insert_entity_map_field",
                format!(
                    "更新{}--{}-{}, 发生错误, 请检查数据正确性。",
                    manage_id, query_doc, r.modified_count
                ),
            )),
        },
        Err(_e) => Err(operation_failed(
            "insert_entity_map_field",
            format!("更新操作失败{}", query_doc),
        )),
    }
}
