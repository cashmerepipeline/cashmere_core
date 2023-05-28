use dependencies_sync::log::debug;
use crate::view_rules_map::get_view_rules_map;
use crate::FilterRule;

/// 检查实体是否可写
// TODO: 需要独立检查是否为主
pub async fn can_entity_write(
    manage_id: &String,
    role_group: &String, 
) -> bool {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    // 没有指定规则则不能访问
    let mut result = false;
    view_rules
        .get(manage_id).map(|rules| &rules.collection)
        .and_then(|rules_map| {
            rules_map
                .get(role_group)
                .map(|rule| {
                    debug!("{}：{}-{}-{:?}", t!("实体可写检查权限"), manage_id, role_group, rule);
                    result = result
                        || rule.write_filters.contains(&FilterRule::NoLimit)
                        || rule.write_filters.contains(&FilterRule::OnlyOwner)
                        || rule.write_filters.contains(&FilterRule::OnlyGroup);
                    
                })
                .or(None)
        })
        .or(None);

    result
}
