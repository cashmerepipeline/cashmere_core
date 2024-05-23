/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 人管理器
Created:  2020-11-28T02:17:47.146Z
Modified: !date!
*/

use std::sync::{Arc, OnceLock};

use dependencies_sync::once_cell::sync::Lazy;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::log::{error, info, warn};
use dependencies_sync::tonic::async_trait;

use dependencies_sync::parking_lot::RwLock;

use super::ManagerInterface;

use cash_core::{manage_from_document, Manage};
use cash_result::*;

use manage_define::manage_ids::PERSONS_MANAGE_ID;
use manage_define::manage_ids::MANAGES_MANAGE_ID;

use crate::entity_interface::EntityInterface;
use crate::hard_coded_cache_interface::HardCodedInterface;
use crate::manager::Manager;
use crate::{declare_common_manager_interface, declare_get_manager, AllManagerInterface};
use dependencies_sync::bson::Document;

#[derive(Default)]
pub struct PersonsManager;

/// 缓存
static PERSONS_MANAGE: OnceLock<Arc<RwLock<Manage>>> = OnceLock::new();
static PERSONS_MANAGE_DOCUMENT: OnceLock<Arc<RwLock<Document>>> = OnceLock::new();

/// 管理器
static INNER: Lazy<Arc<Box<dyn AllManagerInterface>>> =
    Lazy::new(|| Arc::new(Box::new(PersonsManager {})));
static PERSONS_MANAGER: OnceLock<Manager> = OnceLock::new();

// 声明管理器取得函数
declare_get_manager!(PersonsManager, PERSONS_MANAGER, INNER.clone());

declare_common_manager_interface!(
    PersonsManager,
    PERSONS_MANAGE,
    PERSONS_MANAGE_DOCUMENT,
    PERSONS_MANAGE_ID
);

#[async_trait]
impl AllManagerInterface for PersonsManager {}
#[async_trait]
impl HardCodedInterface for PersonsManager {}
#[async_trait]
impl EntityInterface for PersonsManager {}
