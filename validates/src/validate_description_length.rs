use dependencies_sync::bson::doc;
use dependencies_sync::rust_i18n::{self, t};

use majordomo::get_majordomo;

use dependencies_sync::tonic::Status;

use manage_define::general_field_ids::ID_FIELD_ID;
use managers::ManagerTrait;

/// zh: 验证描述字符串长度 
pub fn validate_description_length(description: &str) -> Result<(), Status> {
    if description.len() > 2000 {
        return Err(Status::invalid_argument(format!(
            "{}",
            t!("描述长度不能超过2000个字符"),
        )));
    }
    Ok(())
}
