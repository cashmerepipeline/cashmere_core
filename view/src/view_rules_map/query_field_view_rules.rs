use crate::view_rule::ViewRule;

use super::get_manage_view_rules;

/// 取得属性视图规则
pub async fn query_field_view_rules(
    manage_id: &String,
    field_id: &String,
    group_id: &String,
) -> Option<ViewRule> {
    if let Some(manage_view_rules_map) = get_manage_view_rules(manage_id).await {
        let m = manage_view_rules_map.read();
        m.schema
            .get(field_id)
            .and_then(|schema| schema.get(group_id).map(|rule| rule.clone()))
    } else {
        None
    }
}
