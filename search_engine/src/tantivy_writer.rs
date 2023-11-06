use std::sync::Arc;

use dependencies_sync::{
    parking_lot::RwLock,
    tantivy::{IndexWriter},
};

use crate::tantivy_index::get_tantivy_index;

static mut TANTIVY_WRITER: Option<Arc<RwLock<IndexWriter>>> = None;

pub fn get_tantivy_writer() -> Arc<RwLock<IndexWriter>> {
    unsafe {
        if TANTIVY_WRITER.is_none() {
            let index_arc = get_tantivy_index();
            let index = index_arc.read();
            let writer = if let Ok(r) = index.writer(15_000_000) {
                r
            } else {
                panic!("Failed to create writer");
            };
            TANTIVY_WRITER.replace(Arc::new(RwLock::new(writer)));
        }

        TANTIVY_WRITER.as_ref().unwrap().clone()
    }
}
