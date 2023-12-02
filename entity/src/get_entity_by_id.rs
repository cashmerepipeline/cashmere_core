use std::collections::BTreeMap;

use dependencies_sync::chrono::Utc;
use dependencies_sync::futures::stream::StreamExt;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::mongodb::options::{UpdateOptions, FindOneOptions};
use dependencies_sync::mongodb::{bson, bson::doc, bson::Bson, bson::Document, Collection};
use serde::Deserialize;

use cash_result::*;
use database::get_database;
use manage_define::general_field_ids::*;

/// 取得 实体
pub async fn get_entity_by_id(
    collection_name: &str,
    id: &str,
    no_present_fields: &[String],
) -> Result<Document, OperationResult> {
    if !database::collection_exists(collection_name).await {
        return Err(collection_not_exists("get_entity_by_id"));
    }

    let collection = match database::get_collection_by_id(collection_name).await {
        Some(c) => c,
        None => return Err(collection_not_exists("get_entity_by_id")),
    };

    let mut project_doc = doc! {};
    no_present_fields.iter().for_each(|f| {project_doc.insert(f.clone(), 0);});

    let result = if no_present_fields.len() > 0 {
        let find_option = FindOneOptions::builder().projection(project_doc).build();
        collection
            .find_one(
                doc! {
                    ID_FIELD_ID.to_string(): id,
                },
                Some(find_option),
            )
            .await
    } else {
        collection
            .find_one(
                doc! {
                    ID_FIELD_ID.to_string(): id,
                },
                None,
            )
            .await
    };

    match result {
        Ok(r) => match r {
            Some(d) => Ok(d),
            None => Err(operation_failed(
                "get_entity_by_id",
                format!("无数据 {}", id),
            )),
        },
        Err(_e) => Err(operation_failed(
            "get_entity_by_id",
            format!("获取失败{}", id),
        )),
    }
}
