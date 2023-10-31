use std::fmt::Debug;

use dependencies_sync::{
    log,
    rust_i18n::{self, t},
};
use serde::{Deserialize, Serialize};

use crate::ConfigTrait;

use super::{get_configs_map, register_config};

pub fn get_config<T>() -> Option<T>
where
    T: for<'de> Deserialize<'de> + Debug + Default + Clone + Serialize + ConfigTrait,
{
    let config_name = &T::name().to_string();
    let map_arc = get_configs_map();
    {
        let map = map_arc.read();

        if let Some(t) = map.get(config_name).cloned() {
            log::info!("{}: {:?}", t!("配置加载成功"), t);
            let config = t.try_into().expect("配置格式错误");
            return Some(config);
        };
    }

    let new_config = {
        log::warn!("{}: {}", t!("配置不存在"), config_name);
        log::warn!("{}: {:?}", t!("使用默认配置"), config_name);
        let config: T = T::default();
        config
    };
    register_config(config_name, &new_config);
    Some(new_config)
}
