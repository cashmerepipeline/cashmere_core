use dependencies_sync::log;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::serde::Serialize;

use crate::configs_map::get_configs_map;

pub trait ConfigTrait {
    fn name() -> &'static str;

    /// 设置序列化为toml格式存储
    fn register_config(config_name: &'static str, config: &impl Serialize) {
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
}
