use dependencies_sync::toml;

/// 拷贝自 utils
/// 取得管理id
pub fn get_login_required(toml_map: &toml::map::Map<String, toml::Value>) -> Option<bool> {
    let result = toml_map.get("login_required");
    result.map(|v| v.as_bool().unwrap())
}
