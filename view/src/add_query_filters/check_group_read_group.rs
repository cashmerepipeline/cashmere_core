use crate::view_rule::ViewRule;
use crate::FilterRule;
use dependencies_sync::linked_hash_map::LinkedHashMap;

/// 取得可读组
pub fn check_group_read_group(rule: &ViewRule) -> bool {
    let mut result = rule.read_filters.contains(&FilterRule::OnlyGroup);
    result
}
