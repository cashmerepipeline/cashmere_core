use std::sync::{Arc, OnceLock};

use dependencies_sync::{indexmap::IndexMap, parking_lot::RwLock};

use super::HardCodedCacheMap;

/// zh: 管理硬编码缓存表: {manage_id: cache_map}
type ManageHardCodeCacheMap<'a> = Arc<RwLock<IndexMap<String, HardCodedCacheMap<'a>>>>;

/// 管理缓存表, {manage_id: cache_map}
static MANAGE_CACHE_MAP: OnceLock<ManageHardCodeCacheMap> = OnceLock::new();

/// zh: 获取管理缓存表
pub fn get_manage_hard_code_cache_map() -> ManageHardCodeCacheMap<'static> {
    MANAGE_CACHE_MAP
        .get_or_init(|| ManageHardCodeCacheMap::new(RwLock::new(IndexMap::new())))
        .clone()
}
