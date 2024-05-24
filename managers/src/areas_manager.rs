use std::sync::{Arc, OnceLock};

use dependencies_sync::once_cell::sync::Lazy;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::log::{error, info, warn};

use dependencies_sync::tonic::async_trait;

use dependencies_sync::bson::Document;
use dependencies_sync::parking_lot::RwLock;

use super::ManagerInterface;

use cash_core::{manage_from_document, Manage};
use cash_result::*;
use manage_define::manage_ids::*;

use crate::entity_interface::EntityInterface;
use crate::hard_coded_cache_interface::HardCodedInterface;
use crate::manager::Manager;
use crate::{declare_common_manager_interface, declare_get_manager, AllManagerInterface};

#[derive(Default)]
pub struct AreasManager;

/// 缓存
static AREAS_MANAGE: OnceLock<Arc<RwLock<Manage>>> = OnceLock::new();
static AREAS_MANAGE_DOCUMENT: OnceLock<Arc<RwLock<Document>>> = OnceLock::new();

/// 管理器
static INNER: Lazy<Arc<Box<dyn AllManagerInterface>>> =
    Lazy::new(|| Arc::new(Box::new(AreasManager {})));
static AREAS_MANAGER: OnceLock<Manager> = OnceLock::new();

// 声明管理器取得函数
declare_get_manager!(AreasManager, AREAS_MANAGER, INNER.clone());

declare_common_manager_interface!(
    AreasManager,
    AREAS_MANAGE,
    AREAS_MANAGE_DOCUMENT,
    AREAS_MANAGE_ID
);

#[async_trait]
impl AllManagerInterface for AreasManager {}
#[async_trait]
impl HardCodedInterface for AreasManager {}
#[async_trait]
impl EntityInterface for AreasManager {}

// 实现接口
// #[async_trait]
// impl ManagerInterface for AreasManager {
//     fn unregister(&self) -> Result<OperationResult, OperationResult> {
//         Err(operation_failed(
//             "unregister",
//             format!(
//                 "{}-{}-{}",
//                 t!("管理器不能被注销"),
//                 self.get_id(),
//                 self.get_name()
//             ),
//         ))
//     }

//     fn get_id(&self) -> &'static str {
//         AREAS_MANAGE_ID
//     }

//     fn get_name(&self) -> String {
//         "AreasManager".to_string()
//     }

//     async fn get_manage(&self) -> Arc<RwLock<Manage>> {
//         unsafe {
//             if AREAS_MANAGE.is_some() {
//                 AREAS_MANAGE.clone().unwrap()
//             } else {
//                 let collection_name = MANAGES_MANAGE_ID.to_string();
//                 let id_str = AREAS_MANAGE_ID.to_string();
//                 let m_doc =
//                     match entity::get_entity_by_id(&collection_name, &id_str, &[], &[]).await {
//                         Ok(r) => r,
//                         Err(e) => panic!("{} {}", e.operation(), e.details()),
//                     };
//                 let manage: Manage = manage_from_document(m_doc).unwrap();
//                 AREAS_MANAGE.replace(Arc::new(RwLock::new(manage)));
//                 AREAS_MANAGE.clone().unwrap()
//             }
//         }
//     }

//     async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
//         unsafe {
//             if AREAS_MANAGE_DOCUMENT.is_some() {
//                 AREAS_MANAGE_DOCUMENT.clone().unwrap()
//             } else {
//                 let collection_name = MANAGES_MANAGE_ID.to_string();
//                 let id_str = AREAS_MANAGE_ID.to_string();
//                 let m_doc =
//                     match entity::get_entity_by_id(&collection_name, &id_str, &[], &[]).await {
//                         Ok(r) => r,
//                         Err(e) => panic!("{} {}", e.operation(), e.details()),
//                     };

//                 AREAS_MANAGE_DOCUMENT.replace(Arc::new(RwLock::new(m_doc)));
//                 AREAS_MANAGE_DOCUMENT.clone().unwrap()
//             }
//         }
//     }
// }
