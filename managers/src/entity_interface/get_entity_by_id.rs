use cash_result::{add_call_name_to_chain, operation_failed, OperationResult};
use dependencies_sync::{
    bson::Document,
    rust_i18n::{self, t},
};
use entity::hard_code_cache::hard_coded_cache_get_entity;

/// 通过id取得实体
pub async fn get_entity_by_id(
    manage_id: &'static str,
    entity_id: &str,
    hard_coded: bool,
    present_fields: &[String],
    no_present_fields: &[String],
) -> Result<Document, OperationResult> {
    // 如果存在缓存，从缓存中取得
    if hard_coded {
        let result = hard_coded_cache_get_entity(manage_id, entity_id, present_fields, no_present_fields).await;
        return match result {
            Some(r) => Ok(r),
            None => Err(operation_failed("get_entity_by_id", t!("取得硬编码实体缓存失败"))),
        };
    }

    match entity::get_entity_by_id(manage_id, entity_id, present_fields, no_present_fields).await {
        Ok(r) => Ok(r),
        Err(e) => Err(add_call_name_to_chain(e, "get_entity_by_id".to_string())),
    }
}
