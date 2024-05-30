use crate::view_rules_map::query_collection_view_rules;
use crate::FilterRule;
use dependencies_sync::log::debug;
use dependencies_sync::rust_i18n::{self, t};

/// 检查实体是否可写
// TODO: 需要独立检查是否为主
pub async fn can_entity_write(
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
    debug!(
        "{}: {}-{}-{:?}",
        t!("实体可写检查权限"),
        manage_id,
        role_group,
        view_rules
    );

    if view_rules.write_filters.contains(&FilterRule::NoLimit) {
        return true;
    }

    let entity = if let Ok(r) = entity::get_entity_by_id(manage_id, entity_id, &[], &[]).await {
        r
    } else {
        return false;
    };

    // 组可写，判断实体的所属组
    if view_rules.write_filters.contains(&FilterRule::OnlyGroup) {
        return match entity::get_entity_groups(&entity) {
            Some(groups) => groups.contains(&role_group.to_string()),
            None => false,
        };
    }

    // 只主人可写
    if view_rules.write_filters.contains(&FilterRule::OnlyOwner) {
        return match entity::get_entity_owner(&entity) {
            Some(owner) => owner == account_id,
            None => false,
        };
    }

    false
}
