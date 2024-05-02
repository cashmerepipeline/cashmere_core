use dependencies_sync::bson::doc;

use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::{debug, error};
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


/// 心跳
/// 客户端参数设置需要匹配
async fn handle_ping(request: RequestStream<PingRequest>) -> StreamResponseResult<PingResponse> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let mut in_stream = request.into_inner();
    let (resp_tx, resp_rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        // 前次时间
        let max_interval = std::time::Duration::from_secs(32);
        // 每轮次数
        let count_per_round = 32u8;

        let mut pre_ping_time: u64 = 0;
        // 当前次数
        let mut count: u8 = 0;
        // 总时间/轮
        let mut total_time_per_round: u64 = 0;
        let mut round_start_time = std::time::SystemTime::now();

        while let Some(result) = in_stream.next().await {
            match result {
                Ok(ping) => {
                    let index = &ping.index;
                    let device_id = &ping.device_id;
                    let time = &ping.time;
                    
                    // 开始计时
                    if index != &0u64 {
                        let eclapse_time = time - pre_ping_time;

                        total_time_per_round += eclapse_time;
                        count += 1;

                        if count == count_per_round {
                            let average_time = total_time_per_round / count as u64;

                            // 30秒内不再回应
                            if round_start_time.elapsed().unwrap() > max_interval{
                                // TODO: 可能是异常访问判断，断开连接
                                round_start_time = std::time::SystemTime::now();
                                continue;
                            }

                            debug!(
                                "device_id: {}, account_id: {}, index: {}, avg_interval:{}",
                                device_id, account_id, index, average_time
                            );

                            // 重置
                            count = 0;
                            total_time_per_round = 0;
                        }
                    }

                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;

                    let resp = Ok(PingResponse {
                        // 返回相同的index
                        index: *index,
                        time: now,
                    });

                    pre_ping_time = *time;

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
