use dependencies_sync::{
    log::error,
    rust_i18n::{self, t},
    tokio::sync::mpsc::Sender,
    tonic::Status,
};

pub async fn send_stream_response<T>(resp_tx: &Sender<Result<T, Status>>, resp: T) {
    if let Err(err) = resp_tx.send(Ok(resp)).await {
        error!("{}: {:?}", t!("发送数据失败"), err.to_string());
    }
}
