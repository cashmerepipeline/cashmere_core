use std::sync::Arc;

use dependencies_sync::tokio;
use dependencies_sync::tokio::runtime::Runtime;

pub fn build_runtime() -> Arc<Runtime> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
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
