/*
Author: 闫刚 (yes7rose@sina.com)
view_rule.rs (c) 2020
Desc: 映像
Created:  2020-10-30T11:13:27.146Z
Modified: !date!
*/

use std::sync::Arc;

use bson::{self, de::from_document, Document};

use futures::stream::StreamExt;
use linked_hash_map::LinkedHashMap;
use manage_define::general_field_ids::ID_FIELD_ID;
use mongodb;
use parking_lot::RwLock;

use dependencies_sync::log::{info};

use manage_define::field_ids::VIEW_RULES_COLLECTION_FIELD_ID;
use manage_define::field_ids::VIEW_RULES_ENTITY_FIELD_ID;
use manage_define::field_ids::VIEW_RULES_MANAGE_FIELD_ID;
use manage_define::manage_ids::VIEW_RULES_MANAGE_ID;
use crate::view_rules::ViewRules;

type MongodbResult<T> = mongodb::error::Result<T>;

pub type ViewRulesMap = LinkedHashMap<String, ViewRules>;

/// 全局映像规则缓存, {id:rules}
static mut VIEW_RULES_MAP: Option<Arc<RwLock<ViewRulesMap>>> = None;

pub async fn get_view_rules_map() -> Arc<RwLock<ViewRulesMap>> {
    unsafe {
        if VIEW_RULES_MAP.is_some() {
            VIEW_RULES_MAP.clone().unwrap()
        } else {
            VIEW_RULES_MAP.get_or_insert(init_view_rules().await.unwrap());
            VIEW_RULES_MAP.clone().unwrap()
        }
    }
}

/// 从数据库读取映射规则
async fn init_view_rules() -> Option<Arc<RwLock<ViewRulesMap>>> {
    let view_rules_collection =
        match database::get_collection_by_id(&VIEW_RULES_MANAGE_ID.to_string()).await {
            Some(r) => r,
            None => return None,
        };
    let cusor = match view_rules_collection.find(None, None).await {
        Ok(c) => c,
        Err(_e) => return None,
    };

    let view_rules: Vec<MongodbResult<Document>> = cusor.collect().await;
    let mut result = ViewRulesMap::new();
    for view_rule in view_rules {
        match view_rule {
            Ok(ref d) => {
                let id = d.get_str(ID_FIELD_ID.to_string()).unwrap().to_string();

                info!("初始化管理映像规则表: {}", id);

                let manage_rule = from_document(
                    d.get_document(&VIEW_RULES_MANAGE_FIELD_ID.to_string())
                        .unwrap()
                        .clone(),
                )
                .unwrap();
                let collection_rule = from_document(
                    d.get_document(&VIEW_RULES_COLLECTION_FIELD_ID.to_string())
                        .unwrap()
                        .clone(),
                )
                .unwrap();
                let schema_rule = from_document(
                    d.get_document(&VIEW_RULES_ENTITY_FIELD_ID.to_string())
                        .unwrap()
                        .clone(),
                )
                .unwrap();

                let view_rules = ViewRules {
                    manage: manage_rule,
                    collection: collection_rule,
                    schema: schema_rule,
                };
                result.insert(id, view_rules);
            }
            Err(_e) => continue,
        }
    }

    Some(Arc::new(RwLock::new(result)))
}

// /// 取得 根级 权限，系统入口控制
// /// 只有root组可写
// pub fn get_root_level_view_rule(
//     root_view_rules_doc: &Document,
//     account_doc: &Document,
// ) -> Option<ViewRule> {
//     let root_view_doc = match root_view_rules_doc.get_document("manage") {
//         Ok(r) => r,
//         Err(_e) => return None
//     };
//     let view_rules: ViewRules = match from_document(root_view_doc.clone()) {
//         Some(r) => r,
//         None => return None
//     };

//     let groups = account::get_account_groups(account_doc).unwrap();

//     // 不可读返回不可见
//     let mut result = ViewRule { read_rule: ReadRule::InVisible, write_rule: WriteRule::ReadOnly, read_filters: [], write_filters: []};
//     // 是否可读，访问入口
//     if can_groups_read(&groups, &view_rules.read) {
//         // 读
//         result.read_rule = ReadRule::GroupRead;
//         if is_read_owner_only(&view_rules.and) {
//             result.read_rule = ReadRule::OwnerRead;
//         }

//         // 是否可写，管理的管理
//         if can_groups_write(&groups, &view_rules.write) {
//             result.write_rule = WriteRule::GroupWrite;
//         } else if view_rules.and.contains(&"WriteOwnerOlny".to_string()) {
//             result.write_rule = WriteRule::OwnerWrite;
//         }
//     }

//     Some(result)
// }

//// 取得 管理级 权限
// pub fn get_manage_level_view_rule(
//     view_rules_doc: &Document,
//     account_doc: &Document,
// ) -> Option<ViewRule> {
//     let manage_doc = match view_rules_doc.get_document("manage") {
//         Ok(r) => r,
//         Err(_e) => return None
//     };
//     let view_rules: ViewRules = match from_document(manage_doc.clone()) {
//         Some(r) => r,
//         None => return None
//     };

//     let groups = account::get_account_groups(account_doc).unwrap();

//     // 不可读返回不可见
//     let mut result = ViewRule { read_rule: ReadRule::InVisible, write_rule: WriteRule::ReadOnly };
//     // 是否可读，访问入口
//     if can_groups_read(&groups, &view_rules.read) {
//         // 读
//         result.read_rule = ReadRule::GroupRead;
//         if is_read_owner_only(&view_rules.and) {
//             result.read_rule = ReadRule::OwnerRead;
//         }

//         // 是否可写，管理的管理
//         if can_groups_write(&groups, &view_rule.) {
//             result.write_rule = WriteRule::GroupWrite;
//         } else if view_rules.and.contains(&"WriteOwnerOlny".to_string()) {
//             result.write_rule = WriteRule::OwnerWrite;
//         }
//     }

//     Some(result)
// }

//// 组可读
// fn can_groups_read(
//     groups: &Vec<String>,
//     reads: &Vec<String>,
// ) -> bool {
//     let mut result = false;

//     for g in groups {
//         if reads.contains(g) {
//             result = true;
//             break;
//         }
//     }

//     result
// }

// 组可写
// fn can_groups_write(
//     groups: &Vec<String>,
//     writes: &Vec<String>,
// ) -> bool {
//     let mut result = false;

//     for g in groups {
//         if writes.contains(g) {
//             result = true;
//             break;
//         }
//     }

//     result
// }

// 只主可读
// fn is_read_owner_only(
//     ands: &Vec<String>,
// ) -> bool {
//     if ands.contains(&"ReadOwnerOnly".to_string()) {
//         true
//     } else {
//         false
//     }
// }

// 只主可写
// fn is_write_owner_only(
//     ands: &Vec<String>
// ) -> bool {
//     if ands.contains(&"WriteOwnerOnly".to_string()) {
//         true
//     } else {
//         false
//     }
// }
