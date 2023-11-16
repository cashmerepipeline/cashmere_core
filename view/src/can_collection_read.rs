use crate::view_rules_map::query_collection_view_rules;
use crate::ReadRule;

/// 集合是否可写，向集合添加或者删除实体
pub async fn can_collection_read(manage_id: &String, role_group: &String) -> bool {
    let view_rules = if let Ok(r) = query_collection_view_rules(manage_id, role_group).await {
        r
    } else {
        return false;
    };

    view_rules.read_rule == ReadRule::Read
        || view_rules.read_rule == ReadRule::OwnerRead
        || view_rules.read_rule == ReadRule::GroupRead
}
