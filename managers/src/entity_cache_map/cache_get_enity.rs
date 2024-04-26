use std::ops::{Deref, Not};

use dependencies_sync::{
    bson::Document,
    log::debug,
    rust_i18n::{self, t},
};

use crate::entity_cache_map::get_manage_entity_cache;

pub async fn cache_get_entity(
    manage_id: &'static str,
    entity_id: &str,
    present_fields: &[String],
    no_present_fields: &[String],
) -> Option<Document> {
    let e_map = {
        let c_map = get_manage_entity_cache(manage_id).await;
        let e_map = c_map.read();
        e_map.get(entity_id).cloned()
    };

    if let Some(e) = e_map {
        let mut result = e.deref().clone();

        // zh: 要排除的字段表
        // en: The table of fields to be excluded
        let mut excluded_fields: Vec<String> = no_present_fields.to_owned();

        // zh: 将需要排除的字段加入
        // en: Add the fields that need to be excluded
        if !present_fields.is_empty() {
            for key in result.keys() {
                if !present_fields.contains(&key.to_string()) {
                    excluded_fields.push(key.to_string());
                }
            }
        }

        // zh: 排除字段
        // en: Exclude fields
        for key in excluded_fields {
            result.remove(key);
        }

        debug!("{}: {}-{:?}", t!("取得实体缓存"), manage_id, result);

        return Some(result);
    }

    None
}
