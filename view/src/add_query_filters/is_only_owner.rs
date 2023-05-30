use dependencies_sync::linked_hash_map::LinkedHashMap;
use crate::FilterRule;
use crate::view_rule::ViewRule;

/// 判断是否只主人可读
pub fn is_only_owner(rules: &Option<&LinkedHashMap<String, ViewRule>>, groups: &Vec<String>) -> bool {
    if let Some(rules) = rules {
        let result: Vec<String> = groups
            .iter()
            .map(|id| id.to_owned())
            .filter(|group| {
                rules
                    .get(group).map(|rule| rule.read_filters.contains(&FilterRule::OnlyOwner))
                    .unwrap()
            })
            .collect();

        return !result.is_empty()
    }
    false
}
