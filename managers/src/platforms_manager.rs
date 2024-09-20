use std::sync::{Arc, OnceLock};

use dependencies_sync::log::error;
use dependencies_sync::once_cell::sync::Lazy;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use cash_core::{manage_from_document, Manage};
use cash_result::*;
use dependencies_sync::bson::Document;
use dependencies_sync::parking_lot::RwLock;
use manage_define::manage_ids::MANAGES_MANAGE_ID;
use crate::hard_coded_cache_interface::HardCodedInterface;
use crate::{
    declare_common_manager_interface, declare_get_manager, entity_interface::EntityInterface,
    AllManagerInterface, Manager, ManagerInterface,
};

use manage_define::manage_ids::PLATFORMS_MANAGE_ID;

#[derive(Default)]
pub struct PlatformsManager;

/// 缓存
static PLATFORMS_MANAGE: OnceLock<Arc<RwLock<Manage>>> = OnceLock::new();
static PLATFORMS_MANAGE_DOCUMENT: OnceLock<Arc<RwLock<Document>>> = OnceLock::new();

/// 管理器
static INNER: Lazy<Arc<Box<dyn AllManagerInterface>>> =
    Lazy::new(|| Arc::new(Box::new(PlatformsManager {})));
static PLATFORMS_MANAGER: OnceLock<Manager> = OnceLock::new();

// 声明管理器取得函数
declare_get_manager!(
    EcommercePlatformsManager,
    PLATFORMS_MANAGER,
    INNER.clone()
);

declare_common_manager_interface!(
    PlatformsManager,
    PLATFORMS_MANAGE,
    PLATFORMS_MANAGE_DOCUMENT,
    PLATFORMS_MANAGE_ID
);

#[async_trait]
impl AllManagerInterface for PlatformsManager {}
#[async_trait]
impl HardCodedInterface for PlatformsManager {}
#[async_trait]
impl EntityInterface for PlatformsManager {}
