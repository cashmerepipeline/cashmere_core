/*
Author: 闫刚 (yes7rose@sina.com)
view_rule.rs (c) 2020
Desc: 映像
Created:  2020-10-30T11:13:27.146Z
Modified: !date!
*/


use linked_hash_map::LinkedHashMap;
use bson::{self, de::from_document, doc, Document};
use cash_result::*;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use futures::stream::StreamExt;
use mongodb;

use  manage_define::manage_ids::VIEW_RULES_MANAGE_ID;
use  manage_define::field_ids::VIEW_RULES_MANAGE_FIELD_ID;
use  manage_define::field_ids::VIEW_RULES_COLLECTION_FIELD_ID;
use  manage_define::field_ids::VIEW_RULES_ENTITY_FIELD_ID;

/// 映像结果
enum ViewRuleResult {
    // 不可读
    InVisible,
    // 全部可读
    Read,
    // 只组的 可读
    GroupRead,
    // 只主的 可读
    OwnerRead,
    // 全部可写
    Write,
    // 只组的可写
    GroupWrite,
    // 只主的 可写
    OwnerWrite,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ReadRule {
    // 不可读
    InVisible,
    // 全部可读
    Read,
    // 只组的 可读
    GroupRead,
    // 只主的 可读
    OwnerRead,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum WriteRule {
    // 只读
    InVisible,
    // 全部可写
    Write,
    // 只组的可写
    GroupWrite,
    // 只主的 可写
    OwnerWrite,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Filter {
    NoLimit,
    // 主 所有
    OwnerALl,
    // 组 所有
    GroupALl,
    //只有组的
    OnlyGroup,
    // 只有主的
    OnlyOwner,
}

/// 映像
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViewRule {
    pub read_rule: ReadRule,
    pub write_rule: WriteRule,
    pub read_filters: Vec<Filter>,
    pub write_filters: Vec<Filter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViewRules {
    // {组：规则}
    pub manage: LinkedHashMap<String, ViewRule>,
    pub collection: LinkedHashMap<String, ViewRule>,
    // {属性：组：规则}
    pub schema: LinkedHashMap<String, LinkedHashMap<String, ViewRule>>,
}

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

                let id = d.get_str("_id").unwrap().to_string();
                info!("初始化管理映像规则表: {}", id);
                let manage_rule = from_document(d.get_document(&VIEW_RULES_MANAGE_FIELD_ID.to_string()).unwrap().clone()).unwrap();
                let collection_rule = from_document(d.get_document(&VIEW_RULES_COLLECTION_FIELD_ID.to_string()).unwrap().clone()).unwrap();
                let schema_rule = from_document(d.get_document(&VIEW_RULES_ENTITY_FIELD_ID.to_string()).unwrap().clone()).unwrap();

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

/// 新建映射
pub async fn new_view_rules(
    id: i32,
    name: &String,
    rules: &ViewRules,
    account_id: &String,
    group_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 创建doc
    let result = new_view_rules_entity_to_database(id, name, rules, account_id, group_id).await;
    // 加入全局列表
    match result {
        Ok(r) => {
            let view_rules_arc = get_view_rules_map().await;
            let mut view_rules_map = view_rules_arc.write();
            view_rules_map.insert(name.clone(), rules.clone());
            Ok(r)
        }
        Err(e) => Err(e),
    }
}

/// 新建映像规则实体
pub async fn new_view_rules_entity_to_database(
    id: i32,
    name: &String,
    rules: &ViewRules,
    account_id: &String,
    group_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 创建doc
    let mut view_rules_doc = bson::to_document(rules).unwrap();
    // 指定名
    view_rules_doc.insert("_id", id);
    view_rules_doc.insert("name", name);

    // 入库
    let result = entity::insert_entity(
        &VIEW_RULES_MANAGE_ID.to_string(),
        &mut view_rules_doc,
        account_id,
        group_id,
    )
    .await;

    result
}

/// 取得管理映射规则
pub async fn get_manage_view_rules_doc(manage_name: &String) -> Result<Document, OperationResult> {
    //1. 取得映像记录
    let view_collection = match database::get_collection_by_id(&VIEW_RULES_MANAGE_ID.to_string()).await {
        Some(r) => r,
        None => return Err(collection_not_exists("get_manage_view_rules_doc")),
    };
    let view_rules_doc = if let Ok(result) = view_collection
        .find_one(
            doc! {
                "name": manage_name.clone()
            },
            None,
        )
        .await
    {
        if let Some(r) = result {
            r
        } else {
            return Err(operation_failed(
                "get_manage_view_rules_doc",
                "映射规则不存在",
            ));
        }
    } else {
        return Err(operation_failed(
            "get_manage_view_rules_doc",
            "取得映射规则失败",
        ));
    };

    Ok(view_rules_doc)
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
