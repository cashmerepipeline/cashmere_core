use dependencies_sync::log;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::toml;

/// 拷贝自 utils
/// 取得管理id
pub fn get_is_searchable(manage_id: &str, toml_map: &toml::map::Map<String, toml::Value>) -> Option<bool> {
    let result = toml_map.get("is_searchable");
    if result.is_none() {
        log::error!("{}: {}", t!("管理没有定义is_searchable标记"), manage_id);
        panic!("{}: {}", t!("管理没有定义is_searchable标记"), manage_id);
    }
    result.map(|v| v.as_bool().unwrap())
}
