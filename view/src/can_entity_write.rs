use crate::view_rules_map::get_view_rules_map;
use crate::FilterRule;

/// 管理是否可写, 管理的字段定义添加删除
pub async fn can_entity_write(_account: &String, groups: &Vec<String>, manage_id: &String) -> bool {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let rule_option = &view_rules
        .get(manage_id)
        .and_then(|rules| Some(&rules.collection))
        .or(None);

    // 没有指定规则则不能访问
    let mut result = false;

    if let Some(rule) = rule_option {
        groups.iter().for_each(|group| {
            rule.get(group)
                .and_then(|rule| {
                    result = result
                        || rule.write_filters.contains(&FilterRule::OnlyOwner)
                        || rule.write_filters.contains(&FilterRule::OnlyGroup);
                    Some(())
                })
                .or(None);
        });
    };

    result
}
