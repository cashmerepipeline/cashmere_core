use dependencies_sync::bson::{from_bson, to_bson};

use dependencies_sync::log::error;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::toml;

use view::ViewRules;

/// 取得映像定义
pub fn get_init_view_rules(toml_map: &toml::map::Map<String, toml::Value>) -> Option<ViewRules> {
    let result = match toml_map.get("view_rules") {
        Some(r) => r.as_table(),
        None => return None,
    };

    let manage_id = crate::get_id(toml_map).unwrap();

    match result {
        Some(rules) => {
            let manage_map = if let Some(m) = rules.get("manage") {
                let b = to_bson(m).unwrap();
                from_bson(b).unwrap()
            } else {
                error!("{}：{}", t!("取得管理权限错误"), manage_id);
                return None;
            };

            let collection_map = if let Some(c) = rules.get("collection") {
                let b = to_bson(c).unwrap();
                from_bson(b).unwrap()
            } else {
                error!("{}：{}", t!("取得集合权限错误"), manage_id);
                return None;
            };

            let schema_map = if let Some(s) = rules.get("schema") {
                let b = to_bson(s).unwrap();
                from_bson(b).unwrap()
            } else {
                error!("{}：{}", t!("取得模式描写权限错误"), manage_id);
                return None;
            };

            Some(ViewRules {
                manage: manage_map,
                collection: collection_map,
                schema: schema_map,
            })
        }
        None => None,
    }
}
