use bson::Document;
use linked_hash_map::LinkedHashMap;

/// 取得属性名表
pub fn get_name_map(toml_map: &toml::map::Map<String, toml::Value>) -> Option<Document> {
    let value = toml_map.get("name_map").expect("取得管理名数据失败");

    let name_map: LinkedHashMap<String, String> =
        toml::from_str(&value.to_string()).expect("建立管理名数据表失败");

    match bson::to_document(&name_map) {
        Ok(r) => Some(r),
        Err(_e) => None,
    }
}
