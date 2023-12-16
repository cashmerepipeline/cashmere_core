use crate::view_rules_map::query_collection_view_rules;
use crate::FilterRule;

/// 实体是否可读
pub async fn can_entity_read(manage_id: &String, role_group: &String) -> bool {
    let view_rules = if let Some(r) = query_collection_view_rules(manage_id, role_group).await {
        r
    } else {
        return false;
    };

    view_rules.read_filters.contains(&FilterRule::NoLimit)
        || view_rules.read_filters.contains(&FilterRule::OnlyOwner)
        || view_rules.read_filters.contains(&FilterRule::OnlyGroup)
}
