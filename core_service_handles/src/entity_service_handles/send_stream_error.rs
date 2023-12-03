use dependencies_sync::{
    log::error,
    rust_i18n::{self, t},
    tokio::sync::mpsc::Sender,
    tonic::Status,
};

pub async fn send_stream_error<T>(resp_tx: &Sender<Result<T, Status>>, e: Status) {
    match resp_tx.send(Err(e)).await {
        Ok(_) => (),
        Err(e) => {
            error!("{}: {}", t!("反馈错误失败"), e);
        }
    }
}
