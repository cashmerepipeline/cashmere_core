use crate::view_rules_map::get_view_rules_map;
use crate::ReadRule;

/// 管理是否可写, 管理的字段定义添加删除
pub async fn can_manage_read(_account: &String, group: &String, manage_id: &String) -> bool {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let rule_option = &view_rules
        .get(manage_id)
        .and_then(|rules| Some(&rules.manage))
        .or(None);

    // 没有指定规则则不能访问
    let mut result = false;
    if let Some(rule) = rule_option {
            rule.get(group)
                .and_then(|rule| {
                    result = result
                        || rule.read_rule == ReadRule::Read
                        || rule.read_rule == ReadRule::OwnerRead
                        || rule.read_rule == ReadRule::GroupRead;
                    Some(())
                })
                .or(None);
    };

    result
}
