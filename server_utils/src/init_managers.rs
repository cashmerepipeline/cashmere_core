/*
   初始化管理器表
*/

use std::sync::Arc;

use dependencies_sync::log::info;
use dependencies_sync::rust_i18n::{self, t};
use majordomo::get_majordomo;
use managers::Manager;
use managers::ManagerInterface;

pub async fn init_managers(manager_arcs: Vec<&'static Manager>) {
    info!("{}", t!("初始化主管"));
    let majordomo_lock = get_majordomo();

    info!("{}", t!("初始化管理映射表"));
    let managers_map_arc = majordomo_lock.get_managers_map();
    let mut manages_map = managers_map_arc.write();
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

        // 初始化硬编码缓存
        /* if m.inner.is_hard_coded().await && m.inner.get_hard_coded_cache(manager_id).await.is_none() {
            panic!("{}: {}", t!("初始化管理缓存失败"), manager_id);
        } */

        manages_map.insert(manager_id, m);
    }

    // 显示加载的管理
    for (k, m) in manages_map.iter() {
        info!("{}: {} {}", t!("已加载管理"), k, m.get_name())
    }
}
