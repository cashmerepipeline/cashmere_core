use crate::view_rules_map::query_manage_view_rules;
use crate::WriteRule;

/// 管理是否可写, 管理的字段定义添加删除
pub async fn can_manage_write(manage_id: &str, group: &str) -> bool {
    let view_rules = if let Some(r) = query_manage_view_rules(manage_id, group).await {
        r
    } else {
        return false;
    };

    view_rules.write_rule == WriteRule::Write
        || view_rules.write_rule == WriteRule::OwnerWrite
        || view_rules.write_rule == WriteRule::GroupWrite
}
