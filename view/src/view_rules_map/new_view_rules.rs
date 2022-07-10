use cash_result::OperationResult;
use crate::view_rules::ViewRules;
use crate::view_rules_map::get_view_rules_map;
use crate::view_rules_map::new_view_rules_entity_to_database::new_view_rules_entity_to_database;

/// 新建映射
pub async fn new_view_rules(
    id: i32,
    name: &String,
    rules: &ViewRules,
    account_id: &String,
    group_id: &String,
) -> Result<String, OperationResult> {
    // 创建doc
    let result = new_view_rules_entity_to_database(id, name, rules, account_id, group_id).await;
    // 加入全局列表
    match result {
        Ok(r) => {
            let view_rules_arc = get_view_rules_map().await;
            let mut view_rules_map = view_rules_arc.write();
            view_rules_map.insert(name.clone(), rules.clone());
            Ok(r)
        }
        Err(e) => Err(e),
    }
}

