

use std::sync::{Arc, OnceLock};

use dependencies_sync::once_cell::sync::Lazy;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::log::{error, info, warn};
use dependencies_sync::tonic::async_trait;

use crate::entity_interface::EntityInterface;
use crate::hard_coded_cache_interface::HardCodedInterface;
use crate::{
    declare_common_manager_interface, declare_get_manager, AllManagerInterface, Manager,
    ManagerInterface,
};
use cash_core::{manage_from_document, Manage};
use cash_result::*;
use dependencies_sync::bson::Document;
use dependencies_sync::parking_lot::RwLock;


use manage_define::manage_ids::RECOMMENDS_MANAGE_ID;
use manage_define::manage_ids::MANAGES_MANAGE_ID;

#[derive(Default)]
pub struct RecommendsManager;

/// 缓存
static RECOMMENDS_MANAGE: OnceLock<Arc<RwLock<Manage>>> = OnceLock::new();
static RECOMMENDS_MANAGE_DOCUMENT: OnceLock<Arc<RwLock<Document>>> = OnceLock::new();

/// 管理器
static INNER: Lazy<Arc<Box<dyn AllManagerInterface>>> =
    Lazy::new(|| Arc::new(Box::new(RecommendsManager {})));
static RECOMMENDS_MANAGER: OnceLock<Manager> = OnceLock::new();

// 声明管理器取得函数
declare_get_manager!(RecommendsManager, RECOMMENDS_MANAGER, INNER.clone());

declare_common_manager_interface!(
    RecommendsManager,
    RECOMMENDS_MANAGE,
    RECOMMENDS_MANAGE_DOCUMENT,
    RECOMMENDS_MANAGE_ID
);

#[async_trait]
impl AllManagerInterface for RecommendsManager {}
#[async_trait]
impl HardCodedInterface for RecommendsManager {}
#[async_trait]
impl EntityInterface for RecommendsManager {}
