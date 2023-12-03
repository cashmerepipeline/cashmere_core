use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::error;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio;
use dependencies_sync::tokio_stream::{wrappers::ReceiverStream, StreamExt};
use dependencies_sync::tonic::async_trait;

use manage_define::cashmere::*;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Response, Status};
use service_utils::types::{RequestStream, ResponseStream, StreamResponseResult};

#[async_trait]
pub trait HandlePing {
    /// 新建管理属性
    async fn handle_ping(
        &self,
        request: RequestStream<PingRequest>,
    ) -> StreamResponseResult<PingResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_ping)
            .await
    }
}

async fn validate_view_rules(
    request: RequestStream<PingRequest>,
) -> Result<RequestStream<PingRequest>, Status> {
    // 不需要权限
    Ok(request)
}

async fn validate_request_params(
    request: RequestStream<PingRequest>,
) -> Result<RequestStream<PingRequest>, Status> {
    // let time = &request.get_ref().time;
    // let device_id = &request.get_ref().device_id;

    // // 必须>=0
    // if time < &0u64 {
    //     return Err(Status::invalid_argument(t!("时间必须大于0")));
    // }

    // // 设备id不能为空
    // if device_id.is_empty() {
    //     return Err(Status::invalid_argument(t!("设备id不能为空")));
    // }

    // TODO: 是否检查时间间隔

    Ok(request)
}

async fn handle_ping(request: RequestStream<PingRequest>) -> StreamResponseResult<PingResponse> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let mut in_stream = request.into_inner();
    let (resp_tx, resp_rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        while let Some(result) = in_stream.next().await {
            match result {
                Ok(ping) => {
                    let index = &ping.index;
                    let device_id = &ping.device_id;
                    let time = &ping.time;

                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;
                    if index != &0u64 {
                        let eclapse_time = now - time;
                        //TODO: 根据网速进行动态加载平衡分组
                        println!(
                            "id: {}, account_id: {}, index: {}, eclapse:{}",
                            device_id, account_id, index, eclapse_time
                        );
                    }

                    // 5秒一次
                    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;

                    let resp = Ok(PingResponse {
                        index: *index + 1,
                        time: now,
                    });

                    // 发送
                    if let Err(e) = resp_tx.send(resp).await {
                        // 发送失败
                        error!("{}: {}", t!("ping返回失败"), e);
                        break;
                    };
                }
                Err(e) => {
                    error!("{}: {}", t!("ping数据错误"), e);
                    // TODO: 断开连接处理和断线重连
                    match resp_tx.send(Err(e)).await {
                        Ok(_) => {
                            error!("{}", t!("ping返回失败"));
                        }
                        Err(e) => {
                            error!("{}: {}", t!("ping返回失败"), e);
                        }
                    }
                    break;
                }
            }
        }
    });

    let resp_stream = ReceiverStream::new(resp_rx);

    Ok(Response::new(
        Box::pin(resp_stream) as ResponseStream<PingResponse>
    ))
}
