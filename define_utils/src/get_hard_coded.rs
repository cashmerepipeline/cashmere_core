use manage_define::hard_coded_field_names::{HARD_CODED_FIELD_NAME};
use dependencies_sync::toml;

/// 取得管理id
pub fn get_hard_coded(toml_map: &toml::map::Map<String, toml::Value>) -> Option<bool> {
    let result = toml_map.get(HARD_CODED_FIELD_NAME);

    result.map(|v| v.as_bool().unwrap())
}

