use crate::{Manager, ManagerTrait};
use dependencies_sync::{
    bson::Document,
    rust_i18n::{self, t},
    tonic::Status,
};
use manage_define::general_field_ids::{ID_FIELD_ID, REMOVED_FIELD_ID};
use std::sync::Arc;

/// 新建一个实体记录
pub async fn make_new_entity_document(
    manager: &Arc<Manager>,
    account_id: &str,
) -> Result<Document, Status> {
    match manager.get_new_entity_id(account_id).await {
        Some(new_id) => {
            let mut new_entity_doc = Document::new();

            new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
            new_entity_doc.insert(REMOVED_FIELD_ID.to_string(), false);

            Ok(new_entity_doc)
        }
        None => Err(Status::aborted(format!(
            "{}: {}",
            t!("获取新实体失败"),
            manager.get_id()
        ))),
    }
}
