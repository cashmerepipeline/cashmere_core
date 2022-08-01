use crate::{Manager, ManagerTrait};
use bson::Document;
use manage_define::general_field_ids::ID_FIELD_ID;
use std::sync::Arc;

/// 新建一个实体记录
pub async fn make_new_entity_document(manager: &Arc<Manager>) -> Option<Document> {
    if let Some(new_id) = manager.get_new_entity_id().await {
        let mut new_entity_doc = Document::new();
        new_entity_doc.insert("_id", new_id.to_string());
        new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());

        Some(new_entity_doc)
    } else {
        None
    }
}

