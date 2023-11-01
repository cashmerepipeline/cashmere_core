use dependencies_sync::bson::{doc, Document};
use dependencies_sync::log::error;
use dependencies_sync::rust_i18n::{self, t};

use manage_define::general_field_ids::{GROUPS_FIELD_ID, OWNER_FIELD_ID};

use crate::add_query_filters::check_group_read_group::check_group_read_group;
use crate::add_query_filters::check_group_read_nolimit::check_group_read_nolimit;
use crate::add_query_filters::check_group_read_only_owner::check_group_read_only_owner;
use crate::view_rules_map::{get_view_rules_map, query_collection_view_rules};

/// 加入查询过滤
pub async fn add_query_filters(
    account: &String,
    group: &String,
    manage_id: &String,
) -> Option<Document> {
    let rule = if let Some(r) = query_collection_view_rules(manage_id, group).await {
        r
    } else {
        error!("{}: {}", t!("读取可见性规则失败"), manage_id);
        return None;
    };

    // 是否无限制
    if check_group_read_nolimit(&rule) {
        let filter_doc = doc! {};
        return Some(filter_doc);
    }

    // 组, 任一组匹配即可, 即每个组检查是否在可读组中
    if check_group_read_group(&rule) {
        let query_filter_doc = doc! {"$elemMatch":{"$eq":group.clone()}};
        let mut filter_doc = doc! {};
        filter_doc.insert(GROUPS_FIELD_ID.to_string(), query_filter_doc);
        return Some(filter_doc);
    }

    // 只主人可读
    if check_group_read_only_owner(&rule) {
        let mut filter_doc = doc! {};
        filter_doc.insert(OWNER_FIELD_ID.to_string(), account);

        return Some(filter_doc);
    }

    // 没有指定规则则不能访问
    None
}
