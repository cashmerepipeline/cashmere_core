use crate::view_rule::ViewRule;
use crate::FilterRule;
use dependencies_sync::linked_hash_map::LinkedHashMap;

/// 取得无限制组
pub fn check_group_read_nolimit(rule: &ViewRule) -> bool {
    let result = rule.read_filters.contains(&FilterRule::NoLimit);
    result
}
