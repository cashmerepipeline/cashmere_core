use dependencies_sync::mongodb::bson::Document;
use manage_define::general_field_ids::*;

/// 检查存在，返回实体id，（ID_FIELD_ID）
pub async fn entity_exists(manage_id: &str, query_doc: &Document) -> Option<String> {
    // 检查
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return None,
    };

    let result = collection.find_one(query_doc.clone(), None).await;

    match result {
        Ok(Some(r)) => {
            if let Ok(i) = r.get_str(ID_FIELD_ID.to_string()) {
                Some(i.to_string())
            } else { None }
        }
        Ok(None) => None,
        Err(_e) => None,
    }
}
