use dependencies_sync::rust_i18n::{self, t};

use majordomo::get_majordomo;
use managers::ManagerTrait;

use dependencies_sync::tonic::Status;

/// zh: 验证管理编号
pub async fn validate_manage_hard_coded(manage_id: &str) -> Result<(), Status> {
    let majordomo = get_majordomo();
    let manager = match majordomo.get_manager_by_id(manage_id) {
        Ok(r) => r,
        Err(err) => {
            return Err(Status::invalid_argument(format!(
                "{}: {} ",
                t!("无效管理编号"),
                err.details()
            )))
        }
    };

    if manager.is_hard_coded().await {
        Ok(())
    } else {
        Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("管理不是硬编码的"),
            manage_id
        )))
    }
}
