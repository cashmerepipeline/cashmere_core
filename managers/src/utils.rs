use std::sync::Arc;
use bson::Document;
use manage_define::general_field_ids::ID_FIELD_ID;
use crate::{Manager, ManagerTrait};


/// 新建一个实体记录
pub async fn make_new_entity_document(manager: &Arc<Manager>) -> Option<Document> {
    if let new_id = manager.get_new_entity_id().await {
        let mut new_entity_doc = Document::new();
        new_entity_doc.insert("_id", new_id.clone());
        new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.clone());

        Some(new_entity_doc)
    } else {
        None
    }
}