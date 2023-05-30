use dependencies_sync::linked_hash_map::LinkedHashMap;
use crate::FilterRule;
use crate::view_rule::ViewRule;

/// 取得无限制组
pub fn check_group_read_nolimit(
    rules: &Option<&LinkedHashMap<String, ViewRule>>,
    group: &String,
) -> bool {
    let mut result = false;
    rules.and_then(|rules_map| {
        rules_map.get(group)
            .map(|rule| {
                result = rule.read_filters.contains(&FilterRule::NoLimit);
                
            })
    });
    // let result = if let Some(rules) = rules {
    //     rules.get(group)
    //         .and_then(|rule| Some(rule.read_filters.contains(&FilterRule::NoLimit)))
    //         .or(None)
    // };

    result
}
