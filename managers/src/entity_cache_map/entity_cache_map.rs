use std::collections::BTreeMap;
use std::sync::Arc;

use dependencies_sync::bson::Document;
use dependencies_sync::parking_lot::RwLock;

// 注意： 只用于少量常用数据的缓存，不要缓存大量数据
// 只支持2种取得访问方式
type EntityCacheMap = BTreeMap<i32, Arc<RwLock<BTreeMap<String, Arc<Document>>>>>;

static mut ENTITY_CACHE_MAP: Option<Arc<RwLock<EntityCacheMap>>> = None;

pub fn get_entity_cache_map() -> Arc<RwLock<EntityCacheMap>> {
    unsafe {
        if ENTITY_CACHE_MAP.is_none() {
            ENTITY_CACHE_MAP.replace(init_managers_map());
            ENTITY_CACHE_MAP.clone().unwrap()
        } else {
            ENTITY_CACHE_MAP.clone().unwrap()
        }
    }
}

/// 初始化管理器映射表
fn init_managers_map() -> Arc<RwLock<EntityCacheMap>> {
    let c_map = RwLock::new(BTreeMap::new());
    Arc::new(c_map)
}
