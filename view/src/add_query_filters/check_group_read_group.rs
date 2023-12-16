use crate::view_rule::ViewRule;
use crate::FilterRule;

/// 取得可读组
pub fn check_group_read_group(rule: &ViewRule) -> bool {
    rule.read_filters.contains(&FilterRule::OnlyGroup)
}
