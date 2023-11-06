use crate::view_rule::ViewRule;
use crate::FilterRule;

/// 取得无限制组
pub fn check_group_read_nolimit(rule: &ViewRule) -> bool {
    rule.read_filters.contains(&FilterRule::NoLimit)
}
