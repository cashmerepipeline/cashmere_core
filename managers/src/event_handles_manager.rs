/*
Author: 闫刚 (yes7rose@sina.com)
events_manager.rs (c) 2020
Desc: 事件处理管理器
Created:  2020-12-03T03:33:44.641Z
Modified: !date!
*/

use std::sync::Arc;

// use log::{error, info, warn};
use async_trait::async_trait;
use bson;
use parking_lot::RwLock;

use super::{Manager, ManagerInner, ManagerTrait};

use cash_core::Manage;
use cash_core::{ids::EVENT_HANDLES_MANAGE_ID, results::*};
use database;

use crate::{declare_get_manager};
use cash_core::ids::MANAGES_MANAGE_ID;
use bson::Document;

#[derive(Default)]
pub struct EventHandlesManager;

/// 事件管理
static mut EVENT_HANDLES_MANAGE: Option<Arc<RwLock<Manage>>> = None;
static mut EVENT_HANDLES_MANAGE_DOCUMENT: Option<Arc<RwLock<Document>>> = None;

/// 管理器
static mut EVENT_HANDLES_MANAGER: Option<Arc<Manager>> = None;

// 声明管理器取得函数
declare_get_manager!(EventHandlesManager, EVENT_HANDLES_MANAGER);

// 实现接口
#[async_trait]
impl ManagerTrait for EventHandlesManager {
    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        Err(operation_failed("unregister", "账户管理器不能被注销"))
    }

    fn get_manager_id(&self) -> i32 {
        return EVENT_HANDLES_MANAGE_ID;
    }

    fn get_manager_name(&self) -> String {
        "EventHandlesManager".to_string()
    }

    fn has_cache(&self) -> bool {
        false
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        unsafe {
            if EVENT_HANDLES_MANAGE.is_some() {
                EVENT_HANDLES_MANAGE.clone().unwrap()
            } else {
                let collection_name = EVENT_HANDLES_MANAGE_ID.to_string();
                let id_str = EVENT_HANDLES_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!(format!("{} {}", e.operation(), e.details())),
                };
                let manage: Manage = bson::from_document(m_doc).unwrap();
                EVENT_HANDLES_MANAGE.replace(Arc::new(RwLock::new(manage)));
                EVENT_HANDLES_MANAGE.clone().unwrap()
            }
        }
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        unsafe {
            if EVENT_HANDLES_MANAGE_DOCUMENT.is_some() {
                EVENT_HANDLES_MANAGE_DOCUMENT.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = EVENT_HANDLES_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!(format!("{} {}", e.operation(), e.details())),
                };

                EVENT_HANDLES_MANAGE_DOCUMENT.replace(Arc::new(RwLock::new(m_doc)));
                EVENT_HANDLES_MANAGE_DOCUMENT.clone().unwrap()
            }
        }
    }
}