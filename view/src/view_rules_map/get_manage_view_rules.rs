use std::sync::Arc;
use dependencies_sync::parking_lot::RwLock;
use crate::view_rules::ViewRules;

use super::get_view_rules_map;

/// 取得管理视图规则
pub async fn get_manage_view_rules(manage_id: &String) -> Option<Arc<RwLock<ViewRules>>> {
    let view_rules_map_arc = get_view_rules_map().await;
    let view_rules_map = view_rules_map_arc.read();

    view_rules_map.get(manage_id).map(|rules| rules.clone())
}
