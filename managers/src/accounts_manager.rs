/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 账户管理
Created:  2020-11-28T02:17:47.146Z
Modified: !date!
*/

use std::sync::Arc;

// use log::{error, info, warn};
use async_trait::async_trait;
use bson;
use bson::doc;
use parking_lot::RwLock;

use super::{Manager, ManagerInner, ManagerTrait};

use cash_core::Manage;
use cash_result::*;
use manage_define::manage_ids::*;
use database;

use crate::declare_get_manager;
use bson::Document;
use  manage_define::manage_ids::MANAGES_MANAGE_ID;

#[derive(Default)]
pub struct AccountsManager;

/// 缓存
static mut ACCOUNTS_MANAGE: Option<Arc<RwLock<Manage>>> = None;
static mut ACCOUNTS_MANAGE_DOCUMENT: Option<Arc<RwLock<Document>>> = None;

/// 管理器
static mut ACCOUNTS_MANAGER: Option<Arc<Manager>> = None;

// 声明管理器取得函数
declare_get_manager!(AccountsManager, ACCOUNTS_MANAGER);

// 实现接口
#[async_trait]
impl ManagerTrait for AccountsManager {
    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        Err(operation_failed("unregister", "账户管理器不能被注销"))
    }

    fn get_manager_id(&self) -> i32 {
        return ACCOUNTS_MANAGE_ID;
    }

    fn get_manager_name(&self) -> String {
        "AccountsManager".to_string()
    }

    fn has_cache(&self) -> bool {
        false
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        unsafe {
            if ACCOUNTS_MANAGE.is_some() {
                ACCOUNTS_MANAGE.clone().unwrap()
            } else {
                let collection_name = ACCOUNTS_MANAGE_ID.to_string();
                let id_str = ACCOUNTS_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details())),
                };
                let manage: Manage = bson::from_document(m_doc).unwrap();
                ACCOUNTS_MANAGE.replace(Arc::new(RwLock::new(manage)));
                ACCOUNTS_MANAGE.clone().unwrap()
            }
        }
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        unsafe {
            if ACCOUNTS_MANAGE_DOCUMENT.is_some() {
                ACCOUNTS_MANAGE_DOCUMENT.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = ACCOUNTS_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details())),
                };

                ACCOUNTS_MANAGE_DOCUMENT.replace(Arc::new(RwLock::new(m_doc)));
                ACCOUNTS_MANAGE_DOCUMENT.clone().unwrap()
            }
        }
    }

    async fn get_new_entity_id(&self) -> Option<i64> {
        return None;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
