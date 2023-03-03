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

pub async fn exists_by_id(manage_id: &String, entity_id: &String) -> bool {
    let query_doc = doc! {
        "_id": entity_id.clone()
    };

    crate::entity_exists(manage_id, &query_doc).await
}
