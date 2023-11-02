use configs::get_config;
use dependencies_sync::rust_i18n::{self, t};

use crate::search_engine_configs::SearchEngineConfigs;

pub fn get_tantivy_index_dir() -> String {
    // let root_dir = &get_configs().database.search_engine_index_root;
    if let Some(c) = get_config::<SearchEngineConfigs>() {
        // let index_dir = format!("{}/{}", c.index_root_dir, manage_id);
        c.index_root_dir.clone()
    } else {
        panic!("{}", t!("取得搜索引擎设置失败"));
    }
}
