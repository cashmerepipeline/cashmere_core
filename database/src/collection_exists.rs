use crate::get_cashmere_database;

/// 集合是否存在
pub async fn collection_exists(collection: &String) -> bool {
    let db = get_cashmere_database().await;
    let collections = db.list_collection_names(None).await.unwrap();

    collections.contains(collection)
}
