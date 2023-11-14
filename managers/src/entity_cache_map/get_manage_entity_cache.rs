use std::collections::BTreeMap;
use std::sync::Arc;

use dependencies_sync::bson::Document;
use dependencies_sync::parking_lot::RwLock;

use crate::entity_cache_map::get_entity_cache_map;

pub fn get_manage_entity_cache(manage_id: i32) -> Arc<RwLock<BTreeMap<String, Arc<Document>>>> {
    let c_map_arc = get_entity_cache_map();
    let mut c_map = c_map_arc.write();
    let e_map = c_map.get(&manage_id);

    if let Some(r) = e_map {
        return r.clone();
    }

    let new_map = Arc::new(RwLock::new(BTreeMap::new()));
    c_map.insert(manage_id, new_map.clone());

    new_map.clone()
}
