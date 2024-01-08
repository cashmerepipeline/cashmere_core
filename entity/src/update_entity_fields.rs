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

use crate::utils::get_timestamp_update_doc;

/// 更新实体多个个属性
pub async fn update_entity_fields(
    entity_id: &str,
    collection: &str,
    new_value: Document,
    account_id: &str,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(collection).await {
        Some(c) => c,
        None => return Err(collection_not_exists(collection, "update_entity_fields")),
    };

    let pipeline_docs: Vec<Document> = vec![
                doc! {
                    "$set": new_value,
                },
                doc! {
                    "$set": {MODIFIER_FIELD_ID.to_string(): account_id}
                },
                get_timestamp_update_doc(),
    ];

    // 更新
    let result = collection
        .update_one(
            doc! {
                ID_FIELD_ID.to_string(): entity_id
            },
            pipeline_docs,
            None,
        )
        .await;

    // 结果
    match result {
        Ok(r) => match r.modified_count == 1 {
            true => Ok(operation_succeed("succeed")),
            false => Err(operation_failed(
                "updata_entity",
                format!("更新了多个实体{}", entity_id),
            )),
        },
        Err(_e) => Err(operation_failed(
            "update_entity",
            format!("更新操作失败{}", entity_id),
        )),
    }
}
