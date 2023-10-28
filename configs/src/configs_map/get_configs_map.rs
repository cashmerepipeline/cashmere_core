use std::{collections::HashMap, sync::Arc};

use dependencies_sync::{parking_lot::RwLock, toml};

type ConfigMap = Arc<RwLock<HashMap<&str, toml::Value>>>;

/// 启动加载后一次完成，不支持动态加载
static CONFIGS_MAP: Option<ConfigMap> = None;

pub fn get_configs_map() -> ConfigMap {
    unsafe {
        CONFIGS_MAP.unwrap_or_else(|| {
            let configs_map = Arc::new(RwLock::new(HashMap::new()));
            CONFIGS_MAP = Some(configs_map.clone());
            configs_map
        })
    }
}
