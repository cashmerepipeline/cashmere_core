use bson::{self, doc, Document};
use linked_hash_map::LinkedHashMap;
use manage_define::general_field_ids::{GROUPS_FIELD_ID, OWNER_FIELD_ID};

use crate::view_rule::ViewRule;
use crate::view_rules_map::get_view_rules_map;
use crate::{FilterRule, ReadRule};

/// 集合是否可写，向集合添加或者删除实体
pub async fn add_query_filters(
    account: &String,
    groups: &Vec<String>,
    manage_id: &String,
) -> Option<Document> {
    let view_rules_arc = get_view_rules_map().await;
    let view_rules = view_rules_arc.read();

    let rules = &view_rules
        .get(manage_id)
        .and_then(|rules| Some(&rules.collection))
        .or(None);

    println!("{:}--{:?}", manage_id, view_rules.get(manage_id).unwrap());
    println!("{:}:{:?}:{:}", bson::to_document(rules.unwrap()).unwrap(), groups,manage_id);

    // 是否无限制
    if let Some(groups) = get_read_nolimit_groups(rules, groups) {
        let query_filter_doc = doc! {"$in": bson::to_bson(&groups).unwrap()};

        let mut filter_doc = doc!();
        filter_doc.insert(GROUPS_FIELD_ID.to_string(), query_filter_doc);

        return Some(filter_doc);
    }

    // 组, 任一组匹配即可, 即每个组检查是否在可读组中
    if let Some(groups) = get_readable_groups(rules, groups) {
        let query_filter_doc = doc! {"$in": bson::to_bson(&groups).unwrap()};

        let mut filter_doc = doc!();
        filter_doc.insert(GROUPS_FIELD_ID.to_string(), query_filter_doc);

        return Some(filter_doc);
    };

    // 主人
    if is_only_owner(rules, groups) {
        let mut filter_doc = doc! {};
        filter_doc.insert(OWNER_FIELD_ID.to_string(), account);

        return Some(filter_doc);
    }

    // 没有指定规则则不能访问
    None
}

/// 取得可读组
fn get_readable_groups(
    rules: &Option<&LinkedHashMap<String, ViewRule>>,
    groups: &Vec<String>,
) -> Option<Vec<String>> {
    if let Some(rules) = rules {
        let result: Vec<String> = groups
            .iter()
            .map(|id| id.to_owned())
            .filter(|group| {
                rules
                    .get(group)
                    .and_then(|rule| Some(rule.read_filters.contains(&FilterRule::OnlyGroup)))
                    .unwrap()
            })
            .collect();

        if result.len() > 0 {
            return Some(result);
        } else {
            return None;
        }
    }

    None
}

/// 取得无限制组
fn get_read_nolimit_groups(
    rules: &Option<&LinkedHashMap<String, ViewRule>>,
    groups: &Vec<String>,
) -> Option<Vec<String>> {
    if let Some(rules) = rules {
        let result: Vec<String> = groups
            .iter()
            .map(|id| id.to_owned())
            .filter(|group| {
                rules
                    .get(group)
                    .and_then(|rule| Some(rule.read_filters.contains(&FilterRule::NoLimit)))
                    .unwrap()
            })
            .collect();

        if result.len() > 0 {
            return Some(result);
        } else {
            return None;
        }
    }
    None
}

/// 判断是否只主人可读
fn is_only_owner(rules: &Option<&LinkedHashMap<String, ViewRule>>, groups: &Vec<String>) -> bool {
    if let Some(rules) = rules {
        let result: Vec<String> = groups
            .iter()
            .map(|id| id.to_owned())
            .filter(|group| {
                rules
                    .get(group)
                    .and_then(|rule| Some(rule.read_filters.contains(&FilterRule::NoLimit)))
                    .unwrap()
            })
            .collect();

        if result.len() > 0 {
            return true;
        } else {
            return false;
        }
    }
    false
}
