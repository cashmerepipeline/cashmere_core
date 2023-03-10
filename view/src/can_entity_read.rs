use crate::view_rules_map::get_view_rules_map;
use crate::FilterRule;

/// 实体是否可读
pub async fn can_entity_read(_account: &String, group: &String, manage_id: &String) -> bool {
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
                    result = result
                        || rule.read_filters.contains(&FilterRule::NoLimit)
                        || rule.read_filters.contains(&FilterRule::OnlyOwner)
                        || rule.read_filters.contains(&FilterRule::OnlyGroup);
                    Some(())
                })
                .or(None)
        })
        .or(None);

    result
}
