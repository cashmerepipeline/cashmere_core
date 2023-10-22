use std::collections::BTreeMap;

use dependencies_sync::chrono::Utc;
// use dependencies_sync::tokio::stream::StreamExt;
use dependencies_sync::futures::stream::StreamExt;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::mongodb::{bson, bson::Bson, bson::doc, bson::Document, Collection};
use dependencies_sync::mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};
use dependencies_sync::log::{trace, debug};
use dependencies_sync::rust_i18n::{self, t};

use serde::Deserialize;


use cash_result::*;
use database::get_cashmere_database;
use manage_define::general_field_ids::*;

/// 取得条件排序分页
pub async fn get_entities_by_page(
    collection_id: &String,
    page_index: u32,
    matches: &Option<Document>,
    sorts: &Option<Document>,
    projects: &Option<Document>,
) -> Result<Vec<Document>, OperationResult> {
    let collection = match database::get_collection_by_id(collection_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("get_entities_by_page")),
    };

    let mut pipeline: Vec<Document> = vec![];
    if matches.is_some() {
        pipeline.push(doc! {"$match": matches});
    }

    if sorts.is_some() {
        pipeline.push(doc! {"$sort": sorts.clone().unwrap()});
    }

    if projects.is_some() {
        pipeline.push(doc! {"$project": projects.clone().unwrap()});
    }

    // zh: 注意: skip 需要在limit之前
    pipeline.push(doc! {"$skip": 20*(page_index-1)});
    pipeline.push(doc! {"$limit": 20_u32});
    
    let cursor = collection.aggregate(pipeline, None).await;

    let mut result: Vec<Document> = Vec::new();
    match cursor {
        Ok(mut r) => {
            while let Some(d) = r.next().await {
                match d {
                    Ok(dc) => result.push(dc),
                    Err(e) => {
                        debug!("{}: {}-{}-{}", t!("获取实体失败"), collection_id, page_index, e);
                        continue
                    },
                }
            }
            debug!("{}: {}-{}-{}", t!("成功获取实体分页"), collection_id, page_index, result.len());
            Ok(result)
        }
        Err(e) => Err(operation_failed(
            "entity::get_entities_by_page",
            format!("获取分页失败{}-{}-{}", collection_id, page_index, e),
        )),
    }
}
