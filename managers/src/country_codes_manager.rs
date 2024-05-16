use std::sync::{Arc, OnceLock};


use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::log::error;
use dependencies_sync::tonic::async_trait;

use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::bson::Document;

use super::ManagerTrait;

use cash_core::{manage_from_document, Manage};
use cash_result::*;
use manage_define::manage_ids::*;

use crate::{declare_get_manager, declare_common_cache_interface};
use crate::entity_cache_map::{cache_init_cache, EntityCacheInterface, MEntityCacheMap};
use crate::manager::Manager;
use crate::manager_inner::ManagerInner;
use manage_define::manage_ids::MANAGES_MANAGE_ID;

#[derive(Default)]
pub struct CountryCodesManager;

/// 缓存
static mut COUNTRY_CODES_MANAGE: Option<Arc<RwLock<Manage>>> = None;
static mut COUNTRY_CODES_MANAGE_DOCUMENT: Option<Arc<RwLock<Document>>> = None;
static mut ENTITY_CACHE: OnceLock<MEntityCacheMap> = OnceLock::new();

/// 管理器
static mut COUNTRY_CODES_MANAGER: Option<Arc<Manager>> = None;

// 声明管理器取得函数
declare_get_manager!(CountryCodesManager, COUNTRY_CODES_MANAGER);

// 实现接口
#[async_trait]
impl ManagerTrait for CountryCodesManager {
    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        Err(operation_failed(
            "unregister",
            format!(
                "{}-{}-{}",
                t!("管理器不能被注销"),
                self.get_id(),
                self.get_name()
            ),
        ))
    }

    fn get_id(&self) -> &'static str {
        COUNTRY_CODES_MANAGE_ID
    }

    fn get_name(&self) -> String {
        "CountryCodesManager".to_string()
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        unsafe {
            if COUNTRY_CODES_MANAGE.is_some() {
                COUNTRY_CODES_MANAGE.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = COUNTRY_CODES_MANAGE_ID.to_string();
                let m_doc =
                    match entity::get_entity_by_id(&collection_name, &id_str, &[], &[]).await {
                        Ok(r) => r,
                        Err(e) => panic!("{} {}", e.operation(), e.details()),
                    };
                let manage: Manage = manage_from_document(m_doc).unwrap();
                COUNTRY_CODES_MANAGE.replace(Arc::new(RwLock::new(manage)));
                COUNTRY_CODES_MANAGE.clone().unwrap()
            }
        }
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        unsafe {
            if COUNTRY_CODES_MANAGE_DOCUMENT.is_some() {
                COUNTRY_CODES_MANAGE_DOCUMENT.clone().unwrap()
            } else {
                let collection_name = MANAGES_MANAGE_ID.to_string();
                let id_str = COUNTRY_CODES_MANAGE_ID.to_string();
                let m_doc =
                    match entity::get_entity_by_id(&collection_name, &id_str, &[], &[]).await {
                        Ok(r) => r,
                        Err(e) => panic!("{} {}", e.operation(), e.details()),
                    };

                COUNTRY_CODES_MANAGE_DOCUMENT.replace(Arc::new(RwLock::new(m_doc)));
                COUNTRY_CODES_MANAGE_DOCUMENT.clone().unwrap()
            }
        }
    }
}

declare_common_cache_interface!(CountryCodesManager, ENTITY_CACHE, COUNTRY_CODES_MANAGE_ID);
/* 
impl EntityCache for CountryCodesManager {
    fn has_cache() -> bool {
        true
    }

    async fn get_cache() -> Option<&'static MEntityCacheMap<'static>> {
        unsafe {
            if ENTITY_CACHE.get().is_some() {
                return Some(ENTITY_CACHE.get().unwrap());
            }

            let cache = if let Some(r) = cache_init_cache(COUNTRY_CODES_MANAGE_ID).await {
                r
            } else {
                error!("{} {}", t!("初始化缓存失败"), COUNTRY_CODES_MANAGE_ID);
                return None;
            };

            Some(&ENTITY_CACHE.get_or_init(|| cache))
        }
    }
}
 */