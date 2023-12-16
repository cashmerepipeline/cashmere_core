use dependencies_sync::bson;

use crate::view_rules::ViewRules;
use cash_result::OperationResult;
use manage_define::manage_ids::VIEW_RULES_MANAGE_ID;

/// 新建映像规则实体
pub async fn new_view_rules_entity_to_database(
    id: i32,
    name: &String,
    rules: &ViewRules,
    account_id: &String,
    group_id: &String,
) -> Result<String, OperationResult> {
    // 创建doc
    let mut view_rules_doc = bson::to_document(rules).unwrap();
    // 指定名
    view_rules_doc.insert("_id", id);
    view_rules_doc.insert("name", name);

    // 入库

    entity::insert_entity(
        VIEW_RULES_MANAGE_ID,
        &mut view_rules_doc,
        account_id,
        group_id,
    )
    .await
}
