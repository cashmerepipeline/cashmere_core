use linked_hash_map::LinkedHashMap;
use crate::FilterRule;
use crate::view_rule::ViewRule;

/// 取得可读组
pub fn get_readable_groups(
    rules: &Option<&LinkedHashMap<String, ViewRule>>,
    groups: &Vec<String>,
) -> Option<Vec<String>> {
    if let Some(rules) = rules {
        let result: Vec<String> = groups
            .iter()
            .map(|id| id.to_owned())
            .filter(|group| {
                rules
                    .get(group).map(|rule| rule.read_filters.contains(&FilterRule::OnlyGroup))
                    .unwrap()
            })
            .collect();

        if !result.is_empty() {
            return Some(result);
        } else {
            return None;
        }
    }

    None
}
