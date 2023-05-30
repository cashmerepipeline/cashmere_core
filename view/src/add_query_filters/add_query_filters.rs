use dependencies_sync::bson::{doc, Document};
use manage_define::general_field_ids::{GROUPS_FIELD_ID, OWNER_FIELD_ID};

use crate::add_query_filters::check_group_read_group::check_group_read_group;
use crate::add_query_filters::check_group_read_nolimit::check_group_read_nolimit;
use crate::add_query_filters::check_group_read_only_owner::check_group_read_only_owner;
use crate::view_rules_map::get_view_rules_map;

/// 集合是否可写，向集合添加或者删除实体
pub async fn add_query_filters(
    account: &String,
    group: &String,
    manage_id: &String,
) -> Option<Document> {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let rules = &view_rules
        .get(manage_id).map(|rules| &rules.collection)
        .or(None);

    // 是否无限制
    if check_group_read_nolimit(rules, group){
        let filter_doc = doc!{};
        return Some(filter_doc);
    }

    // 组, 任一组匹配即可, 即每个组检查是否在可读组中
    if check_group_read_group(rules, group){
        let query_filter_doc = doc! {"$elemMatch":{"$eq":group.clone()}};
        let mut filter_doc = doc!{};
        filter_doc.insert(GROUPS_FIELD_ID.to_string(), query_filter_doc);
        return Some(filter_doc);
    }

    // 只主人可读
    if check_group_read_only_owner(rules, group){
        let mut filter_doc = doc! {};
        filter_doc.insert(OWNER_FIELD_ID.to_string(), account);

        return Some(filter_doc);
    }
    // 没有指定规则则不能访问
    None
}
