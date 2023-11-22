use dependencies_sync::bson::Document;
use dependencies_sync::mongodb::Collection;
use manage_define::manage_ids::MANAGES_MANAGE_ID;
use crate::{collection_exists, get_database};

/// 取得管理-管理集合, 不存在则新建
pub async fn get_manages_collection() -> Collection<Document> {
    let cashmere_db = get_database().await;

    let manages_id = &MANAGES_MANAGE_ID.to_string();

    // manages 不存在则创建
    if !collection_exists(manages_id).await {
        cashmere_db
            .create_collection(manages_id, None)
            .await
            .expect("创建管理失败");
    }

    cashmere_db.collection(manages_id)
}
