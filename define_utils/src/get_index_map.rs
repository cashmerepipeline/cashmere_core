use dependencies_sync::bson::{self, Document};

use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::toml;

use manage_define::hard_coded_field_names::INDEX_MAP_FIELD_NAME;

/// 取得管理名
pub fn get_index_map(toml_map: &toml::map::Map<String, toml::Value>) -> Option<Document> {
    let value = toml_map.get(INDEX_MAP_FIELD_NAME).unwrap_or_else(|| {
        panic!("{}: {:?}", t!("取得索引数据失败"), toml_map);
    });

    match bson::to_document(&value) {
        Ok(r) => Some(r),
        Err(_e) => None,
    }
}
