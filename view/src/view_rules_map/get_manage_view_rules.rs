use std::sync::Arc;

use cash_result::{collection_not_exists, operation_failed, OperationResult};
use dependencies_sync::{
    bson::{doc, Document},
    parking_lot::RwLock,
};
use manage_define::manage_ids::VIEW_RULES_MANAGE_ID;

use crate::view_rules::ViewRules;

use super::get_view_rules_map;

/// 取得管理视图规则
pub async fn get_manage_view_rules(manage_id: &String) -> Option<Arc<RwLock<ViewRules>>> {
    let view_rules_map_arc = get_view_rules_map().await;
    let view_rules_map = view_rules_map_arc.read();

    view_rules_map.get(manage_id).map(|rules| rules.clone())
}
