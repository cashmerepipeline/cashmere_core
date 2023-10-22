use std::sync::Arc;

use cash_result::{operation_failed, OperationResult};
use dependencies_sync::log::debug;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::rust_i18n::{self, t};
use managers::ManagerTrait;
use managers::Manager;

use crate::{get_managers_map, ManagersMap};
use crate::managers_map::add_managers;

/// 管理管理器
#[derive(Debug, Default)]
pub struct Majordomo;

impl Majordomo {
    /// 新建
    pub fn new() -> Majordomo {
        Majordomo::default()
    }

    /// 取得管理器, 返回Arc指针
    pub fn get_manager_by_id(&self, id: i32) -> Result<Arc<Manager>, OperationResult> {
        let managers = get_managers_map();
        let managers = managers.read();

        managers
            .get(&id)
            .cloned()
            .ok_or(operation_failed(
                format!("get_manager_by_id {}", id),
                "取得管理器失败",
            ))
            .and_then(|manager| {
                debug!("{}: {}-{}", t!("成功获取管理器"), id, manager.get_name());
                Ok(manager)
            })
    }

    /// 取得管理编号表
    pub fn get_manager_ids(&self) -> Vec<i32> {
        let managers_ids: Vec<i32> = self.get_managers_map().read().keys().copied().collect();
        managers_ids
    }

    /// 取得管理表
    pub fn get_managers_map(&self) -> Arc<RwLock<ManagersMap>> {
        get_managers_map()
    }

    /// 设置管理器表
    pub async fn add_managers(
        &self,
        new_managers: Vec<Arc<Manager>>,
    ) -> Result<OperationResult, OperationResult> {
        add_managers(new_managers)
    }

    // TODO: 管理依赖检查，全部管理库加载完成后
    // pub fn check_dependents(&self) -> Result<OperationResult, OperationResult> {}
}
