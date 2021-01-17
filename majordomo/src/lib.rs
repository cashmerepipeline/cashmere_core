/*
Author: 闫刚 (yes7rose@sina.com)
managers.rs (c) 2020
Desc: 管理器包裹
Created:  2020-11-29T04:53:15.516Z
Modified: !date!
*/

use std::sync::Arc;
use std::{collections::HashMap};

use bson::{Document};
use parking_lot::RwLock;

use managers::{self, Manager};
use managers::traits::ManagerTrait;
use cash_core::ids::MANAGES_MANAGE_ID;
use cash_core::field::ids::MANAGES_SCHEMA_FIELD_ID;
use cash_core::results::*;
use cash_core::field::PropertyField;
use std::ops::Deref;

// use log::info;

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
            .get(&id)
            .map(|v| {
                v.clone()
            })
            .ok_or(operation_failed("get_manager_by_id", "取得管理器失败"))
    }

    /// 取得管理编号表
    pub async fn get_manager_ids(&self) -> Vec<i32> {
        let managers_ids: Vec<i32> =
            self.get_managers_map().await
                .read()
                .keys()
                .map(|x| x.clone())
                .collect();
        managers_ids
    }

    /// 取得管理表
    pub async fn get_managers_map(&self) -> Arc<RwLock<ManagersMap>> {
        get_managers_map().await.clone()
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


/// 取得管理映射表
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
    let majordomo = Majordomo::new();

    get_managers_map().await;

    Arc::new(majordomo)
}

/// 初始化管理器映射表
async fn init_managers_map() -> Arc<RwLock<ManagersMap>> {
    let managers = managers::get_managers().await;
    if managers.is_empty() { panic!("没有任何管理器加载") }

    let mut m_map: ManagersMap = HashMap::new();

    for m in managers.iter() {
        m_map.insert(m.get_manager_id().clone(), m.clone());
    }

    Arc::new(RwLock::new(m_map))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
