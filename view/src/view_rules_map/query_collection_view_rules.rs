use dependencies_sync::log;
use dependencies_sync::rust_i18n::{self, t};

use cash_result::{operation_failed, OperationResult};

use super::get_manage_view_rules;
use crate::view_rule::ViewRule;

/// 取得集合视图规则
pub async fn query_collection_view_rules(manage_id: &str, group_id: &str) -> Option<ViewRule> {
    if let Some(manage_view_rules_map) = get_manage_view_rules(manage_id).await {
        let m = manage_view_rules_map.read();
        m.collection.get(group_id).cloned()
    } else {
        log::error!(
            "{}: {}, {}",
            t!("取得集合可见规则失败"),
            manage_id,
            group_id
        );
        None
    }
}
