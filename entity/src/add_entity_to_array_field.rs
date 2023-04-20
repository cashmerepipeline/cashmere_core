use std::collections::BTreeMap;

use chrono::Utc;
// use tokio::stream::StreamExt;
use futures::stream::StreamExt;
use linked_hash_map::LinkedHashMap;
use mongodb::{bson, bson::Bson, bson::doc, bson::Document, Collection};
use mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};
use serde::Deserialize;
use log::{trace, debug};

use cash_result::*;
use database::get_cashmere_database;
use manage_define::general_field_ids::*;

///  添加实体到数组字段
/// 数组相当于集合，不会重复添加
pub async fn add_entity_to_array_field(
    manage_id: &String,
    query_doc: Document,
    modify_doc: Document,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("push_entity_array_field")),
    };

    // 更新
    let result = collection
        .update_one(
            query_doc.clone(),
            doc! {
                // 添加
                "$addToSet": modify_doc.clone(),
                "$set": {
                    MODIFIER_FIELD_ID.to_string(): account_id.clone(),
                    MODIFY_TIMESTAMP_FIELD_ID.to_string(): Utc::now().timestamp()
                }
            },
            None,
        )
        .await;

    // 结果
    match result {
        Ok(r) => match r.modified_count {
            0 => {
                trace!("没有实体被更新: {}-{}", query_doc, modify_doc);
                Ok(operation_succeed("succeed"))
            },
            1 => Ok(operation_succeed("succeed")),
            _ => Err(operation_failed(
                "push_entity_array_field",
                format!("更新了多个实体{}-{}", query_doc, r.modified_count),
            )),
        },
        Err(_e) => Err(operation_failed(
            "push_entity_array_field",
            format!("添加操作失败{}", query_doc),
        )),
    }
}
