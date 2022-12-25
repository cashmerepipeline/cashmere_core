use crate::view_rules_map::get_view_rules_map;
use crate::WriteRule;

/// 实体的可写性，可否修改实体的字段
pub async fn can_field_write(
    _account: &String,
    group: &String,
    manage_id: &String,
    field_id: &String,
) -> bool {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let mut result = false;
    view_rules
        .get(manage_id)
        .and_then(|rules| rules.schema.get(field_id))
        .and_then(|rules_map| {
            rules_map
                .get(group)
                .and_then(|rule| {
                    // println!("{:?}", rule);
                    result = result
                        || rule.write_rule == WriteRule::Write
                        || rule.write_rule == WriteRule::OwnerWrite
                        || rule.write_rule == WriteRule::GroupWrite;
                    Some(())
                })
                .or(None)
        })
        .or(None);

    result
}
