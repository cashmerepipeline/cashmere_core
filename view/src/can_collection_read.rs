use crate::view_rules_map::get_view_rules_map;
use crate::ReadRule;

/// 集合是否可写，向集合添加或者删除实体
pub async fn can_collection_read(
    manage_id: &String,
    role_group: &String,
) -> bool {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let mut result = false;
    view_rules
        .get(manage_id).map(|rules| &rules.collection)
        .and_then(|rules_map| {
            rules_map
                .get(role_group)
                .map(|rule| {
                    result = result
                        || rule.read_rule == ReadRule::Read
                        || rule.read_rule == ReadRule::OwnerRead
                        || rule.read_rule == ReadRule::GroupRead;
                    
                })
                .or(None)
        })
        .or(None);

    result
}
