use crate::view_rules_map::get_view_rules_map;
use crate::WriteRule;

/// 管理是否可写, 管理的字段定义添加删除
pub async fn can_manage_write(_account: &String, groups: &Vec<String>, manage_id: &String) -> bool {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let rule_option = &view_rules
        .get(manage_id)
        .and_then(|rules| Some(&rules.manage))
        .or(None);

    let mut result = false;
    if let Some(rule) = rule_option {
        groups.iter().for_each(|group| {
            rule.get(group)
                .and_then(|rule| {
                    result = result
                        || rule.write_rule == WriteRule::Write
                        || rule.write_rule == WriteRule::OwnerWrite
                        || rule.write_rule == WriteRule::GroupWrite;
                    Some(())
                })
                .or(None);
        });
    };

    result
}
