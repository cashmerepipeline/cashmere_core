use dependencies_sync::{
    rust_i18n::{self, t},
    tonic::Status,
};
use manage_define::cashmere::Name;

/// 验证名称有效性
pub fn validate_name(name: &Option<Name>) -> Result<(), Status> {
    if name.is_none() {
        return Err(Status::invalid_argument(t!("名称不能为空").to_string()));
    }

    // 名称不为空
    if let Some(name) = name.as_ref() {
        if name.language.is_empty() {
            return Err(Status::invalid_argument(t!("语言不能为空").to_string()));
        }
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
