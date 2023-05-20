/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 工作管理
Created:  2020-11-28T02:17:47.146Z
Modified: !date!
*/

use std::sync::Arc;

// use log::{error, info, warn};
use dependencies_sync::tonic::async_trait;
use bson;
use bson::Document;
use parking_lot::RwLock;

use cash_core::{manage_from_document, Manage};
use cash_result::*;
use crate::{declare_get_manager, traits::ManagerTrait, Manager, ManagerInner};

use manage_define::manage_ids::MANAGES_MANAGE_ID;
use manage_define::manage_ids::STAGES_MANAGE_ID;

#[derive(Default)]
pub struct StagesManager;

/// 缓存
static mut STAGES_MANAGE: Option<Arc<RwLock<Manage>>> = None;
static mut STAGES_MANAGE_DOCUMENT: Option<Arc<RwLock<Document>>> = None;

/// 管理器
static mut STAGES_MANAGER: Option<Arc<Manager>> = None;

// 声明管理器取得函数
declare_get_manager!(StagesManager, STAGES_MANAGER);

// 实现接口
#[async_trait]
impl ManagerTrait for StagesManager {
    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        Err(operation_failed("unregister", "账户管理器不能被注销"))
    }

    fn get_manager_id(&self) -> i32 {
        return STAGES_MANAGE_ID;
    }

    fn get_manager_name(&self) -> String {
        "StagesManager".to_string()
    }

    fn has_cache(&self) -> bool {
        false
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        unsafe {
            if STAGES_MANAGE.is_some() {
                STAGES_MANAGE.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = STAGES_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };
                let manage: Manage = manage_from_document(m_doc).unwrap();
                STAGES_MANAGE.replace(Arc::new(RwLock::new(manage)));
                STAGES_MANAGE.clone().unwrap()
            }
        }
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        unsafe {
            if STAGES_MANAGE_DOCUMENT.is_some() {
                STAGES_MANAGE_DOCUMENT.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = STAGES_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };

                STAGES_MANAGE_DOCUMENT.replace(Arc::new(RwLock::new(m_doc)));
                STAGES_MANAGE_DOCUMENT.clone().unwrap()
            }
        }
    }
}

