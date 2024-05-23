use dependencies_sync::{
    log,
    rust_i18n::{self, t},
    toml,
};
use serde::Serialize;

use super::get_configs_map;

/// 设置序列化为toml格式存储
pub fn register_config(config_name: &str, config: &impl Serialize) {
    let map_arc = get_configs_map();
    let mut map = map_arc.write();

    let doc = if let Ok(r) = toml::Table::try_from(config) {
        r
    } else {
        log::error!("{}, {}", t!("转换设置错误"), config_name);
        panic!("{}, {}", t!("转换设置错误"), config_name);
    };

    map.insert(config_name.to_owned(), doc);
}
