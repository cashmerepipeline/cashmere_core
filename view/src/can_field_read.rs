use dependencies_sync::log;

use crate::view_rules_map::get_view_rules_map;
use crate::view_rules_map::query_collection_view_rules;
use crate::view_rules_map::query_field_view_rules;
use crate::FilterRule;
use crate::ReadRule;

/// 实体的可写性，可否修改实体的字段
pub async fn can_field_read(manage_id: &String, field_id: &String, role_group: &String) -> bool {
    // objectId 总可见
    if field_id == "_id"{
        return true;
    }
    
    let collection_view_rules =
        if let Ok(r) = query_collection_view_rules(manage_id, role_group).await {
            r
        } else {
            log::error!("{}:{}, {}", t!("取得集合可见性规则失败"), role_group, manage_id);
            return false;
        };
    
    log::debug!("collection_view_rules: {:?}", collection_view_rules);

    let field_view_rules =
        if let Some(r) = query_field_view_rules(manage_id, field_id, role_group).await {
            r
        } else {
            log::error!("{}:{}, {}-{}", t!("取得字段可见性规则失败"), role_group, manage_id, field_id);
            return false;
        };
    
    log::debug!("field_view_rules: {:?}", field_view_rules);

    collection_view_rules.read_rule == ReadRule::Read
        || collection_view_rules.read_rule == ReadRule::OwnerRead
        || collection_view_rules.read_rule == ReadRule::GroupRead
            && field_view_rules.read_filters.contains(&FilterRule::NoLimit)
        || field_view_rules.read_filters.contains(&FilterRule::OnlyOwner)
        || field_view_rules.read_filters.contains(&FilterRule::OnlyGroup)
}
