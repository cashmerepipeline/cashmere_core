
use std::fmt::Debug;

use dependencies_sync::{
    log,
    rust_i18n::{self, t},
};
use serde::Deserialize;

use super::get_configs_map;

pub fn get_config<T>(config_name: &String) -> Option<T>
where
    T: for<'de> Deserialize<'de> + Debug,
{
    let map_arc = get_configs_map();
    let map = map_arc.read();

    let config: T = if let Some(t) = map.get(config_name).cloned() {
        log::info!("{}: {:?}", t!("配置加载成功"), t);
        t.try_into().expect("配置格式错误")
    } else {
        log::error!("{}: {}", t!("配置不存在"), config_name);
        return None;
    };

    Some(config)
}
