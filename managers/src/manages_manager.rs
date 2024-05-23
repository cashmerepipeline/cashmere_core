/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 管理管理器
Created:  2020-11-24T10:17:46.416Z
Modified: !date!
*/

use std::sync::{Arc, OnceLock};

use cash_result::*;
use dependencies_sync::bson::Document;

use dependencies_sync::once_cell::sync::Lazy;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::log::{error, info, warn};
use dependencies_sync::tonic::async_trait;

use crate::entity_interface::EntityInterface;
use crate::hard_coded_cache_interface::HardCodedInterface;
use crate::{declare_common_manager_interface, declare_get_manager, AllManagerInterface, ManagerInterface};
use manage_define::manage_ids::MANAGES_MANAGE_ID;

use crate::manager::Manager;
use cash_core::{manage_from_document, Manage};

#[derive(Default)]
pub struct ManagesManager {}

/// 管理
static MANAGES_MANAGE: OnceLock<Arc<RwLock<Manage>>> = OnceLock::new();
static MANAGES_MANAGE_DOCUMENT: OnceLock<Arc<RwLock<Document>>> = OnceLock::new();

/// 管理器
static INNER: Lazy<Arc<Box<dyn AllManagerInterface>>> =
    Lazy::new(|| Arc::new(Box::new(ManagesManager {})));

static MANAGES_MANAGER: OnceLock<Manager> = OnceLock::new();

declare_get_manager!(ManagesManager, MANAGES_MANAGER, INNER.clone());

#[async_trait]
impl AllManagerInterface for ManagesManager {}

declare_common_manager_interface!(
    ManagesManager,
    MANAGES_MANAGE,
    MANAGES_MANAGE_DOCUMENT,
    MANAGES_MANAGE_ID
);

/// zh: 这里不使用宏生成，留作订制模板
// #[async_trait]
// impl ManagerInterface for ManagesManager {
//     fn unregister(&self) -> Result<OperationResult, OperationResult> {
//         Err(operation_failed(
//             "unregister",
//             format!(
//                 "{}-{}-{}",
//                 t!("管理不能被注销"),
//                 self.get_id(),
//                 self.get_name()
//             ),
//         ))
//     }

//     fn get_id(&self) -> &'static str {
//         MANAGES_MANAGE_ID
//     }

//     fn get_name(&self) -> String {
//         "ManagesManager".to_string()
//     }

//     async fn get_manage(&self) -> Arc<RwLock<Manage>> {
//         match MANAGES_MANAGE.get() {
//             Some(r) => r.clone(),
//             None => {
//                 let id_str = MANAGES_MANAGE_ID.to_string();

//                 let m_doc =
//                     match entity::get_entity_by_id(MANAGES_MANAGE_ID.to_string(), &id_str, &[], &[]).await {
//                         Ok(r) => r,
//                         Err(e) => panic!("{} {}", e.operation(), e.details()),
//                     };

//                 let manage: Manage = manage_from_document(m_doc).unwrap();
//                 let ma = Arc::new(RwLock::new(manage));
//                 MANAGES_MANAGE.set(ma.clone());
//                 ma
//             }
//         }
//     }

//     async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
//         match MANAGES_MANAGE_DOCUMENT.get() {
//             Some(r) => r.clone(),
//             None => {
//                 let id_str = MANAGES_MANAGE_ID.to_string();

//                 let m_doc =
//                     match entity::get_entity_by_id(MANAGES_MANAGE_ID.to_string(), &id_str, &[], &[]).await {
//                         Ok(r) => r,
//                         Err(e) => panic!("{} {}", e.operation(), e.details()),
//                     };

//                 let m = Arc::new(RwLock::new(m_doc));
//                 if let Err(err) = MANAGES_MANAGE_DOCUMENT.set(m.clone()) {
//                     error!("{} {}", t!("初始化缓存失败"), MANAGES_MANAGE_ID);
//                     panic!("{} {}", t!("初始化缓存失败"), MANAGES_MANAGE_ID);
//                 };
//                 m
//             }
//         }
//     }
// }

#[async_trait]
impl HardCodedInterface for ManagesManager {}
#[async_trait]
impl EntityInterface for ManagesManager {}
