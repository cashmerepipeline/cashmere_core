use dependencies_sync::bson::{self, doc, Document, Timestamp};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::error;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio;
use dependencies_sync::tokio_stream::{self, iter, StreamExt};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use view::{self, filter_can_read_fields};

use service_utils::types::{ResponseStream, StreamResponseResult, UnaryResponseResult};

#[async_trait]
pub trait HandleCheckUpdatesLaterThenTime {
    /// 取得管理记录数量
    async fn handle_check_updates_later_then_time(
        &self,
        request: Request<CheckUpdatesLaterThenTimeRequest>,
    ) -> StreamResponseResult<CheckUpdatesLaterThenTimeResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_check_updates_later_then_time)
            .await
    }
}

async fn validate_view_rules(
    request: Request<CheckUpdatesLaterThenTimeRequest>,
) -> Result<Request<CheckUpdatesLaterThenTimeRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<CheckUpdatesLaterThenTimeRequest>,
) -> Result<Request<CheckUpdatesLaterThenTimeRequest>, Status> {
    let timestamp = &request.get_ref().timestamp;

    if timestamp.is_empty() {
        return Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("时间戳不能为空"),
            "check_update_later_then_time"
        )));
    }
    // TODO:深入检查时间戳格式是否正确

    Ok(request)
}

async fn handle_check_updates_later_then_time(
    request: Request<CheckUpdatesLaterThenTimeRequest>,
) -> StreamResponseResult<CheckUpdatesLaterThenTimeResponse> {
    let (_account_id, _groups, _role_group) = request_account_context(request.metadata());

    let manage_id = &request.get_ref().manage_id;
    let timestamp = &request.get_ref().timestamp;
    let ascending_order = &request.get_ref().ascending_order;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

    let timestamp_doc: Document = bson::from_slice(timestamp).unwrap();
    let timestamp = timestamp_doc.get_timestamp("value").unwrap();

    let query_doc = doc! {
    MODIFY_TIMESTAMP_FIELD_ID.to_string(): {"$gt": timestamp},
        };

    let sort_doc = if *ascending_order {
        doc! {
            MODIFY_TIMESTAMP_FIELD_ID.to_string(): 1,
        }
    } else {
        doc! {
            MODIFY_TIMESTAMP_FIELD_ID.to_string(): -1,
        }
    };

    let project_doc = doc! {
        ID_FIELD_ID.to_string(): 1,
        MODIFY_TIMESTAMP_FIELD_ID.to_string(): 1,
    };

    let mut query_cursor = match manager
        .get_query_cursor(query_doc, Some(project_doc), Some(sort_doc))
        .await
    {
        Ok(cursor) => cursor,
        Err(err) => {
            error!(
                "{}-{}: {}",
                t!("数据库查询更新失败"),
                manage_id,
                err.details()
            );

            return Err(Status::data_loss(format!(
                "{}-{}",
                t!("数据库查询更新失败"),
                manage_id
            )));
        }
    };

    // 创建返回流
    let (resp_tx, resp_rx) = tokio::sync::mpsc::channel(4);

    tokio::spawn(async move {
        // 最多获取1000个
        let mut limit_count = 0;
        let mut ids = vec![];
        while let Some(result) = query_cursor.next().await {
            match result {
                Ok(d) => {
                    let id = d.get_str(ID_FIELD_ID.to_string()).unwrap().to_string();
                    ids.push(id);
                }
                Err(e) => {
                    error!("{}: {}", t!("取得查询失败"), e);
                }
            }

            // 满20，发送到返回流
            if ids.len() >= 20 {
                let resp = CheckUpdatesLaterThenTimeResponse {
                    entity_ids: ids.clone(),
                };
                resp_tx.send(Ok(resp)).await.unwrap();
                ids.clear();
            }

            // 最多1000条
            if limit_count >= 1000 {
                break;
            }

            limit_count += 1;
        }
        // 发送最后一批
        if !ids.is_empty() {
            let resp = CheckUpdatesLaterThenTimeResponse { entity_ids: ids };
            resp_tx.send(Ok(resp)).await.unwrap();
        }
    });

    let resp_stream = tokio_stream::wrappers::ReceiverStream::new(resp_rx);
    Ok(Response::new(
        Box::pin(resp_stream) as ResponseStream<CheckUpdatesLaterThenTimeResponse>
    ))
}
