use linked_hash_map::LinkedHashMap;
use crate::FilterRule;
use crate::view_rule::ViewRule;

/// 判断是否只主人可读
pub fn check_group_read_only_owner(rules: &Option<&LinkedHashMap<String, ViewRule>>, group: &String) -> bool {
    let mut result = false;
    rules.and_then(|rules_map| {
        rules_map.get(group)
            .map(|rule| {
                result = rule.read_filters.contains(&FilterRule::NoLimit);
                
            })
    });
    // let resutl = if let Some(rules) = rules {
    //     rules
    //         .get(group)
    //         .and_then(|rule| Some(rule.read_filters.contains(&FilterRule::OnlyOwner)))
    //         .or(None)
    // };
    result
}
