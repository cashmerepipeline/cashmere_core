use manage_define::general_field_ids::{
    CREATE_TIMESTAMP_FIELD_ID, ID_FIELD_ID, MODIFIER_FIELD_ID, MODIFY_TIMESTAMP_FIELD_ID,
};
use manage_define::system_fields::get_system_fields;

use crate::view_rules_map::query_field_view_rules;
use crate::WriteRule;

/// 实体的可写性，可否修改实体的字段
pub async fn can_field_write(
    manage_id: &str,
    field_id: &str,
    manage_hard_coded: bool,
    role_group: &String,
) -> bool {
    let view_rules = if let Some(r) =
        query_field_view_rules(&manage_id.to_string(), field_id, role_group).await
    {
        r
    } else {
        // zh: 默认不可写，不存在则不可写
        return false;
    };

    // zh: 硬编码管理 ID 字段 admin组可写，其他情况ID字段不可写
    if field_id == ID_FIELD_ID.to_string().as_str() {
        if !manage_hard_coded {
            return false;
        }
    }

    // zh: 系统字段，不可写
    let system_fields = get_system_fields();
    if system_fields.contains(&field_id.to_string()) {
        return false;
    }

    view_rules.write_rule == WriteRule::Write
        || view_rules.write_rule == WriteRule::OwnerWrite
        || view_rules.write_rule == WriteRule::GroupWrite
}
