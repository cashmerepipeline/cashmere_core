use crate::view_rule::ViewRule;
use crate::FilterRule;
use dependencies_sync::linked_hash_map::LinkedHashMap;

/// 判断是否只主人可读
pub fn check_group_read_only_owner(rule: &ViewRule) -> bool {
    let result = rule.read_filters.contains(&FilterRule::OnlyOwner);
    result
}
