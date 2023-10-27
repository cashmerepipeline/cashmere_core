use dependencies_sync::log;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::tantivy::IndexWriter;
use std::collections::HashMap;
use std::sync::Arc;

pub type ManageIndexWriterMap = HashMap<i32, Arc<RwLock<IndexWriter>>>;

static mut MANAGE_TANTIVY_INDEX_WRITER_MAP: Option<Arc<RwLock<ManageIndexWriterMap>>> = None;

pub fn get_manage_index_writer_map() -> Arc<RwLock<ManageIndexWriterMap>> {
    unsafe {
        if MANAGE_TANTIVY_INDEX_WRITER_MAP.is_some() {
            return MANAGE_TANTIVY_INDEX_WRITER_MAP.clone().unwrap();
        }

        let map = Arc::new(RwLock::new(HashMap::new()));
        MANAGE_TANTIVY_INDEX_WRITER_MAP.replace(map);

        return MANAGE_TANTIVY_INDEX_WRITER_MAP.clone().unwrap();
    }
}

