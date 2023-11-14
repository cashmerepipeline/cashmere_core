use std::sync::Arc;
use dependencies_sync::bson::Document;
use manage_define::general_field_ids::{ID_FIELD_ID, REMOVED_FIELD_ID};
use crate::{Manager, ManagerTrait};

/// 新建一个实体记录
pub async fn make_new_entity_document(manager: &Arc<Manager>, account_id: &str) -> Option<Document> {
    if let Some(new_id) = manager.get_new_entity_id(account_id).await {
        let mut new_entity_doc = Document::new();

        new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
        new_entity_doc.insert(REMOVED_FIELD_ID.to_string(), false);

        Some(new_entity_doc)
    } else {
        None
    }
}
