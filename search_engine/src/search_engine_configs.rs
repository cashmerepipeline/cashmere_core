use dependencies_sync::serde::{Deserialize, Serialize};
use configs::ConfigTrait;

pub const SEARCH_ENGINE_CONFIG: &str = "SearchEngineConfig";

/// 搜索引擎设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngineConfigs{
  // zh: 索引存储位置
  pub index_root_dir: String,
  // zh: 提交时间间隔，根据修改繁忙程度设置
  pub writer_commit_interval: u64,
  // zh: 写如用缓存块？
  pub memory_budget_in_bytes: usize,
}

impl ConfigTrait for SearchEngineConfigs{}