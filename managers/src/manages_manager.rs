/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 管理管理器
Created:  2020-11-24T10:17:46.416Z
Modified: !date!
*/

use std::sync::Arc;

// use log::{error, info, warn};
use async_trait::async_trait;
use bson::{self, Document};
use cash_result::*;
use parking_lot::RwLock;

use crate::{declare_get_manager, Manager, ManagerInner, ManagerTrait};
use manage_define::manage_ids::MANAGES_MANAGE_ID;

use cash_core::{manage_from_document, Manage};

#[derive(Default)]
pub struct ManagesManager;

/// 管理
static mut MANAGES_MANAGE: Option<Arc<RwLock<Manage>>> = None;
static mut MANAGES_MANAGE_DOCUMENT: Option<Arc<RwLock<Document>>> = None;

/// 管理器
static mut MANAGES_MANAGER: Option<Arc<Manager>> = None;

declare_get_manager!(ManagesManager, MANAGES_MANAGER);

#[async_trait]
impl ManagerTrait for ManagesManager {
    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        Err(operation_failed("unregister", "管理的管理不能被注销"))
    }

    fn get_manager_id(&self) -> i32 {
        MANAGES_MANAGE_ID
    }

    fn get_manager_name(&self) -> String {
        "ManagesManager".to_string()
    }

    fn has_cache(&self) -> bool {
        true
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        unsafe {
            if MANAGES_MANAGE.is_some() {
                MANAGES_MANAGE.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = MANAGES_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };
                println!("{:?}", m_doc);
                let manage: Manage = manage_from_document(m_doc).unwrap();
                MANAGES_MANAGE.replace(Arc::new(RwLock::new(manage)));
                MANAGES_MANAGE.clone().unwrap()
            }
        }
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        unsafe {
            if MANAGES_MANAGE_DOCUMENT.is_some() {
                MANAGES_MANAGE_DOCUMENT.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = MANAGES_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };

                MANAGES_MANAGE_DOCUMENT.replace(Arc::new(RwLock::new(m_doc)));
                MANAGES_MANAGE_DOCUMENT.clone().unwrap()
            }
        }
    }

    async fn get_new_entity_id(&self) -> Option<i64> {
        return None;
    }

    // async fn get_entities_by_filter(filter: &Document) -> Result<Vec<Document>, OperationResult>{
    //     Err(operation_failed("get_entities_by_filter"))

    // }

    // async fn get_schema_document() -> Result<Document, OperationResult>{
    //     Err(operation_succeed("ok"))
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
