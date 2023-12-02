use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::error;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio;
use dependencies_sync::tokio::sync::mpsc::Sender;
use dependencies_sync::tokio_stream::{wrappers::ReceiverStream, StreamExt};
use dependencies_sync::tonic::async_trait;

use majordomo::get_majordomo;
use manage_define::cashmere::*;
use managers::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Response, Status};
use service_utils::types::{RequestStream, ResponseStream, StreamResponseResult};
use validates::validate_manage_id;

use super::get_manage_entities_page;
use super::send_stream_response::send_stream_response;
use super::send_stream_error::send_stream_error;

#[async_trait]
pub trait HandleInteractiveGetEntities {
    /// 新建管理属性
    async fn handle_interactive_get_entities(
        &self,
        request: RequestStream<InteractiveGetEntitiesRequest>,
    ) -> StreamResponseResult<InteractiveGetEntitiesResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_interactive_entities_stream)
            .await
    }
}

async fn validate_view_rules(
    request: RequestStream<InteractiveGetEntitiesRequest>,
) -> Result<RequestStream<InteractiveGetEntitiesRequest>, Status> {
    // 不需要权限
    Ok(request)
}

async fn validate_request_params(
    request: RequestStream<InteractiveGetEntitiesRequest>,
) -> Result<RequestStream<InteractiveGetEntitiesRequest>, Status> {
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

async fn handle_interactive_entities_stream(
    request: RequestStream<InteractiveGetEntitiesRequest>,
) -> StreamResponseResult<InteractiveGetEntitiesResponse> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let mut in_stream = request.into_inner();
    let (resp_tx, resp_rx) =
        tokio::sync::mpsc::channel::<Result<InteractiveGetEntitiesResponse, Status>>(1);

    tokio::spawn(async move {
        while let Some(request) = in_stream.next().await {
            match request {
                Ok(request) => {
                    let manage_id = &request.manage_id;
                    let page_index = &request.page_index;
                    let match_doc = &request.match_doc;
                    let sort_doc = &request.sort_doc;
                    let no_present_fields = &request.no_present_fields;

                    if let Err(err) = validate_manage_id(manage_id).await {
                        if let Err(_) = resp_tx.send(Err(err)).await {
                            error!("{}", t!("反馈错误失败"));
                        }
                    }

                    let majordomo_arc = get_majordomo();
                    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

                    let match_doc: Document =
                        bson::from_slice(&match_doc).unwrap_or(Document::new());
                    let sort_doc: Document = bson::from_slice(&sort_doc).unwrap_or(Document::new());

                    // zh: 没有实体，返回空表
                    let count = if let Ok(c) = manager.count_entity(match_doc.clone()).await {
                        c
                    } else {
                        let empty_resp = InteractiveGetEntitiesResponse {
                            page_index: 0,
                            entities: vec![],
                            total_count: 0,
                        };

                        send_stream_response(&resp_tx, empty_resp).await;

                        continue;
                    };

                    let result = get_manage_entities_page(
                        &account_id,
                        &role_group,
                        &manage_id,
                        &match_doc,
                        &Some(sort_doc),
                        page_index,
                        no_present_fields,
                    )
                    .await;

                    match result {
                        Ok(entities) => {
                            let b_d = entities
                                .iter()
                                .map(|x| bson::to_vec(x).unwrap())
                                .collect::<Vec<Vec<u8>>>();
                            let resp = InteractiveGetEntitiesResponse {
                                page_index: *page_index,
                                entities: b_d,
                                total_count: count,
                            };
                            send_stream_response(&resp_tx, resp).await;
                        }
                        Err(e) => {
                            error!("{}: {}", t!("交互查询数据错误"), e.details());
                            send_stream_error(&resp_tx, Status::aborted(t!("从数据库取得实体列表失败")))
                                .await;
                        }
                    };
                }
                Err(e) => {
                    error!("{}: {}", t!("交互查询数据错误"), e);

                    // TODO: 断开连接处理和断线重连
                    match resp_tx.send(Err(e)).await {
                        Ok(_) => (),
                        Err(e) => {
                            error!("{}: {}", t!("反馈错误失败"), e);
                        }
                    }
                    break;
                }
            }
        }
    });

    let resp_stream = ReceiverStream::new(resp_rx);

    Ok(Response::new(
        Box::pin(resp_stream) as ResponseStream<InteractiveGetEntitiesResponse>
    ))
}
