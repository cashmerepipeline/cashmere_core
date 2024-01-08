use cash_result::{add_call_name_to_chain, operation_failed, OperationResult};
use dependencies_sync::{
    bson::Document,
    rust_i18n::{self, t},
};

use crate::entity_cache_map::cache_get_entity;

/// 通过id取得实体
pub async fn get_entity_by_id(
    manage_id: &'static str,
    entity_id: &str,
    has_cache: bool,
    no_present_fields: &Vec<String>,
) -> Result<Document, OperationResult> {
    // 如果存在缓存，从缓存中取得
    if has_cache {
        let result = cache_get_entity(manage_id, entity_id);
        return match result {
            Some(r) => Ok(r),
            None => Err(operation_failed("get_entity_by_id", t!("取得实体缓存失败"))),
        };
    }

    match entity::get_entity_by_id(manage_id, entity_id, no_present_fields).await {
        Ok(r) => Ok(r),
        Err(e) => Err(add_call_name_to_chain(e, "get_entity_by_id".to_string())),
    }
}
