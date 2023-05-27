use crate::view_rules_map::get_view_rules_map;
use crate::ReadRule;

/// 管理是否可写, 管理的字段定义添加删除
pub async fn can_manage_read(manage_id: &String, role_group: &String) -> bool {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    // 没有指定规则则不能访问
    let mut result = false;
    view_rules
        .get(manage_id)
        .map(|rules| &rules.manage)
        .and_then(|rules_map| {
            rules_map
                .get(role_group)
                .map(|rule| {
                    result = result
                        || rule.read_rule == ReadRule::Read
                        || rule.read_rule == ReadRule::OwnerRead
                        || rule.read_rule == ReadRule::GroupRead;
                })
                .or(None)
        })
        .or(None);

    result
}
