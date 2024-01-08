use cash_result::{add_call_name_to_chain, OperationResult};
use dependencies_sync::bson::Document;

/// 通过过滤取得实体
pub async fn get_entities_by_filter(
    manage_id: &str,
    filter: &Option<Document>,
) -> Result<Vec<Document>, OperationResult> {
    match entity::get_entities(manage_id, filter, &vec![]).await {
        Ok(r) => Ok(r),
        Err(e) => Err(add_call_name_to_chain(e, "get_entity_by_id".to_string())),
    }
}
