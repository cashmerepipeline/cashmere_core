use crate::view_rules_map::{get_view_rules_map, query_field_view_rules};
use crate::WriteRule;

/// 实体的可写性，可否修改实体的字段
pub async fn can_field_write(manage_id: &String, field_id: &String, role_group: &String) -> bool {
    let view_rules = if let Some(r) = query_field_view_rules(manage_id, field_id, role_group).await
    {
        r
    } else {
        return false;
    };

    view_rules.write_rule == WriteRule::Write
        || view_rules.write_rule == WriteRule::OwnerWrite
        || view_rules.write_rule == WriteRule::GroupWrite
}
