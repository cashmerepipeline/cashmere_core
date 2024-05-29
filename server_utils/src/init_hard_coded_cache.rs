use dependencies_sync::log::info;
use dependencies_sync::rust_i18n::{self, t};
use managers::hard_coded_cache_interface::HardCodedInterface;
use managers::Manager;
use managers::ManagerInterface;

pub async fn init_hard_coded_cache(manager_arcs: &Vec<&'static Manager>) {
    info!("{}", t!("初始化硬编码缓存"));
    for m in manager_arcs {
        // 初始化硬编码缓存
        if m.is_hard_coded().await && m.get_hard_coded_cache(m.get_id()).await.is_none() {
            panic!("{}: {}", t!("初始化管理缓存失败"), m.get_id());
        }
    }
}
