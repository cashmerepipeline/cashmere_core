use std::collections::BTreeMap;

use dependencies_sync::chrono::Utc;
// use dependencies_sync::tokio::stream::StreamExt;
use dependencies_sync::futures::stream::StreamExt;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};
use dependencies_sync::mongodb::{
    bson, bson::doc, bson::oid::ObjectId, bson::Bson, bson::Document, Collection, Cursor,
};
use serde::Deserialize;

use dependencies_sync::log::trace;

use cash_result::*;
use database::get_database;
use manage_define::general_field_ids::*;

/// zh: 取得查询游标，查询从指定的实体编号开始，使用oid作为排序索引
pub async fn get_query_cursor(
    collection_id: &str,
    matches: Document,
    unsets: Option<Vec<String>>,
    sort_doc: Option<Document>,
    start_oid: Option<&str>,
    skip_count: u32,
) -> Result<Cursor<Document>, OperationResult> {
    let collection = match database::get_collection_by_id(collection_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("get_entities_by_page")),
    };

    let mut pipeline: Vec<Document> = vec![];
    if let Some(last_oid) = start_oid {
        let oid = ObjectId::parse_str(last_oid).unwrap();
        let mut r = doc! {"_id":{"$lt": oid}};
        matches.iter().for_each(|(k, v)| {
            r.insert(k, v.clone());
        });
        pipeline.push(doc! {"$match": r});
    } else {
        pipeline.push(doc! {"$match": matches});
    }

    if unsets.is_some() {
        pipeline.push(doc! {"$unset": unsets.clone().unwrap()});
    }

    if sort_doc.is_some() {
        pipeline.push(doc! {"$sort": sort_doc});
    }

    pipeline.push(doc! {"$skip": skip_count});

    let cursor = collection.aggregate(pipeline, None).await;

    match cursor {
        Ok(r) => Ok(r),
        Err(e) => Err(operation_failed(
            "entity::get_query_cursor",
            format!("获取查询游标失败{}-{}", collection_id, e),
        )),
    }
}
