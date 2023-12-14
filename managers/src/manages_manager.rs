/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 管理管理器
Created:  2020-11-24T10:17:46.416Z
Modified: !date!
*/

use std::sync::Arc;

// use dependencies_sync::log::{error, info, warn};
use cash_result::*;
use dependencies_sync::bson::Document;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use crate::{declare_get_manager, ManagerTrait};
use manage_define::manage_ids::MANAGES_MANAGE_ID;

use cash_core::{Manage, manage_from_document};
use crate::manager::Manager;
use crate::manager_inner::ManagerInner;

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
        Err(operation_failed(
            "unregister",
            format!(
                "{}-{}-{}",
                t!("管理不能被注销"),
                self.get_id(),
                self.get_name()
            ),
        ))
    }

    fn get_id(&self) -> &'static str {
        MANAGES_MANAGE_ID.clone()
    }

    fn get_name(&self) -> String {
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
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str, &[]).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };

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
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str, &[]).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };

                MANAGES_MANAGE_DOCUMENT.replace(Arc::new(RwLock::new(m_doc)));
                MANAGES_MANAGE_DOCUMENT.clone().unwrap()
            }
        }
    }

    async fn get_new_entity_id(&self, _account_id: &str) -> Option<i64> {
        None
    }

    // async fn get_entities_by_filter(filter: &Document) -> Result<Vec<Document>, OperationResult>{
    //     Err(operation_failed("get_entities_by_filter"))

    // }

    // async fn get_schema_document() -> Result<Document, OperationResult>{
    //     Err(operation_succeed("ok"))
    // }
}
