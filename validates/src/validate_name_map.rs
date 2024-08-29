use std::collections::HashMap;

use dependencies_sync::{
    rust_i18n::{self, t},
    tonic::Status,
};
use majordomo::get_majordomo;
use manage_define::{cashmere::Name, manage_ids::LANGUAGE_CODES_MANAGE_ID};

use crate::validate_entity_id;

/// 验证名称有效性
pub async fn validate_name_map(name_map: Option<&HashMap<String, String>>) -> Result<(), Status> {
    if name_map.is_none() {
        return Err(Status::invalid_argument(t!("名称不能为空").to_string()));
    }

    for (lang, name) in name_map.unwrap() {
        validate_entity_id(LANGUAGE_CODES_MANAGE_ID, lang).await?;
        // 名称不为空
        if name.is_empty() {
            return Err(Status::invalid_argument(t!("名字不能为空").to_string()));
        }
        if name.len() > 100 {
            return Err(Status::invalid_argument(
                t!("名字不能超过100个字符").to_string(),
            ));
        }
        if name.len() < 2 {
            return Err(Status::invalid_argument(
                t!("名字不能少于2个字符").to_string(),
            ));
        }
    }

    Ok(())
}
