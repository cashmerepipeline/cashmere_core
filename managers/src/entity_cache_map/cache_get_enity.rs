use std::ops::{Deref, Not};

use dependencies_sync::bson::Document;

use crate::entity_cache_map::get_manage_entity_cache;

pub fn cache_get_entity(manage_id: &'static str, entity_id: &str, present_fields: &[String], no_present_fields: &[String]) -> Option<Document> {
    let c_map = get_manage_entity_cache(manage_id);
    let e_map = c_map.read();
    let e_map = e_map.get(entity_id);

    if let Some(e) = e_map {
        let mut result = e.deref().clone();
        
        // zh: 要排除的字段表
        // en: The table of fields to be excluded
        let mut excluded_fields = no_present_fields.clone();
        
        // zh: 将需要排除的字段加入
        // en: Add the fields that need to be excluded
        for key in result.keys() {
            if !present_fields.contains(&key.to_string()) {
                excluded_fields.push(key.to_string());
            }
        }

        // zh: 排除字段
        // en: Exclude fields
        for key in excluded_fields {
            result.remove(key);
        }

        return Some(result);
    }

    None
}
