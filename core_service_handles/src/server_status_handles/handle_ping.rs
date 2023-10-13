use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::error;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio;
use dependencies_sync::tokio_stream::{wrappers::ReceiverStream, StreamExt};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::{
    PHONE_AREA_CODES_CODE_FIELD_ID, PHONE_AREA_CODES_USING_AREAS_FIELD_ID,
};
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::{request_account_context, validate_auth_token, validate_has_role_group};

use dependencies_sync::tonic::{Request, Response, Status};
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
    let (account_id, _groups, role_group) = request_account_context(request.metadata());

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
                        println!("id: {}, account_id: {}, index: {}, eclapse:{}", device_id.to_string(), account_id, index, eclapse_time);
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
                    if resp_tx.send(resp).await.is_err() {
                        // 发送失败
                        error!("{}", t!("ping返回失败"));
                        break;
                    };
                }
                Err(e) => {
                    error!("{}", t!("ping数据错误"));
                    // TODO: 断开连接处理
                    match resp_tx.send(Err(e)).await {
                        Ok(_) => {
                            error!("{}", t!("ping返回失败"));
                        }
                        Err(e) => {
                            error!("{}", t!("ping返回失败"));
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
