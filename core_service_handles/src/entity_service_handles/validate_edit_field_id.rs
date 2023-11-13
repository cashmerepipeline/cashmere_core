use dependencies_sync::bson::doc;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::Status;

use majordomo::get_majordomo;
use managers::ManagerTrait;

/// zh: 返回管理的schema表, 
pub(crate) async fn validate_edit_field_id(manage_id: &i32, entity_id: &String, field_id: &String) -> Result<Vec<property_field::PropertyField>, Status> {
    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

    let fields = manager.get_manage_schema().await;
    if !manager.has_schema_field(field_id).await{
        return Err(Status::invalid_argument(format!(
            "{}: {}-{}, {}",
            t!("属性不存在"),
            manage_id,
            field_id,
            "validate_edit_field_id"
        )));
    }
    
    Ok(fields)
}
