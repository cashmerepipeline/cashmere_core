use std::{collections::HashMap, sync::Arc};

use dependencies_sync::parking_lot::RwLock;

type ConfigMap = Arc<RwLock<HashMap<String, toml::Table>>>;

/// 启动加载后一次完成，不支持动态加载
static mut CONFIGS_MAP: Option<ConfigMap> = None;

pub fn get_configs_map() -> ConfigMap {
    unsafe {
        if CONFIGS_MAP.is_some() {
            return CONFIGS_MAP.as_ref().unwrap().clone();
        } else {
            let configs_map = Arc::new(RwLock::new(HashMap::new()));
            CONFIGS_MAP.replace(configs_map.clone());
            return configs_map;
        }
    }
}
