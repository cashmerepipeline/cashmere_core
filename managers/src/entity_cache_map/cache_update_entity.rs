use std::sync::Arc;

use dependencies_sync::bson::Document;
use crate::entity_cache_map::get_manage_entity_cache;

// 更新极少发生
pub fn cache_update_entity(manage_id: i32, entity_id: &String, new_doc: Document) {
    let c_map = get_manage_entity_cache(manage_id);
    let mut c_map = c_map.write();

    c_map.insert(entity_id.to_string(), Arc::new(new_doc));
}
