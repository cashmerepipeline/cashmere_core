use dependencies_sync::log::info;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio::signal;
use dependencies_sync::tokio::sync::oneshot::{self};

// 退出程序信号
pub async fn wait_for_terminat_signal(tx: oneshot::Sender<()>) {
    let _ = signal::ctrl_c().await;
    info!("{}: SIGINT, {}", t!("事件发生"), t!("开始终止程序"));
    let _ = tx.send(());
}
