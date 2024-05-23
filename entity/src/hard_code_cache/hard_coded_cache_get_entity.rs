use std::ops::Deref;

use dependencies_sync::{
    bson::Document,
    log::debug,
    rust_i18n::{self, t},
};

use super::get_hard_coded_cache_map;

pub async fn hard_coded_cache_get_entity(
    manage_id: &'static str,
    entity_id: &str,
    present_fields: &[String],
    no_present_fields: &[String],
) -> Option<Document> {
    let entity = {
        let c_map = get_hard_coded_cache_map(manage_id).await.unwrap();
        let e_map = c_map.read();
        e_map.get(entity_id).cloned()
    };

    if let Some(e) = entity {
        let mut result = e.clone();

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
