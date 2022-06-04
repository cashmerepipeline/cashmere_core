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

/// 更新实体多个个属性
pub async fn update_entity_fields(
    entity_id: &String,
    collection: &String,
    new_value: Document,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(collection).await {
        Some(c) => c,
        None => return Err(collection_not_exists("update_entity")),
    };

    // 更新
    let result = collection
        .update_one(
            doc! {
                "_id": entity_id
            },
            doc! {
                "$set": new_value,
                MODIFIER_FIELD_ID.to_string(): account_id.clone(),
                MODIFY_TIMESTAMP_FIELD_ID.to_string(): Utc::now().timestamp()
            },
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
