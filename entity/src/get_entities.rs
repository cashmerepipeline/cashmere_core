use std::collections::BTreeMap;

use dependencies_sync::chrono::Utc;
// use dependencies_sync::tokio::stream::StreamExt;
use dependencies_sync::futures::stream::StreamExt;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::mongodb::options::{FindOneAndUpdateOptions, FindOptions, UpdateOptions};
use dependencies_sync::mongodb::{bson, bson::doc, bson::Bson, bson::Document, Collection};
use dependencies_sync::rust_i18n::{self, t};
use serde::Deserialize;

use cash_result::*;
use database::get_database;
use manage_define::general_field_ids::*;

/// zh: 如果没有查找到实体，则返回空表
pub async fn get_entities(
    collection_name: &str,
    filter: Option<&Document>,
    no_present_fields: &[String],
) -> Result<Vec<Document>, OperationResult> {
    let collection = match database::get_collection_by_id(collection_name).await {
        Some(c) => c,
        None => return Err(collection_not_exists(collection_name, "get_entities")),
    };

    let cursor = if !no_present_fields.is_empty() {
        let mut project_doc = doc! {};
        no_present_fields.iter().for_each(|f| {
            project_doc.insert(f.clone(), 0);
        });
        let find_options = FindOptions::builder().projection(project_doc).build();

        collection.find(filter.cloned().unwrap()).projection(project_doc).await
    } else {
        collection.find(filter.cloned().unwrap()).await
    };

    let mut result: Vec<Document> = Vec::new();
    match cursor {
        Ok(mut r) => {
            while let Some(d) = r.next().await {
                match d {
                    Ok(dc) => result.push(dc),
                    _ => continue,
                }
            }
            Ok(result)
        }
        Err(_e) => Err(operation_failed(
            "get_entities",
            format!("{}: {:?}", t!("取得实体失败"), filter),
        )),
    }
}
