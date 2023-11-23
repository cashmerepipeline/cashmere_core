use dependencies_sync::bson::doc;
use dependencies_sync::rust_i18n::{self, t};

use majordomo::get_majordomo;

use manage_define::general_field_ids::ID_FIELD_ID;
use managers::ManagerTrait;

use dependencies_sync::tonic::Status;

/// zh: 验证管理编号
pub async fn validate_manage_id(manage_id: &i32) -> Result<(), Status> {
    let majordomo = get_majordomo();
    if let Err(e) = majordomo.get_manager_by_id(*manage_id){
      return Err(Status::invalid_argument(format!("{}: {:?} ", t!("无效管理编号"), e)));
    };

    Ok(())
}
