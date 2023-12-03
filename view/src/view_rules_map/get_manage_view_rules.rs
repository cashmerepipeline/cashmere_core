use std::sync::Arc;

use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::log;
use dependencies_sync::rust_i18n::{self, t};

use crate::view_rules::ViewRules;

use super::get_view_rules_map;

/// 取得管理视图规则
pub async fn get_manage_view_rules(manage_id: &String) -> Option<Arc<RwLock<ViewRules>>> {
    let view_rules_map_arc = get_view_rules_map().await;
    let view_rules_map = view_rules_map_arc.read();

    match view_rules_map.get(manage_id) {
        Some(r) => Some(r.clone()),
        None => {
            log::error!("{}: {}", t!("获取管理可见规则表失败"), manage_id);
            None
        }
    }
}
