use std::collections::BTreeMap;

use dependencies_sync::chrono::Utc;
use dependencies_sync::futures::stream::StreamExt;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::log::{error, debug};
use dependencies_sync::mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};
use dependencies_sync::mongodb::{bson, bson::doc, bson::Bson, bson::Document, Collection};
use serde::Deserialize;

use cash_result::*;
use database::get_database;
use manage_define::general_field_ids::*;

/// 插入实体, 返回插入的实体的_id
pub async fn insert_entity(
    manage_id: &str,
    entity_doc: &mut Document,
    account_id: &str,
    group_id: &str,
) -> Result<String, OperationResult> {
    // 检查
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists(manage_id, "insert_entity")),
    };
    
    debug!("insert_entity {:?}", entity_doc);

    if !entity_doc.contains_key(ID_FIELD_ID.to_string()) {
        return Err(collection_not_exists(manage_id, "insert_entity"));
    }

    let id = entity_doc
        .get_str(&ID_FIELD_ID.to_string())
        .unwrap()
        .to_string();

    // 创建标记
    entity_doc.insert(CREATOR_FIELD_ID.to_string(), account_id);
    entity_doc.insert(MODIFIER_FIELD_ID.to_string(), account_id);
    entity_doc.insert(OWNER_FIELD_ID.to_string(), account_id);
    entity_doc.insert(GROUPS_FIELD_ID.to_string(), vec![group_id]);

    // 插入, 返回插入后的ID
    let result = collection
        .insert_one(entity_doc.clone(), None)
        .and_then(|_r| async {
            // 需要单独更新时间戳
            let query_doc = doc! {
                ID_FIELD_ID.to_string(): id.clone(),
            };

            let updates = doc! {
                    "$currentDate": doc!{
                        MODIFY_TIMESTAMP_FIELD_ID.to_string(): {"$type":"timestamp"},
                    CREATE_TIMESTAMP_FIELD_ID.to_string(): {"$type":"timestamp"},
                }
            };
            collection.update_one(query_doc, updates, None).await
        })
        .await;

    // 结果
    match result {
        Ok(_r) => Ok(id),
        Err(_e) => {
            error!("{}", _e);
            Err(operation_failed(
                "insert_entity",
                format!("插入实体失败 {}-{}: {}", manage_id, id, _e),
            ))
        }
    }
}
