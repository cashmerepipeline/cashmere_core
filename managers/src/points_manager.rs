/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 工作管理
Created:  2020-11-28T02:17:47.146Z
Modified: !date!
*/

use std::sync::Arc;

// use log::{error, info, warn};
use async_trait::async_trait;
use bson;
use parking_lot::RwLock;

use super::{Manager, ManagerInner, ManagerTrait};

use cash_core::Manage;
use cash_core::{ids::POINTS_MANAGE_ID, results::*};
use database;

use crate::{declare_get_manager};
use bson::Document;
use cash_core::ids::MANAGES_MANAGE_ID;

#[derive(Default)]
pub struct WorksManager;

/// 缓存
static mut POINTS_MANAGE: Option<Arc<RwLock<Manage>>> = None;
static mut POINTS_MANAGE_DOCUMENT: Option<Arc<RwLock<Document>>> = None;

/// 管理器
static mut POINTS_MANAGER: Option<Arc<Manager>> = None;

// 声明管理器取得函数
declare_get_manager!(WorksManager, POINTS_MANAGER);

// 实现接口
#[async_trait]
impl ManagerTrait for WorksManager {
    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        Err(operation_failed("unregister", "账户管理器不能被注销"))
    }

    fn get_manager_id(&self) -> i32 {
        return POINTS_MANAGE_ID;
    }

    fn get_manager_name(&self) -> String {
        "PointsManager".to_string()
    }

    fn has_cache(&self) -> bool {
        false
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        unsafe {
            if POINTS_MANAGE.is_some() {
                POINTS_MANAGE.clone().unwrap()
            } else {
                let collection_name = POINTS_MANAGE_ID.to_string();
                let id_str = POINTS_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!(format!("{} {}", e.operation(), e.details())),
                };
                let manage: Manage = bson::from_document(m_doc).unwrap();
                POINTS_MANAGE.replace(Arc::new(RwLock::new(manage)));
                POINTS_MANAGE.clone().unwrap()
            }
        }
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        unsafe {
            if POINTS_MANAGE_DOCUMENT.is_some() {
                POINTS_MANAGE_DOCUMENT.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = POINTS_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!(format!("{} {}", e.operation(), e.details())),
                };

                POINTS_MANAGE_DOCUMENT.replace(Arc::new(RwLock::new(m_doc)));
                POINTS_MANAGE_DOCUMENT.clone().unwrap()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
