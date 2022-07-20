use crate::view_rules_map::get_view_rules_map;
use crate::ReadRule;

/// 集合是否可写，向集合添加或者删除实体
pub async fn get_collection_read_rule(
    _account: &String,
    groups: &Vec<String>,
    manage_id: &String,
) -> Vec<String> {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let field_opt = &view_rules
        .get(manage_id)
        .and_then(|rules| Some(&rules.collection))
        .or(None);

    let mut result = vec![];
    if let Some(field) = field_opt {
        groups.iter().for_each(|group| {
            field
                .get(group)
                .and_then(|rule| {
                    result = result
                        || rule.read_rule == ReadRule::Read
                        || rule.read_rule == ReadRule::OwnerRead
                        || rule.read_rule == ReadRule::GroupRead;
                    Some(())
                })
                .or(None);
        });
    };

    result
}


