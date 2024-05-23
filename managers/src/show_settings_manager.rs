/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 人管理器
Created:  2020-11-28T02:17:47.146Z
Modified: !date!
*/

use std::sync::Arc;

use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::log::{error, info, warn};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::bson::Document;

use cash_core::{manage_from_document, Manage};
use cash_result::*;
use manage_define::manage_ids::SHOW;
use  manage_define::manage_ids::MANAGES_MANAGE_ID;
use crate::{Manager, ManagerInner, traits::ManagerTrait,declare_get_manager};

#[derive(Default)]
pub struct ShowSettingsManager;

/// 缓存
static TAGS_MANAGE: OnceLock<Arc<RwLock<Manage>>> = OnceLock::new();
static TAGS_MANAGE_DOCUMENT: OnceLock<Arc<RwLock<Document>>> = OnceLock::new();

/// 管理器
static INNER: Lazy<Arc<Box<dyn AllManagerInterface>>> =
    Lazy::new(|| Arc::new(Box::new(TagsManager {})));
static TAGS_MANAGER: OnceLock<Manager> = OnceLock::new();

// 声明管理器取得函数
declare_get_manager!(TagsManager, TAGS_MANAGER, INNER.clone());

declare_common_manager_interface!(
    TagsManager,
    TAGS_MANAGE,
    TAGS_MANAGE_DOCUMENT,
    TAGS_MANAGE_ID
);

#[async_trait]
impl AllManagerInterface for TagsManager {}
#[async_trait]
impl HardCodedInterface for TagsManager {}
#[async_trait]
impl EntityInterface for TagsManager {}
