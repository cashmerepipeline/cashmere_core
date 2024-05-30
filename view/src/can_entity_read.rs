use crate::view_rules_map::query_collection_view_rules;
use crate::FilterRule;

/// 实体是否可读
pub async fn can_entity_read(
    manage_id: &str,
    entity_id: &str,
    account_id: &str,
    role_group: &str,
) -> bool {
    let view_rules = if let Some(r) = query_collection_view_rules(manage_id, role_group).await {
        r
    } else {
        return false;
    };

    if view_rules.read_filters.contains(&FilterRule::NoLimit) {
        return true;
    }

    let entity = if let Ok(r) = entity::get_entity_by_id(manage_id, entity_id, &[], &[]).await {
        r
    } else {
        return false;
    };

    // 组可读，判断实体的所属组
    if view_rules.read_filters.contains(&FilterRule::OnlyGroup) {
        return match entity::get_entity_groups(&entity) {
            Some(groups) => groups.contains(&role_group.to_string()),
            None => false,
        };
    }

    // 只主人可读
    if view_rules.read_filters.contains(&FilterRule::OnlyOwner) {
        return match entity::get_entity_owner(&entity) {
            Some(owner) => owner == account_id,
            None => false,
        };
    }

    false
}
