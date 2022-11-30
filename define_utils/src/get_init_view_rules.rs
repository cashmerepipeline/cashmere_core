use linked_hash_map::LinkedHashMap;

use cash_core::view_rules::{ViewRule, ViewRules};

/// 取得映像定义
pub fn get_init_view_rules(toml_map: &toml::map::Map<String, toml::Value>) -> Option<ViewRules> {
    let result = match toml_map.get("view_rules") {
        Some(r) => r.as_table(),
        None => return None,
    };

    match result {
        Some(r) => {
            let manage_str = r.get("manage").unwrap().to_string();
            let collection_str = r.get("collection").unwrap().to_string();
            let schema_str = r.get("schema").unwrap().to_string();
            println!("{} {} {}", manage_str, collection_str, schema_str);

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
