
use manage_define::general_field_ids::REMOVED_FIELD_ID;
use manage_define::manage_ids::GROUPS_MANAGE_ID;

/// 检查组是否有效
pub async fn validate_group(group_id: &String) -> bool {
    // 存在
    match entity::get_entity_by_id(&GROUPS_MANAGE_ID.to_string(), group_id).await {
        Ok(group_entity) => {
            // 没有移除
            !group_entity.get_bool(REMOVED_FIELD_ID.to_string()).unwrap()
        }
        Err(..) => {
            false
        }
    }
}
