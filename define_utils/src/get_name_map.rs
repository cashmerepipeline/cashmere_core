use dependencies_sync::bson::{self, Document};
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::toml;
use dependencies_sync::rust_i18n::{self, t};

use manage_define::general_field_names::NAME_MAP_FIELD_NAME;

/// 取得管理名
pub fn get_name_map(toml_map: &toml::map::Map<String, toml::Value>) -> Option<Document> {
    let value = toml_map
        .get(NAME_MAP_FIELD_NAME)
        .expect(format!("{}: {:?}", t!("取得管理名数据失败"), toml_map).as_str());

    let name_map: LinkedHashMap<String, String> =
        toml::from_str(&value.to_string()).expect("建立管理名数据表失败");

    match bson::to_document(&name_map) {
        Ok(r) => Some(r),
        Err(_e) => None,
    }
}
