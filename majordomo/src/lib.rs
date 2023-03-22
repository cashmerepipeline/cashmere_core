/*
Author: 闫刚 (yes7rose@sina.com)
managers.rs (c) 2020
Desc: 管理器包裹
Created:  2020-11-29T04:53:15.516Z
Modified: !date!
*/

use std::sync::Arc;
use std::{collections::HashMap};
use log::{info};

use parking_lot::RwLock;

use managers::{self, Manager};
use managers::traits::ManagerTrait;

use cash_result::*;

pub use manager_inits::init_managers;

mod manager_inits;

/// 管理表类型
pub type ManagersMap = HashMap<i32, Arc<Manager>>;

/// 管理管理器
#[derive(Debug, Default)]
pub struct Majordomo;

impl Majordomo {
    /// 新建
    pub fn new() -> Majordomo {
        Majordomo::default()
    }

    /// 取得管理器, 返回Arc指针
    pub async fn get_manager_by_id(&self, id: i32) -> Result<Arc<Manager>, OperationResult> {
        let managers = get_managers_map().await;
        let managers = managers.read();
        managers
            .get(&id).cloned()
            .ok_or(operation_failed(format!("get_manager_by_id {}", id), "取得管理器失败"))
    }

    /// 取得管理编号表
    pub async fn get_manager_ids(&self) -> Vec<i32> {
        let managers_ids: Vec<i32> =
            self.get_managers_map().await
                .read()
                .keys().copied()
                .collect();
        managers_ids
    }

    /// 取得管理表
    pub async fn get_managers_map(&self) -> Arc<RwLock<ManagersMap>> {
        get_managers_map().await
    }

    /// 设置管理器表
    pub async fn add_managers(&self, new_managers: Vec<Arc<Manager>>) -> Result<OperationResult, OperationResult> {
        add_managers(new_managers).await
    }


    // TODO: 管理依赖检查，全部管理库加载完成后
    // pub fn check_dependents(&self) -> Result<OperationResult, OperationResult> {}
}


/// 管理管理器表
static mut MAJORDOMO: Option<Arc<Majordomo>> = None;
static mut MANAGERS_MAP: Option<Arc<RwLock<ManagersMap>>> = None;

pub async fn get_majordomo() -> Arc<Majordomo> {
    unsafe {
        // 有数据
        if MAJORDOMO.is_some() {
            MAJORDOMO.clone().unwrap()
        }
        // 没有则新解空表
        else {
            MAJORDOMO.replace(init_majordomo().await);
            MAJORDOMO.clone().unwrap()
        }
    }
}

/// 设置管理器
async fn add_managers(new_managers: Vec<Arc<Manager>>) -> Result<OperationResult, OperationResult> {
    if new_managers.is_empty() { return Ok(operation_succeed("ok")); }

    let managers_map_arc = get_managers_map().await;
    let mut managers_map_lock = managers_map_arc.write();
    for m in new_managers.iter() {
        managers_map_lock.insert(m.get_manager_id(), m.clone());
    }

    Ok(operation_succeed("ok"))
}


/// 取得管理器映射表
async fn get_managers_map() -> Arc<RwLock<ManagersMap>> {
    unsafe {
        if MANAGERS_MAP.is_none() {
            MANAGERS_MAP.replace(init_managers_map().await);
            MANAGERS_MAP.clone().unwrap()
        } else {
            MANAGERS_MAP.clone().unwrap()
        }
    }
}

/// 从设置新建管理管理器
async fn init_majordomo() -> Arc<Majordomo> {
    info!("初始化主管实例");
    let majordomo = Majordomo::new();

    get_managers_map().await;

    Arc::new(majordomo)
}

/// 初始化管理器映射表
async fn init_managers_map() -> Arc<RwLock<ManagersMap>> {
    let m_map: ManagersMap = HashMap::new();
    Arc::new(RwLock::new(m_map))
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
