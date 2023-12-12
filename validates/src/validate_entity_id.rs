
use dependencies_sync::rust_i18n::{self, t};

use majordomo::get_majordomo;

use dependencies_sync::tonic::Status;

use manage_define::general_field_ids::{REMOVED_FIELD_ID};
use managers::ManagerTrait;

/// zh: 验证目标实体存在性
pub async fn validate_entity_id(manage_id: &i32, entity_id: &String) -> Result<(), Status> {
    if entity_id.is_empty() {
        return Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("实体编号为空"),
            entity_id
        )));
    }

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();
    match manager.get_entity_by_id(entity_id, &vec![]).await {
        Ok(r) => {
            if r.get_bool(REMOVED_FIELD_ID.to_string()).unwrap_or(true) {
                Err(Status::invalid_argument(format!(
                    "{}: {}-{}, {}",
                    t!("实体已删除"),
                    manage_id,
                    entity_id,
                    "validate_entity_id"
                )))
            } else {
                Ok(())
            }
        }
        Err(_e) => {
            Err(Status::invalid_argument(format!(
                "{}: {}-{}, {}",
                t!("实体不存在"),
                manage_id,
                entity_id,
                "validate_entity_id"
            )))
        }
    }
}
