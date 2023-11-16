use crate::view_rules_map::query_collection_view_rules;
use crate::WriteRule;

/// 集合是否可写，向集合添加或者删除实体
pub async fn can_collection_write(manage_id: &String, group: &String) -> bool {
    let view_rules = if let Ok(r) = query_collection_view_rules(manage_id, group).await {
        r
    } else {
        return false;
    };

    view_rules.write_rule == WriteRule::Write
        || view_rules.write_rule == WriteRule::OwnerWrite
        || view_rules.write_rule == WriteRule::GroupWrite
}
