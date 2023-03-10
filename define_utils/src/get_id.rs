use property_field::general_field_names::ID_FIELD_NAME;

/// 取得管理id
pub fn get_id(toml_map: &toml::map::Map<String, toml::Value>) -> Option<i32> {
    let result = toml_map.get(ID_FIELD_NAME);
    result.map(|v| v.as_integer().unwrap() as i32)
}
