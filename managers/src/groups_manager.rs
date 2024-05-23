/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 组管理器
Created:  2020-11-28T02:17:47.146Z
Modified: !date!
*/

use std::sync::{Arc, OnceLock};


use dependencies_sync::once_cell::sync::Lazy;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::log::{error, info, warn};
use dependencies_sync::tonic::async_trait;

use dependencies_sync::parking_lot::RwLock;

use crate::entity_interface::EntityInterface;
use crate::hard_coded_cache_interface::HardCodedInterface;
use crate::{declare_common_manager_interface, AllManagerInterface, ManagerInterface};

use cash_core::{manage_from_document, Manage};
use cash_result::*;

use crate::declare_get_manager;
use dependencies_sync::bson::Document;


use crate::manager::Manager;

use manage_define::manage_ids::GROUPS_MANAGE_ID;
use manage_define::manage_ids::MANAGES_MANAGE_ID;

#[derive(Default)]
pub struct GroupsManager;

/// 缓存
static GROUPS_MANAGE: OnceLock<Arc<RwLock<Manage>>> = OnceLock::new();
static GROUPS_MANAGE_DOCUMENT: OnceLock<Arc<RwLock<Document>>> = OnceLock::new();

/// 管理器
static INNER: Lazy<Arc<Box<dyn AllManagerInterface>>> =
    Lazy::new(|| Arc::new(Box::new(GroupsManager {})));
static GROUPS_MANAGER: OnceLock<Manager> = OnceLock::new();

// 声明管理器取得函数
declare_get_manager!(GroupsManager, GROUPS_MANAGER, INNER.clone());

declare_common_manager_interface!(
    GroupsManager,
    GROUPS_MANAGE,
    GROUPS_MANAGE_DOCUMENT,
    GROUPS_MANAGE_ID
);

#[async_trait]
impl AllManagerInterface for GroupsManager {}
#[async_trait]
impl HardCodedInterface for GroupsManager {}
#[async_trait]
impl EntityInterface for GroupsManager {}
