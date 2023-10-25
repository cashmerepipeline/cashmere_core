use std::sync::Arc;

use crate::get_manage_tantivy_index_map;
use dependencies_sync::tantivy::Index;

use super::init_tantivy_index::init_tantivy_index;

/// 取得管理索引
pub fn get_manage_tantivy_index(manage_id: i32) -> Arc<Index> {
    let map_arc = get_manage_tantivy_index_map();

    {
        let mut map = map_arc.read();
        if let Some(r) = map.get(&manage_id) {
            return r.clone();
        }
    }
    {
        init_tantivy_index(manage_id);
        let mut map = map_arc.read();
        map.get(&manage_id).unwrap().clone()
    }
}
