use crate::view_rules_map::get_view_rules_map;
use crate::ReadRule;

/// 集合是否可写，向集合添加或者删除实体
pub async fn can_collection_read(
    _account: &String,
    role_group: &String,
    manage_id: &String,
) -> bool {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let mut result = false;
    view_rules
        .get(manage_id)
        .and_then(|rules| Some(&rules.collection))
        .and_then(|rules_map| {
            rules_map
                .get(role_group)
                .and_then(|rule| {
                    result = result
                        || rule.read_rule == ReadRule::Read
                        || rule.read_rule == ReadRule::OwnerRead
                        || rule.read_rule == ReadRule::GroupRead;
                    Some(())
                })
                .or(None)
        })
        .or(None);

    result
}
