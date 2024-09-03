use dependencies_sync::{
    rust_i18n::{self, t},
    tonic::Status,
};
use manage_define::{cashmere::Name, manage_ids::LANGUAGE_CODES_MANAGE_ID};

use crate::validate_entity_id;

/// 验证名称有效性
pub async fn validate_name(name: Option<&Name>) -> Result<(), Status> {
    if name.is_none() {
        return Err(Status::invalid_argument(t!("名称不能为空").to_string()));
    }

    // 名称不为空
    if let Some(name) = name.as_ref() {
        validate_entity_id(LANGUAGE_CODES_MANAGE_ID, &name.language.as_str()).await?;
        if name.name.is_empty() {
            return Err(Status::invalid_argument(t!("名字不能为空").to_string()));
        }
        if name.name.len() > 100 {
            return Err(Status::invalid_argument(t!("名字不能超过100个字符").to_string()));
        }
        if name.name.len() < 2 {
            return Err(Status::invalid_argument(t!("名字不能少于2个字符").to_string()));
        }
    }

    Ok(())
}
