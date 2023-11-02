use std::sync::Arc;

use dependencies_sync::{tantivy::Index, parking_lot::RwLock};

static mut TANTIVY_INDEX: Option<Arc<RwLock<Index>>> = None;

pub fn get_tantivy_index() -> Arc<RwLock<Index>> {
    unsafe {
        if TANTIVY_INDEX.is_none() {
          panic!("Tantivy index not initialized");
        }
        TANTIVY_INDEX.as_ref().unwrap().clone()
    }
}

pub fn set_tantivy_index(index: Arc<RwLock<Index>>) {
    unsafe {
        TANTIVY_INDEX.replace(index);
    }
}