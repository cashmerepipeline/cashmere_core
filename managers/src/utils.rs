use crate::{Manager, ManagerTrait};
use dependencies_sync::bson;
use dependencies_sync::bson::Document;
use manage_define::general_field_ids::{COMMENTS_FIELD_ID, DATAS_FIELD_ID, DATAS_REMOVED_FIELD_ID, ENTITY_REMOVED_FIELD_ID, ID_FIELD_ID};
use std::sync::Arc;

/// 新建一个实体记录
pub async fn make_new_entity_document(manager: &Arc<Manager>) -> Option<Document> {
    if let Some(new_id) = manager.get_new_entity_id().await {
        let mut new_entity_doc = Document::new();
        let empty_vec:Vec<String> = vec![];
        new_entity_doc.insert("_id", new_id.to_string());

        new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
        new_entity_doc.insert(DATAS_FIELD_ID.to_string(), bson::to_bson(&empty_vec).unwrap());
        new_entity_doc.insert(DATAS_REMOVED_FIELD_ID.to_string(), bson::to_bson(&empty_vec).unwrap());
        new_entity_doc.insert(COMMENTS_FIELD_ID.to_string(), bson::to_bson(&empty_vec).unwrap());
        new_entity_doc.insert(ENTITY_REMOVED_FIELD_ID.to_string(), false);

        Some(new_entity_doc)
    } else {
        None
    }
}

