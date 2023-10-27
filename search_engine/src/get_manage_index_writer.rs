use std::sync::Arc;
use dependencies_sync::{parking_lot::RwLock, tantivy::{IndexWriter, TantivyDocument}, log, rust_i18n::{self, t}};

use crate::{get_manage_tantivy_index, manage_index_writer_map::get_manage_index_writer_map};

pub fn get_manage_index_writer(manage_id: i32) -> Arc<RwLock<IndexWriter>> {
    let map_arc = get_manage_index_writer_map();
    {
        let map = map_arc.read();
        if let Some(r) = map.get(&manage_id) {
            return r.clone();
        }
    }

    {
        let mut map = map_arc.write();
        let index = get_manage_tantivy_index(manage_id);
        let writer = if let Ok(r) = index.writer::<TantivyDocument>(15_000_000){
            r
        }else{
            log::error!("{}: {}", t!("获取writer失败"), manage_id);
            panic!("{}", t!("获取writer失败"));
        };

        let writer = Arc::new(RwLock::new(writer));
        map.insert(manage_id, writer.clone());

        writer
    }
}
