use std::sync::Arc;

use cash_result::{operation_failed, OperationResult};
use dependencies_sync::log::debug;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::rust_i18n::{self, t};
use managers::Manager;
use managers::ManagerInterface;

use crate::managers_map::add_managers;
use crate::{get_managers_map, ManagersMap};

/// 管理管理器
#[derive(Debug, Default)]
pub struct Majordomo;

impl Majordomo {
    /// 新建
    pub fn new() -> Majordomo {
        Majordomo
    }

    /// 取得管理器, 返回Arc指针
    pub fn get_manager_by_id(&self, id: &str) -> Result<&'static Manager, OperationResult> {
        let managers_arc = get_managers_map();
        let managers = managers_arc.read();

        managers
            .get(&id)
            .ok_or(operation_failed(
                format!("get_manager_by_id {}", id),
                "取得管理器失败",
            ))
            .cloned()
            .map(|manager| {
                debug!("{}: {}-{}", t!("成功获取管理器"), id, manager.get_name());
                manager
            })
    }

    /// 取得管理编号表
    pub fn get_manager_ids(&self) -> Vec<&str> {
        let managers_ids: Vec<&str> = self.get_managers_map().read().keys().copied().collect();
        managers_ids
    }

    /// 取得管理表
    pub fn get_managers_map(&self) -> Arc<RwLock<ManagersMap>> {
        get_managers_map()
    }

    /// 设置管理器表
    pub async fn add_managers(
        &self,
        new_managers: Vec<&'static Manager>,
    ) -> Result<OperationResult, OperationResult> {
        add_managers(new_managers)
    }
    

    // TODO: 管理依赖检查，全部管理库加载完成后
    // pub fn check_dependents(&self) -> Result<OperationResult, OperationResult> {}
}
