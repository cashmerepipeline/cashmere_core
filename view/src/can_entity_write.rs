use crate::view_rules_map::query_collection_view_rules;
use crate::FilterRule;
use dependencies_sync::log::debug;

/// 检查实体是否可写
// TODO: 需要独立检查是否为主
pub async fn can_entity_write(manage_id: &String, role_group: &String) -> bool {
    let view_rules = if let Some(r) = query_collection_view_rules(manage_id, role_group).await {
        r
    } else {
        return false;
    };
    debug!(
        "{}：{}-{}-{:?}",
        t!("实体可写检查权限"),
        manage_id,
        role_group,
        view_rules
    );

    view_rules.write_filters.contains(&FilterRule::NoLimit)
        || view_rules.write_filters.contains(&FilterRule::OnlyOwner)
        || view_rules.write_filters.contains(&FilterRule::OnlyGroup)
}
