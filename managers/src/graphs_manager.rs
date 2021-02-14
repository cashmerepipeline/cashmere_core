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
use cash_result::*;
use manage_define::manage_ids::*;
use database;

use crate::{declare_get_manager};
use bson::Document;
use  manage_define::manage_ids::MANAGES_MANAGE_ID;

#[derive(Default)]
pub struct WorksManager;

/// 缓存
static mut GRAPHS_MANAGE: Option<Arc<RwLock<Manage>>> = None;
static mut GRAPHS_MANAGE_DOCUMENT: Option<Arc<RwLock<Document>>> = None;

/// 管理器
static mut GRAPHS_MANAGER: Option<Arc<Manager>> = None;

// 声明管理器取得函数
declare_get_manager!(WorksManager, GRAPHS_MANAGER);

// 实现接口
#[async_trait]
impl ManagerTrait for WorksManager {
    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        Err(operation_failed("unregister", "账户管理器不能被注销"))
    }

    fn get_manager_id(&self) -> i32 {
        return GRAPHS_MANAGE_ID;
    }

    fn get_manager_name(&self) -> String {
        "GraphsManager".to_string()
    }

    fn has_cache(&self) -> bool {
        false
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        unsafe {
            if GRAPHS_MANAGE.is_some() {
                GRAPHS_MANAGE.clone().unwrap()
            } else {
                let collection_name = GRAPHS_MANAGE_ID.to_string();
                let id_str = GRAPHS_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!(format!("{} {}", e.operation(), e.details())),
                };
                let manage: Manage = bson::from_document(m_doc).unwrap();
                GRAPHS_MANAGE.replace(Arc::new(RwLock::new(manage)));
                GRAPHS_MANAGE.clone().unwrap()
            }
        }
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        unsafe {
            if GRAPHS_MANAGE_DOCUMENT.is_some() {
                GRAPHS_MANAGE_DOCUMENT.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = GRAPHS_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!(format!("{} {}", e.operation(), e.details())),
                };

                GRAPHS_MANAGE_DOCUMENT.replace(Arc::new(RwLock::new(m_doc)));
                GRAPHS_MANAGE_DOCUMENT.clone().unwrap()
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
