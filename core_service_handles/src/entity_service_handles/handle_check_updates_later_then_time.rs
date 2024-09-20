use dependencies_sync::bson::{self, doc, Document};

use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::{debug, error};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio;
use dependencies_sync::tokio_stream::{self, StreamExt};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use entity::query_filters::{generate_filter_document, generate_sort_document};
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use managers::entity_interface::EntityInterface;
use managers::hard_coded_cache_interface::HardCodedInterface;
use request_utils::request_account_context;
use service_utils::types::{ResponseStream, StreamResponseResult};
use validates::{validate_field_id, validate_manage_id};
use view::view_rules_map::query_collection_view_rules;
use view::{can_collection_read, can_entity_read};

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
        let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) = view::validates::validate_collection_can_read(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<CheckUpdatesLaterThenTimeRequest>,
) -> Result<Request<CheckUpdatesLaterThenTimeRequest>, Status> {
    let manage_id = request.get_ref().manage_id.clone();
    let timestamp = &request.get_ref().timestamp;
    let filters = &request.get_ref().filters;
    let sorts = &request.get_ref().sorts;

    validate_manage_id(manage_id.as_str()).await?;

    if timestamp.is_empty() {
        return Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("时间戳不能为空"),
            "check_update_later_then_time"
        )));
    }

    // 格式二进制 bson Document 形式{"value": Timestamp()}
    let timestamp_doc: Document = if let Ok(t) = bson::from_slice(timestamp) {
        t
    } else {
        return Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("反序列化时间戳失败"),
            "check_update_later_then_time"
        )));
    };

    if let Err(err) = timestamp_doc.get_timestamp("value") {
        return Err(Status::invalid_argument(format!(
            "{}: {}, {}",
            t!("时间戳格式错误"),
            err,
            "check_update_later_then_time"
        )));
    }

    // 验证过滤条件字段
    if !filters.is_empty() {
        for filter in filters {
            validate_field_id(manage_id.as_str(), &filter.field_id).await?;
        }
    }

    // 验证排序字段
    if !sorts.is_empty() {
        for sort in sorts {
            validate_field_id(manage_id.as_str(), &sort.field).await?;
        }
    }

    Ok(request)
}

/// zh: 根据帐号访问条件，返回最新的更新，
async fn handle_check_updates_later_then_time(
    request: Request<CheckUpdatesLaterThenTimeRequest>,
) -> StreamResponseResult<CheckUpdatesLaterThenTimeResponse> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let manage_id = request.get_ref().manage_id.clone();
    let timestamp = &request.get_ref().timestamp;
    let filters = &request.get_ref().filters;
    let sorts = &request.get_ref().sorts;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id.as_str()).unwrap();

    // 可读检查
    if !can_collection_read(manage_id.as_str(), role_group.as_str()).await {
        return Err(Status::unauthenticated(format!(
            "{}: {}, {}",
            t!("无可见权限设置"),
            manage_id,
            role_group
        )));
    };

    let timestamp_doc: Document = bson::from_slice(timestamp).unwrap();
    let timestamp = timestamp_doc.get_timestamp("value").unwrap();

    let mut query_doc = doc! {
    MODIFY_TIMESTAMP_FIELD_ID.to_string(): {"$gt": timestamp},
        };

    if !filters.is_empty() {
        let filter_doc: Document = if let Ok(r) = generate_filter_document(filters) {
            r
        } else {
            error!("{}: {:?}", t!("生成过滤条件失败"), filters);

            return Err(Status::invalid_argument(format!(
                "{}: {}",
                t!("生成过滤条件失败"),
                manage_id
            )));
        };

        filter_doc.iter().for_each(|(k, v)| {
            query_doc.insert(k, v);
        });
    }

    let sort_doc = generate_sort_document(sorts);
    let sort_doc = if sort_doc.len() > 0 {
        Some(sort_doc)
    } else {
        None
    };

    let mut query_cursor = match manager
        .get_entity_stream(query_doc, &[], sort_doc, None, 0)
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
        let mut infos = vec![];

        while let Some(result) = query_cursor.next().await {
            let entity_id = result.get_str(ID_FIELD_ID.to_string()).unwrap();

            if cfg!(debug_assertions) {
                debug!("{}: {}-{}", t!("取得新实体"), manage_id, entity_id,)
            };

            // 可读过滤
            if !can_entity_read(&manage_id, entity_id, &account_id, &role_group).await {
                if cfg!(debug_assertions) {
                    debug!("{}: {}", t!("实体不可读"), manage_id,);
                }
                continue;
            };

            let mut r = doc! {};

            // 从缓存取得的数据会返回所有数据，需要过滤
            if manager.is_hard_coded().await {
                let e_timestamp = result
                    .get_timestamp(MODIFY_TIMESTAMP_FIELD_ID.to_string())
                    .unwrap();
                if timestamp >= e_timestamp {
                    if cfg!(debug_assertions) {
                        debug!(
                            "{}: {}-{}",
                            t!("不需要拉取"),
                            manage_id,
                            result.get_str(ID_FIELD_ID.to_string()).unwrap()
                        );
                    }
                    continue;
                }
            }

            r.insert(OID_FIELD_ID, result.get_object_id(OID_FIELD_ID).unwrap());
            r.insert(ID_FIELD_ID.to_string(), entity_id);
            r.insert(
                MODIFY_TIMESTAMP_FIELD_ID.to_string(),
                result
                    .get_timestamp(MODIFY_TIMESTAMP_FIELD_ID.to_string())
                    .unwrap(),
            );

            infos.push(bson::to_vec(&r).unwrap());

            // 满20，发送到返回流
            if infos.len() >= 20 {
                let resp = CheckUpdatesLaterThenTimeResponse {
                    results: infos.clone(),
                };
                resp_tx.send(Ok(resp)).await.unwrap();
                infos.clear();
            }

            // 最多1000条
            if limit_count >= 1000 {
                break;
            }

            limit_count += 1;
        }

        // 发送最后一批
        if cfg!(debug_assertions) {
            debug!(
                "{}: {}, {} {}",
                t!("发送最后一批"),
                manage_id,
                infos.len(),
                t!("个")
            );
        }

        if !infos.is_empty() {
            let resp = CheckUpdatesLaterThenTimeResponse { results: infos };
            resp_tx.send(Ok(resp)).await.unwrap();
        }
    });

    let resp_stream = tokio_stream::wrappers::ReceiverStream::new(resp_rx);
    Ok(Response::new(
        Box::pin(resp_stream) as ResponseStream<CheckUpdatesLaterThenTimeResponse>
    ))
}
