
use dependencies_sync::rust_i18n::{self, t};

use majordomo::get_majordomo;




use dependencies_sync::tonic::Status;

/// zh: 验证管理编号
pub async fn validate_manage_id(manage_id: &str) -> Result<(), Status> {
    if manage_id.is_empty(){
      return Err(Status::invalid_argument(format!("{}: {:?} ", t!("管理编号不能为空"), manage_id)));
    };

    let majordomo = get_majordomo();
    if let Err(e) = majordomo.get_manager_by_id(manage_id){
      return Err(Status::invalid_argument(format!("{}: {:?} ", t!("无效管理编号"), e)));
    };

    Ok(())
}
