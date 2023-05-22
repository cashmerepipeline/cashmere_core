use linked_hash_map::LinkedHashMap;
use crate::FilterRule;
use crate::view_rule::ViewRule;

/// 取得可读组
pub fn check_group_read_group (
    rules: &Option<&LinkedHashMap<String, ViewRule>>,
    group: &String,
) -> bool {
    let mut result = false;
    rules.and_then(|rules_map| {
        rules_map.get(group)
            .map(|rule| {
                result = rule.read_filters.contains(&FilterRule::OnlyGroup);
                
            })
    });
    // let result = if let Some(rules) = rules {
    //     rules
    //         .get(group)
    //         .and_then(|rule| Some(rule.read_filters.contains(&FilterRule::OnlyGroup)))
    // };

    result
}
