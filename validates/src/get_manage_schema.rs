use dependencies_sync::{log, rust_i18n::{self, t}, tonic::Status, chrono::format::format};
use majordomo::get_majordomo;
use cash_core::PropertyField;
use managers::ManagerTrait;

pub async fn get_manage_schema_fields(manage_id: &i32) -> Result<Vec<PropertyField>, Status> {
    let majordomo_arc = get_majordomo();
    let manager = if let Ok(r) = majordomo_arc.get_manager_by_id(*manage_id) {
        r
    } else {
        log::error!("{}, {}", t!("取得管理器失败"), manage_id);
        return Err(Status::not_found(format!("{}", t!("管理器不存在"))));
    };

    Ok(manager.get_manage_schema().await)
}
