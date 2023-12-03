
use dependencies_sync::rust_i18n::{self, t};



use dependencies_sync::tonic::Status;




/// zh: 验证描述字符串长度
pub fn validate_description_length(description: &str) -> Result<(), Status> {
    if description.len() > 2000 {
        return Err(Status::invalid_argument(
            t!("描述长度不能超过2000个字符").to_string().to_string(),
        ));
    }
    Ok(())
}
