use std::collections::BTreeMap;

use dependencies_sync::chrono::Utc;
// use dependencies_sync::tokio::stream::StreamExt;
use dependencies_sync::futures::stream::StreamExt;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};
use dependencies_sync::mongodb::{bson, bson::doc, bson::Bson, bson::Document, Collection, Cursor};
use serde::Deserialize;

use dependencies_sync::log::trace;

use cash_result::*;
use database::get_database;
use manage_define::general_field_ids::*;

/// 取得查询游标
pub async fn get_query_cursor(
    collection_id: &String,
    matches: Document,
    projects: Option<Document>,
    sorts: Option<Document>,
) -> Result<Cursor<Document>, OperationResult> {
    let collection = match database::get_collection_by_id(collection_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("get_entities_by_page")),
    };

    let mut pipeline: Vec<Document> = vec![];
    pipeline.push(doc! {"$match": matches});

    if projects.is_some() {
        pipeline.push(doc! {"$project": projects.clone().unwrap()});
    }
    
    if sorts.is_some() {
        pipeline.push(doc! {"$sort": sorts.clone().unwrap()});
    }

    let cursor = collection.aggregate(pipeline, None).await;

    match cursor {
        Ok(r) => Ok(r),
        Err(e) => Err(operation_failed(
            "entity::get_entities_by_page",
            format!("获取查询游标失败{}-{}", collection_id, e),
        )),
    }
}
