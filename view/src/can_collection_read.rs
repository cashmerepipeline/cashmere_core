use crate::view_rules_map::query_collection_view_rules;
use crate::ReadRule;

/// 集合是否可写，向集合添加或者删除实体
pub async fn can_collection_read(manage_id: &str, role_group: &str) -> bool {
    let view_rule = if let Some(r) = query_collection_view_rules(manage_id, role_group).await {
        r
    } else {
        return false;
    };

    view_rule.read_rule == ReadRule::Read
        || view_rule.read_rule == ReadRule::OwnerRead
        || view_rule.read_rule == ReadRule::GroupRead
}
