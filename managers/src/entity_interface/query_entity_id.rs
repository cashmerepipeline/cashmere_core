use cash_result::{operation_failed, OperationResult};
use dependencies_sync::{
    bson::Document,
    log,
    rust_i18n::{self, t},
};
use manage_define::general_field_ids::ID_FIELD_ID;

use crate::Manager;

use super::EntityInterface;

/// zh: 根据查询条件获取实体id, 使用前需要确定实体唯一性
pub async fn query_entity_id(
    manager: &Manager,
    query_doc: &Document,
) -> Result<String, OperationResult> {
    let entity_id = if let Ok(r) = manager.get_entities_by_filter(Some(query_doc)).await {
        if r.is_empty() {
            return Err(operation_failed("query_entity_id", t!("未找到实体")));
        }

        r[0].get_str(ID_FIELD_ID.to_string()).unwrap().to_string()
    } else {
        log::error!("{}: {:?}", t!("取得实体失败"), query_doc);
        return Err(operation_failed("query_entity_id", t!("取得实体失败")));
    };

    Ok(entity_id)
}
