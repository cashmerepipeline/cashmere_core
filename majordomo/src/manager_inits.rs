/*
   初始化管理器表
*/

use std::sync::Arc;

use log::info;

use managers::traits::ManagerTrait;
use managers::Manager;
use crate::get_majordomo;

pub async fn init_managers(manager_arcs: Vec<Arc<Manager>>) {
    // 显示加载的管理
    info!("初始化主管");
    let majordomo_lock = get_majordomo().await;

    info!("初始化管理映射表");
    let managers_map_lock = majordomo_lock.get_managers_map().await;
    let mut manages_map = managers_map_lock.write();
    for m in manager_arcs {
        manages_map.insert(m.get_manager_id(), m.clone());
    }

    let mut m_keys = Vec::from_iter(manages_map.keys());
    m_keys.sort();
    for k in m_keys {
        info!("已加载管理: {} {}", k, manages_map.get(&k).unwrap().get_manager_name());
    }
}



