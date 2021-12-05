/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 学科管理
Created:  2020-11-28T02:17:47.146Z
Modified: !date!
*/

use std::sync::Arc;

// use log::{error, info, warn};
use async_trait::async_trait;
use bson;
use parking_lot::RwLock;
use bson::Document;

use cash_core::Manage;
use cash_result::*;
use manage_define::manage_ids::*;
use crate::{Manager, ManagerInner, traits::ManagerTrait, declare_get_manager};

use  manage_define::manage_ids::MANAGES_MANAGE_ID;

#[derive(Default)]
pub struct DisciplinesManager;

/// 缓存
static mut DISCIPLINE_MANAGE: Option<Arc<RwLock<Manage>>> = None;
static mut DISCIPLINE_MANAGE_DOCUMENT: Option<Arc<RwLock<Document>>> = None;

/// 管理器
static mut DISCIPLINE_MANAGER: Option<Arc<Manager>> = None;

// 声明管理器取得函数
declare_get_manager!(DisciplinesManager, DISCIPLINE_MANAGER);

// 实现接口
#[async_trait]
impl ManagerTrait for DisciplinesManager {
    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        Err(operation_failed("unregister", "账户管理器不能被注销"))
    }

    fn get_manager_id(&self) -> i32 {
        return DISCIPLINES_MANAGE_ID;
    }

    fn get_manager_name(&self) -> String {
        "DisciplinesManager".to_string()
    }

    fn has_cache(&self) -> bool {
        false
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        unsafe {
            if DISCIPLINE_MANAGE.is_some() {
                DISCIPLINE_MANAGE.clone().unwrap()
            } else {
                let collection_name = DISCIPLINES_MANAGE_ID.to_string();
                let id_str = DISCIPLINES_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };
                let manage: Manage = bson::from_document(m_doc).unwrap();
                DISCIPLINE_MANAGE.replace(Arc::new(RwLock::new(manage)));
                DISCIPLINE_MANAGE.clone().unwrap()
            }
        }
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        unsafe {
            if DISCIPLINE_MANAGE_DOCUMENT.is_some() {
                DISCIPLINE_MANAGE_DOCUMENT.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = DISCIPLINES_MANAGE_ID.to_string();
                let m_doc = match entity::get_entity_by_id(&collection_name, &id_str).await {
                    Ok(r) => r,
                    Err(e) => panic!("{} {}", e.operation(), e.details()),
                };

                DISCIPLINE_MANAGE_DOCUMENT.replace(Arc::new(RwLock::new(m_doc)));
                DISCIPLINE_MANAGE_DOCUMENT.clone().unwrap()
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