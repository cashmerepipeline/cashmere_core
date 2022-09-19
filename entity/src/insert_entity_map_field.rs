use std::collections::BTreeMap;

use chrono::Utc;
// use tokio::stream::StreamExt;
use futures::stream::StreamExt;
use linked_hash_map::LinkedHashMap;
use mongodb::{bson, bson::Bson, bson::doc, bson::Document, Collection};
use mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};
use serde::Deserialize;

use cash_result::*;
use database::get_cashmere_database;
use manage_define::general_field_ids::*;

/// --------------------------
/// 插入或者更新实体的一个Map属性字段
/// --------------------------
pub async fn insert_entity_map_field(
    manage_id: &String,
    query_doc: Document,
    modify_doc: Document,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("update_entity")),
    };

    // 更新
    let result = collection
        .update_one(
            query_doc.clone(),
            doc! {
                "$set":modify_doc,
                "$set": {
                MODIFIER_FIELD_ID.to_string(): account_id.clone(),
                MODIFY_TIMESTAMP_FIELD_ID.to_string(): Utc::now().timestamp()
                }
            },
            UpdateOptions::builder().upsert(true).build(),
        )
        .await;

    println!("{:?}-{:?}", manage_id,  result);

    // 结果
    match result {
        Ok(r) => match r.modified_count == 1 {
            true => Ok(operation_succeed("ok")),
            false => Err(operation_failed(
                "insert_entity_map_field",
                format!("更新了多个实体{}, 更新发生错误, 请检查数据正确性。", query_doc),
            )),
        },
        Err(_e) => Err(operation_failed(
            "insert_entity_map_field",
            format!("更新操作失败{}", query_doc),
        )),
    }
}
