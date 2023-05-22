use crate::view_rules_map::get_view_rules_map;
use crate::WriteRule;

/// 集合是否可写，向集合添加或者删除实体
pub async fn can_collection_write(_account: &String, group: &String, manage_id: &String) -> bool {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let mut result = false;
    view_rules
        .get(manage_id).map(|rules| &rules.collection)
        .and_then(|rules_map| {
            rules_map.get(group).map(|rule| {
                result = result
                    || rule.write_rule == WriteRule::Write
                    || rule.write_rule == WriteRule::OwnerWrite
                    || rule.write_rule == WriteRule::GroupWrite;
                
            })
        })
        .or(None);

    result
}
