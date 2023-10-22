/*
   初始化管理器表
*/

use std::sync::Arc;

use dependencies_sync::log::info;
use dependencies_sync::rust_i18n::{self, t};

use managers::Manager;
use managers::ManagerTrait;

use crate::majordomo_arc::get_majordomo;

pub async fn init_managers(manager_arcs: Vec<Arc<Manager>>) {
    info!("初始化主管");
    let majordomo_lock = get_majordomo();

    info!("初始化管理映射表");
    let managers_map_lock = majordomo_lock.get_managers_map();
    let mut manages_map = managers_map_lock.write();
    for m in manager_arcs {
        let manager_id = m.get_id();
        // 管理编号不能相同
        if manages_map.contains_key(&manager_id) {
            panic!(
                "{}: {}, {}",
                t!("管理编号重复"),
                t!("请检查管理编号指定"),
                manager_id
            );
        }

        if m.has_cache() {
            if m.init_cache().await.is_err() {
                panic!("{}: {}", t!("初始化管理缓存失败"), t!("请检查管理编号指定"));
            };
        }

        manages_map.insert(manager_id, m.clone());
    }

    // 显示加载的管理
    for (k, m) in manages_map.iter() {
        info!("已加载管理: {} {}", k, m.get_name())
    }
}
