use configs::ConfigTrait;
use serde::{Deserialize, Serialize};

pub const SEARCH_ENGINE_CONFIG_NAME: &str = "search_engine";

/// 搜索引擎设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngineConfigs {
    // zh: 索引存储位置
    pub index_root_dir: String,
    // zh: 提交时间间隔，根据修改繁忙程度设置
    pub writer_commit_interval: u64,
    // zh: 写如用缓存块？
    pub memory_budget_in_bytes: usize,
}

impl ConfigTrait for SearchEngineConfigs {
    fn name() -> &'static str {
        SEARCH_ENGINE_CONFIG_NAME
    }
}

impl Default for SearchEngineConfigs {
    fn default() -> Self {
        SearchEngineConfigs {
            index_root_dir: "tantivy".to_string(),
            writer_commit_interval: 15,
            memory_budget_in_bytes: 15_000_000,
        }
    }
}
