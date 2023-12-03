use dependencies_sync::log::error;
use dependencies_sync::linked_hash_map::LinkedHashMap;
use dependencies_sync::toml;

use view::{ViewRule, ViewRules};

/// 取得映像定义
pub fn get_init_view_rules(toml_map: &toml::map::Map<String, toml::Value>) -> Option<ViewRules> {
    let result = match toml_map.get("view_rules") {
        Some(r) => r.as_table(),
        None => return None,
    };

    let manage_id = crate::get_id(toml_map).unwrap();

    match result {
        Some(rules) => {
            let manage_str = if let Some(m_str) = rules.get("manage"){
                m_str.to_string()
            }else {
                error!("取得管理权限错误：{}", manage_id);
                return None;
            };

            let collection_str = if let Some(c_str) = rules.get("collection"){
                c_str.to_string()
            }else {
                error!("取得集合权限错误：{}", manage_id);
                return None;
            };
            let schema_str = if let Some(s_str) = rules.get("schema"){
                s_str.to_string()
            } else {
                error!("取得模式描写权限错误：{}", manage_id);
                return None;
            };

            let manage_map: LinkedHashMap<String, ViewRule> =
                toml::from_str(manage_str.as_str()).unwrap();
            let entity_map: LinkedHashMap<String, ViewRule> =
                toml::from_str(collection_str.as_str()).unwrap();
            let schema_map: LinkedHashMap<String, LinkedHashMap<String, ViewRule>> =
                toml::from_str(schema_str.as_str()).unwrap();

            Some(ViewRules {
                manage: manage_map,
                collection: entity_map,
                schema: schema_map,
            })
        }
        None => None,
    }
}
