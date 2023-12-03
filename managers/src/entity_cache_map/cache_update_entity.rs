use std::{ops::Deref, sync::Arc};

use crate::entity_cache_map::get_manage_entity_cache;
use dependencies_sync::bson::Document;

// 更新极少发生
// 返回旧数据，后继操作失败以恢复缓存
pub fn cache_update_entity(
    manage_id: i32,
    entity_id: &str,
    new_doc: Document,
) -> Option<Document> {
    let c_map = get_manage_entity_cache(manage_id);
    let mut c_map = c_map.write();

    let old_doc = c_map
        .remove(&entity_id.to_string()).map(|d| d.deref().clone());

    c_map.insert(entity_id.to_string(), Arc::new(new_doc));

    old_doc
}
