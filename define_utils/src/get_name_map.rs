use dependencies_sync::bson::{self, Document};

use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::toml;

use manage_define::hard_coded_field_names::NAME_MAP_FIELD_NAME;

/// 取得管理名
pub fn get_name_map(toml_map: &toml::map::Map<String, toml::Value>) -> Option<Document> {
    let value = toml_map.get(NAME_MAP_FIELD_NAME).unwrap_or_else(|| {
        panic!("{}: {:?}", t!("取得管理名数据失败"), toml_map);
    });


    match bson::to_document(&value) {
        Ok(r) => Some(r),
        Err(_e) => None,
    }
}
