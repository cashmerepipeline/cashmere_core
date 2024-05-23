use std::sync::OnceLock;

use dependencies_sync::tokio_util::sync::CancellationToken;

static mut SHUTDOWN_CANCELATION_TOKEN: OnceLock<CancellationToken> = OnceLock::new();

/// zh: 取得cancelation token
pub fn get_shutdown_cancellation_token() -> CancellationToken {
    unsafe {
        SHUTDOWN_CANCELATION_TOKEN
            .get_or_init(CancellationToken::new)
            .clone()
    }
}

/// zh: 取消所有任务
pub fn call_shutdown_cancel() {
    unsafe { SHUTDOWN_CANCELATION_TOKEN.get().unwrap().cancel() }
}
