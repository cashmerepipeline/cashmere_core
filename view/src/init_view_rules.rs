use crate::view_rules_map::get_view_rules_map;

/// 初始化可见规则
pub async fn init_view_rules() {
    let _ = get_view_rules_map().await;
}