use dependencies_sync::toml;

/// 拷贝自 utils
/// 取得管理id
pub fn get_id(toml_map: &toml::map::Map<String, toml::Value>) -> Option<i32> {
    let result = toml_map.get("id");
    result.map(|v| v.as_integer().unwrap() as i32)
}
