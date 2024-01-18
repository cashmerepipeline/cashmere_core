use cash_result::{OperationResult, add_call_name_to_chain};
use dependencies_sync::bson::Document;

/// 添加映射字段
pub async fn insert_entity_map_field(
    manage_id: &str,
    query_doc: Document,
    modify_doc: Document,
    account_id: &str,
) -> Result<OperationResult, OperationResult> {
    match entity::insert_entity_map_field(&manage_id, query_doc, modify_doc, account_id).await {
        Ok(r) => Ok(r),
        Err(e) => Err(add_call_name_to_chain(
            e,
            "insert_entity_map_field".to_string(),
        )),
    }
}
