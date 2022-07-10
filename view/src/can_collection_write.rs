use crate::view_rules_map::get_view_rules_map;
use crate::WriteRule;

/// 集合是否可写，向集合添加或者删除实体
pub async fn can_collection_write(
    _account: &String,
    groups: &Vec<String>,
    manage_id: &String,
) -> bool {
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
            return true;
        }
    }

    // 不在可写组中
    false
}
