use dependencies_sync::bson::doc;
use dependencies_sync::rust_i18n::{self, t};

use majordomo::get_majordomo;

use dependencies_sync::tonic::Status;

use manage_define::general_field_ids::ID_FIELD_ID;
use managers::ManagerTrait;

/// zh: 验证编辑的目标实体存在性 
pub(crate) async fn validate_edit_entity_id(manage_id: &i32, entity_id: &String) -> Result<(), Status> {
    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();
    if manager.entity_exists(&doc! {
        ID_FIELD_ID.to_string(): entity_id,
    }).await.is_none(){
        return Err(Status::invalid_argument(format!(
            "{}: {}-{}, {}",
            t!("实体不存在"),
            manage_id,
            entity_id,
            "validate_edit_entity_id"
        )));
    }
    
    Ok(())
}
