use std::sync::Arc;

use dependencies_sync::{parking_lot::RwLock, rust_i18n::{self, t}};
use tantivy::Index;

static mut TANTIVY_INDEX: Option<Arc<RwLock<Index>>> = None;

pub fn get_tantivy_index() -> Arc<RwLock<Index>> {
    unsafe {
        if TANTIVY_INDEX.is_none() {
            panic!("{}", t!("搜索引擎没有初始化"));
        }
        TANTIVY_INDEX.as_ref().unwrap().clone()
    }
}

pub fn set_tantivy_index(index: Arc<RwLock<Index>>) {
    unsafe {
        TANTIVY_INDEX.replace(index);
    }
}
