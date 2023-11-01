use crate::view_rules_map::query_manage_view_rules;
use crate::ReadRule;

/// 管理是否可写, 管理的字段定义添加删除
pub async fn can_manage_read(manage_id: &String, role_group: &String) -> bool {
    let view_rules = if let Some(r) = query_manage_view_rules(manage_id, role_group).await {
        r
    } else {
        return false;
    };

    view_rules.read_rule == ReadRule::Read
        || view_rules.read_rule == ReadRule::OwnerRead
        || view_rules.read_rule == ReadRule::GroupRead
}
