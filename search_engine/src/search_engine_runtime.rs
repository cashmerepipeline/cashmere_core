use std::sync::{Arc, OnceLock};

use dependencies_sync::{tokio, log};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio::runtime::Runtime;
use dependencies_sync::tokio::sync::oneshot;

static mut SEARCH_ENGINE_RUNTIME: Option<Arc<Runtime>> = None;
static mut SEARCH_ENGINE_RUNTIME_SHUTDOWN: OnceLock<bool> = OnceLock::new();

pub fn get_search_engine_runtime() -> Arc<Runtime> {
    unsafe {
        if SEARCH_ENGINE_RUNTIME.is_some() {
            SEARCH_ENGINE_RUNTIME.clone().unwrap()
        } else {
            SEARCH_ENGINE_RUNTIME.get_or_insert(build_runtime());
            SEARCH_ENGINE_RUNTIME.clone().unwrap()
        }
    }
}

pub fn build_runtime() -> Arc<Runtime> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()
        .unwrap();

    let rt_arc = Arc::new(rt);
    let result = rt_arc.clone();

    // Ctrl+c 终止运行时
    let (tx, shutdown_rx) = oneshot::channel();
    let _sig = tokio::spawn(server_utils::wait_for_terminat_signal(tx));

    // 在新线程中启动一个新的tokio运行时
    std::thread::spawn(move || {
        rt_arc.block_on(async {
            match shutdown_rx.await{
                Ok(()) => {
                    log::info!("{}", t!("收到退出信号"));
                },
                Err(_) => {
                    log::info!("{}", t!("退出信号错误"));
                }
            };

            log::info!("{}", t!("开始退出搜索运行时"));
        });
    });

    result
}

pub fn is_search_engine_runtime_shutdown() -> bool {
    unsafe { *SEARCH_ENGINE_RUNTIME_SHUTDOWN.get_or_init(|| false) }
}
