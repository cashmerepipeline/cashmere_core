use std::collections::HashMap;
use std::sync::Arc;

use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::tantivy::Index;

pub type ManageIndexMap = HashMap<i32, Arc<Index>>;

static mut MANAGE_TANTIVY_INDEX_MAP: Option<Arc<RwLock<ManageIndexMap>>> = None;

pub fn get_manage_tantivy_index_map() -> Arc<RwLock<ManageIndexMap>> {
    unsafe {
        if MANAGE_TANTIVY_INDEX_MAP.is_some() {
            return MANAGE_TANTIVY_INDEX_MAP.clone().unwrap();
        }

        let map = Arc::new(RwLock::new(HashMap::new()));
        MANAGE_TANTIVY_INDEX_MAP.replace(map);

        MANAGE_TANTIVY_INDEX_MAP.clone().unwrap()
    }
}
