use cash_result::{operation_failed, operation_succeed, OperationResult};
use dependencies_sync::mongodb::bson::Document;
use dependencies_sync::rust_i18n::{self, t};
use manage_define::general_field_ids::*;

/// 检查存在，返回实体id，（ID_FIELD_ID）
pub async fn delete_entity(
    manage_id: &str,
    match_doc: &Document,
) -> Result<OperationResult, OperationResult> {
    // 检查
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(operation_failed("delete_entity", t!("取得集合失败"))),
    };

    let result = collection.delete_one(match_doc.clone(), None).await;

    match result {
        Ok(r) => Ok(operation_succeed("ok")),
        Err(_e) => Err(operation_failed("delete_entity", t!("删除失败"))),
    }
}
