use crate::view_rules_map::get_view_rules_map;
use crate::WriteRule;

/// 实体的可写性，可否修改实体的字段
pub async fn can_field_write(
    _account: &String,
    groups: &Vec<String>,
    manage_id: &String,
    field_id: &String,
) -> bool {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let field_opt = &view_rules
        .get(manage_id)
        .and_then(|rules| rules.schema.get(field_id))
        .or(None);

    let mut result = false;
    if let Some(field) = field_opt {
        groups.iter().for_each(|group| {
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
        });
    };

    result
}