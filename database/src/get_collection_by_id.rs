use dependencies_sync::bson::Document;
use dependencies_sync::mongodb::Collection;
use crate::{collection_exists, get_database};

/// 取得集合
pub async fn get_collection_by_id(manage_id: &str) -> Option<Collection<Document>> {
    let cashmere_db = get_database().await;

    // 不存在
    if !collection_exists::collection_exists(manage_id).await {
        return None;
    }

    Some(cashmere_db.collection(manage_id))
}
