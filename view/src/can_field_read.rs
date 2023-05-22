use crate::view_rules_map::get_view_rules_map;
use crate::FilterRule;
use crate::ReadRule;

/// 实体的可写性，可否修改实体的字段
pub async fn can_field_read(
    _account: &String,
    group: &String,
    manage_id: &String,
    field_id: &String,
) -> bool {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let rule_option = &view_rules
        .get(manage_id)
        .and_then(|rules| rules.schema.get(field_id))
        .or(None);

    let mut result = false;
    if let Some(field) = rule_option {
            field
                .get(group)
                .map(|rule| {
                    result = result
                        || rule.read_rule == ReadRule::Read
                        || rule.read_rule == ReadRule::OwnerRead
                        || rule.read_rule == ReadRule::GroupRead;
                    
                })
                .or(None);
    };

    // 过滤项
    if let Some(rule) = rule_option {
            rule.get(group)
                .map(|rule| {
                    result = rule.read_filters.contains(&FilterRule::NoLimit)
                        || rule.read_filters.contains(&FilterRule::OnlyOwner)
                        || rule.read_filters.contains(&FilterRule::OnlyGroup);
                    
                })
                .or(None);
    };


    result
}
