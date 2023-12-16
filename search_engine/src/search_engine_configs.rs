use std::sync::OnceLock;

use configs::{ConfigTrait, get_config};
use dependencies_sync::rust_i18n::{self, t};
use serde::{Deserialize, Serialize};

pub const SEARCH_ENGINE_CONFIG_NAME: &str = "search_engine";

static SEARCH_ENGINE_CONFIGS: OnceLock<SearchEngineConfigs> = OnceLock::new();

/// 搜索引擎设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngineConfigs {
    // zh: 索引存储位置
    pub index_root_dir: String,
    // zh: 提交时间间隔，根据修改繁忙程度设置，单位：秒
    pub writer_commit_interval: u64,
    // zh: 写入用缓存块？
    pub memory_budget_in_bytes: usize,
    // zh: 不可搜索管理
    pub unsearchable_manages: Vec<String>,
}

impl ConfigTrait for SearchEngineConfigs {
    fn name() -> &'static str {
        SEARCH_ENGINE_CONFIG_NAME
    }
    fn get() -> &'static Self {
        if let Some(configs) = SEARCH_ENGINE_CONFIGS.get() {
            return configs;
        } else {
            let configs = get_config::<SearchEngineConfigs>().expect(t!("取得配置失败").as_str());
            SEARCH_ENGINE_CONFIGS.set(configs).expect("设置配置失败");
        }

        SEARCH_ENGINE_CONFIGS.get().unwrap()
    }
}

impl Default for SearchEngineConfigs {
    fn default() -> Self {
        SearchEngineConfigs {
            index_root_dir: "tantivy".to_string(),
            writer_commit_interval: 15,
            memory_budget_in_bytes: 15_000_000,
            unsearchable_manages: [].to_vec(),
        }
    }
}
