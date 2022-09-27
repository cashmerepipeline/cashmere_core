use crate::view_rules_map::get_view_rules_map;
use crate::WriteRule;

/// 集合是否可写，向集合添加或者删除实体
pub async fn can_collection_write(
    _account: &String,
    group: &String,
    manage_id: &String,
) -> bool {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let field_opt = &view_rules
        .get(manage_id)
        .and_then(|rules| Some(&rules.collection))
        .or(None);

    let mut result = false;
    if let Some(field) = field_opt {
            field
                .get(group)
                .and_then(|rule| {
                    result = result
                        || rule.write_rule == WriteRule::Write
                        || rule.write_rule == WriteRule::OwnerWrite
                        || rule.write_rule == WriteRule::GroupWrite;
                    Some(())
                })
                .or(None);
    };

    result
}
