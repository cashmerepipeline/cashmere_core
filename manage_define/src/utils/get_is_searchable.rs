use dependencies_sync::toml;

/// 拷贝自 utils
/// 取得管理id
pub fn get_is_searchable(toml_map: &toml::map::Map<String, toml::Value>) -> Option<bool> {
    let result = toml_map.get("get_is_searchable");
    result.map(|v| v.as_bool().unwrap())
}
