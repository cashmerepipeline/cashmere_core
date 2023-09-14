use manage_define::cashmere::Name;

/// 验证名称有效性
pub fn validate_name(name: &Option<Name>) ->  bool {
    if name.is_none() {
        return false;
    }

    // 名称不为空
    if let Some(name) = name.as_ref() {
        if name.language.is_empty() || name.name.is_empty() {
            return false;
        }
        true
    } else {
        false
    }
}
