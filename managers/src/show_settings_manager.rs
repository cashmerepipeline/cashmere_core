/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 人管理器
Created:  2020-11-28T02:17:47.146Z
Modified: !date!
*/

use std::sync::Arc;

// use dependencies_sync::log::{error, info, warn};
use dependencies_sync::rust_i18n::{self, t}
use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::bson::Document;

use cash_core::{manage_from_document, Manage};
use cash_result::*;
use manage_define::manage_ids::*;
use  manage_define::manage_ids::MANAGES_MANAGE_ID;
use crate::{Manager, ManagerInner, traits::ManagerTrait,declare_get_manager};

#[derive(Default)]
pub struct ShowSettingsManager;

/// 缓存
static mut SHOW_SETTINGS_MANAGE: Option<Arc<RwLock<Manage>>> = None;
static mut SHOW_SETTINGS_MANAGE_DOCUMENT: Option<Arc<RwLock<Document>>> = None;

/// 管理器
static mut SHOW_SETTINGS_MANAGER: Option<Arc<Manager>> = None;

// 声明管理器取得函数
declare_get_manager!(ShowSettingsManager, SHOW_SETTINGS_MANAGER);

// 实现接口
#[async_trait]
impl ManagerTrait for ShowSettingsManager {
    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        Err(operation_failed("unregister", format!("{}-{}", t!(format!("{}-{}-{}", t!("管理器不能被注销"), self.get_id() , self.get_name())), self.get_id() )))
    }

    fn get_id(&self) -> i32 {
        SHOW_SETTINGS_MANAGE_ID
    }

    fn get_name(&self) -> String {
        "ShowSettingsManager".to_string()
    }

    fn has_cache(&self) -> bool {
        false
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        unsafe {
            if SHOW_SETTINGS_MANAGE.is_some() {
                SHOW_SETTINGS_MANAGE.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = SHOW_SETTINGS_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };
                let manage: Manage = manage_from_document(m_doc).unwrap();
                SHOW_SETTINGS_MANAGE.replace(Arc::new(RwLock::new(manage)));
                SHOW_SETTINGS_MANAGE.clone().unwrap()
            }
        }
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        unsafe {
            if SHOW_SETTINGS_MANAGE_DOCUMENT.is_some() {
                SHOW_SETTINGS_MANAGE_DOCUMENT.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = SHOW_SETTINGS_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };

                SHOW_SETTINGS_MANAGE_DOCUMENT.replace(Arc::new(RwLock::new(m_doc)));
                SHOW_SETTINGS_MANAGE_DOCUMENT.clone().unwrap()
            }
        }
    }
}



