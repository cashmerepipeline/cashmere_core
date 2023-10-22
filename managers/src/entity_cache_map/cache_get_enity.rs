use std::ops::Deref;

use dependencies_sync::bson::Document;

use crate::entity_cache_map::get_manage_entity_cache;

pub fn cache_get_entity(manage_id: i32, entity_id: &String) -> Option<Document> {
    let c_map = get_manage_entity_cache(manage_id);
    let e_map = c_map.read();
    let e_map = e_map.get(entity_id);

    if let Some(e) = e_map {
        return Some(e.deref().clone());
    }

    None
}
