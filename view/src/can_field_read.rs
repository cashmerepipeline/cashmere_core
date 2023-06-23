use crate::view_rules_map::get_view_rules_map;
use crate::view_rules_map::query_collection_view_rules;
use crate::view_rules_map::query_field_view_rules;
use crate::FilterRule;
use crate::ReadRule;

/// 实体的可写性，可否修改实体的字段
pub async fn can_field_read(manage_id: &String, field_id: &String, role_group: &String) -> bool {
    let collection_view_rules =
        if let Some(r) = query_collection_view_rules(manage_id, role_group).await {
            r
        } else {
            return false;
        };

    let field_view_rules =
        if let Some(r) = query_field_view_rules(manage_id, field_id, role_group).await {
            r
        } else {
            return false;
        };

    collection_view_rules.read_rule == ReadRule::Read
        || collection_view_rules.read_rule == ReadRule::OwnerRead
        || collection_view_rules.read_rule == ReadRule::GroupRead
            && field_view_rules.read_filters.contains(&FilterRule::NoLimit)
        || field_view_rules.read_filters.contains(&FilterRule::OnlyOwner)
        || field_view_rules.read_filters.contains(&FilterRule::OnlyGroup)
}
