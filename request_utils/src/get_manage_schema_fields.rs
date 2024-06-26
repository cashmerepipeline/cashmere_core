use dependencies_sync::{log, rust_i18n::{self, t}, tonic::Status};

use cash_core::SchemaField;
use majordomo::get_majordomo;
use managers::{manager_trait::ManagerInterface};

pub async fn get_manage_schema_fields(manage_id: &str) -> Result<Vec<SchemaField>, Status> {
    let majordomo_arc = get_majordomo();
    let manager = if let Ok(r) = majordomo_arc.get_manager_by_id(manage_id) {
        r
    } else {
        log::error!("{}, {}", t!("取得管理器失败"), manage_id);
        return Err(Status::not_found(t!("管理器不存在").to_string()));
    };

    Ok(manager.get_manage_schema().await)
}
