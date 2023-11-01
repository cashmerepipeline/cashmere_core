use std::sync::Arc;

use dependencies_sync::tokio;
use dependencies_sync::tokio::runtime::Runtime;

static mut SEARCH_ENGINE_RUNTIME: Option<Arc<Runtime>> = None;

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

    // 在新线程中启动一个新的tokio运行时
    std::thread::spawn(move || {
        rt_arc.block_on(async {
            tokio::signal::ctrl_c().await.unwrap();
        });
    });

    result
}
