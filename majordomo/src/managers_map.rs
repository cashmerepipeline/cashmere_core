use std::collections::HashMap;
use std::sync::Arc;


use dependencies_sync::parking_lot::RwLock;

use cash_result::*;
use managers::{self};
use managers::Manager;
use managers::ManagerTrait;

/// 管理表类型
pub type ManagersMap = HashMap<&'static str, Arc<Manager>>;

/// 管理管理器表
static mut MANAGERS_MAP: Option<Arc<RwLock<ManagersMap>>> = None;

/// 添加管理器
pub fn add_managers(new_managers: Vec<Arc<Manager>>) -> Result<OperationResult, OperationResult> {
    if new_managers.is_empty() {
        return Ok(operation_succeed("ok"));
    }

    let managers_map_arc = get_managers_map();
    let mut managers_map_lock = managers_map_arc.write();
    for m in new_managers.iter() {
        managers_map_lock.insert(m.get_id(), m.clone());
    }

    Ok(operation_succeed("ok"))
}

/// 取得管理器映射表
pub fn get_managers_map() -> Arc<RwLock<ManagersMap>> {
    unsafe {
        if MANAGERS_MAP.is_none() {
            MANAGERS_MAP.replace(init_managers_map());
            MANAGERS_MAP.clone().unwrap()
        } else {
            MANAGERS_MAP.clone().unwrap()
        }
    }
}

/// 初始化管理器映射表
fn init_managers_map() -> Arc<RwLock<ManagersMap>> {
    let m_map: ManagersMap = HashMap::new();
    Arc::new(RwLock::new(m_map))
}
