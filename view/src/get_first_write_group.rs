use crate::view_rules_map::get_view_rules_map;
use crate::WriteRule;

/// 取得第一个可写组
pub async fn get_first_write_group(groups: &Vec<String>, manage_id: &String) -> Option<String> {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    for group in groups {
        let rule = &view_rules
            .get(manage_id)
            .unwrap()
            .collection
            .get(group)
            .unwrap()
            .write_rule;

        if rule == &WriteRule::Write
            || rule == &WriteRule::OwnerWrite
            || rule == &WriteRule::GroupWrite
        {
            return Some(group.clone());
        }
    }
    None
}
