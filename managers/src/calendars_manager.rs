use std::sync::Arc;

use dependencies_sync::bson::Document;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use crate::{declare_get_manager, manager_trait::ManagerTrait};
use cash_core::{Manage, manage_from_document};
use cash_result::*;
use manage_define::manage_ids::CALENDARS_MANAGE_ID;
use manage_define::manage_ids::MANAGES_MANAGE_ID;
use crate::manager::Manager;
use crate::manager_inner::ManagerInner;

#[derive(Default)]
pub struct CalendarsManager;

/// 缓存
static mut CALENDARS_MANAGE: Option<Arc<RwLock<Manage>>> = None;
static mut CALENDARS_MANAGE_DOCUMENT: Option<Arc<RwLock<Document>>> = None;

/// 管理器
static mut CALENDARS_MANAGER: Option<Arc<Manager>> = None;

// 声明管理器取得函数
declare_get_manager!(CalendarsManager, CALENDARS_MANAGER);

// 实现接口
#[async_trait]
impl ManagerTrait for CalendarsManager {
    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        Err(operation_failed(
            "unregister",
            format!(
                "{}-{}-{}",
                t!("管理器不能被注销"),
                self.get_id(),
                self.get_name()
            ),
        ))
    }

    fn get_id(&self) -> &'static str {
        CALENDARS_MANAGE_ID
    }

    fn get_name(&self) -> String {
        "CalendarsManager".to_string()
    }

    fn has_cache(&self) -> bool {
        false
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        unsafe {
            if CALENDARS_MANAGE.is_some() {
                CALENDARS_MANAGE.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = CALENDARS_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str, &[]).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };
                let manage: Manage = manage_from_document(m_doc).unwrap();
                CALENDARS_MANAGE.replace(Arc::new(RwLock::new(manage)));
                CALENDARS_MANAGE.clone().unwrap()
            }
        }
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        unsafe {
            if CALENDARS_MANAGE_DOCUMENT.is_some() {
                CALENDARS_MANAGE_DOCUMENT.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = CALENDARS_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str, &[]).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };

                CALENDARS_MANAGE_DOCUMENT.replace(Arc::new(RwLock::new(m_doc)));
                CALENDARS_MANAGE_DOCUMENT.clone().unwrap()
            }
        }
    }
}
