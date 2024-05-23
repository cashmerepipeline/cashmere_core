use std::collections::HashMap;

use dependencies_sync::rust_i18n::{self, t};

use dependencies_sync::tonic::Status;

/// zh: 验证描述字符串长度
pub fn validate_description_length(description: &HashMap<String, String>) -> Result<(), Status> {
    if description.is_empty() {
        return Ok(())
    }

    let r = description.iter().any(|(_lang, v)| v.len() > 2000);

    if r {
        return Err(Status::invalid_argument(
            t!("描述长度不能超过2000个字符").to_string().to_string(),
        ));
    }

    Ok(())
}
