use dependencies_sync::bson::doc;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::Status;

use majordomo::get_majordomo;
use manage_define::general_field_ids::ID_FIELD_ID;
use manage_define::manage_ids::GROUPS_MANAGE_ID;
use managers::ManagerTrait;

/// zh: 验证管理字段是否存在
pub async fn validate_group_id(group_id: &str) -> Result<(), Status> {
    let manage_id = GROUPS_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    let query_doc = doc!{ID_FIELD_ID.to_string(): group_id.to_string()};
    if manager.entity_exists(&query_doc).await.is_none(){
        return Err(Status::invalid_argument(format!(
            "{}: {}-{}, {}",
            t!("组不存在"),
            manage_id,
            group_id,
            "validate_group_id"
        )));
    }
    
    Ok(())
}

