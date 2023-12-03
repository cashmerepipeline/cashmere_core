use dependencies_sync::log;

use cash_result::{operation_failed, OperationResult};

use super::get_manage_view_rules;
use crate::view_rule::ViewRule;

/// 取得集合视图规则
pub async fn query_collection_view_rules(
    manage_id: &String,
    group_id: &String,
) -> Result<ViewRule, OperationResult> {
    match get_manage_view_rules(manage_id).await {
        Some(manage_view_rules_map) => {
            let m = manage_view_rules_map.read();
            let result = m.collection
                .get(group_id)
                .map(|rule| rule.clone())
                .ok_or(operation_failed(
                    "query_collection_view_rules",
                    "取得集合可见性规则失败",
                ));
            log::debug!("{}: {:?}", t!("取得集合可见规则成功"), result);
            result
        }
        None => {
            log::error!(
                "{}: {}, {}",
                t!("取得集合可见规则失败"),
                manage_id,
                group_id
            );
            Err(operation_failed(
                "query_collection_view_rules",
                "取得可见规则失败",
            ))
        }
    }
}
