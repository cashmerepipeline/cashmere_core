use log::debug;
use crate::view_rules_map::get_view_rules_map;
use crate::FilterRule;

/// 检查实体是否可写
pub async fn can_entity_write(_account: &String, group: &String, manage_id: &String) -> bool {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    // 没有指定规则则不能访问
    let mut result = false;
    view_rules
        .get(manage_id)
        .and_then(|rules| Some(&rules.collection))
        .and_then(|rules_map| {
            rules_map
                .get(group)
                .and_then(|rule| {
                    debug!("{}：{}-{}-{:?}", "检查权限", manage_id, group, rule);
                    result = result
                        || rule.write_filters.contains(&FilterRule::NoLimit)
                        || rule.write_filters.contains(&FilterRule::OnlyOwner)
                        || rule.write_filters.contains(&FilterRule::OnlyGroup);
                    Some(())
                })
                .or(None)
        })
        .or(None);

    result
}
