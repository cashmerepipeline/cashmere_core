use crate::view_rule::ViewRule;

use super::get_manage_view_rules;

/// 取得管理视图规则
pub async fn query_manage_view_rules(manage_id: &String, group_id: &String) -> Option<ViewRule> {
    if let Some(manage_view_rules_map) = get_manage_view_rules(manage_id).await {
        let m = manage_view_rules_map.read();
        m.manage.get(group_id).cloned()
    } else {
        None
    }
}
