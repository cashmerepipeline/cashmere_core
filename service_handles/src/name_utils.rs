use crate::cashmere::Name;

/// 验证名称有效性
pub fn validate_name(name: &Option<Name>) -> Result<bool, bool> {
    if let name = name.as_ref().unwrap() {
        if name.language.is_empty() || name.name.is_empty() {
            return Err(false);
        }
        return Ok(true);
    } else {
        return Err(false);
    }
}