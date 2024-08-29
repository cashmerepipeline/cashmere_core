use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::{debug, error};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio;
use dependencies_sync::tokio_stream::wrappers::ReceiverStream;
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::general_field_ids::ID_FIELD_ID;
use managers::{entity_interface::EntityInterface, manager_trait::ManagerInterface};
use request_utils::request_account_context;

use dependencies_sync::tokio_stream::{self as stream, StreamExt};
use dependencies_sync::tonic::{Request, Response, Status};
use service_utils::send_stream_response;
use view::{
    self, can_collection_write, can_entity_read, can_entity_write, get_manage_schema_view_mask,
};

use service_utils::types::{ResponseStream, StreamResponseResult};

#[async_trait]
pub trait HandleDeleteEntities {
    /// 取得管理记录数量
    async fn handle_delete_entities(
        &self,
        request: Request<DeleteEntitiesRequest>,
    ) -> StreamResponseResult<DeleteEntitiesResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_delete_entities)
            .await
    }
}

async fn validate_view_rules(
    request: Request<DeleteEntitiesRequest>,
) -> Result<Request<DeleteEntitiesRequest>, Status> {
    // 只在debug模式下可用
    if !cfg!(debug_assertions) {
        return Err(Status::unavailable(t!("该接口只能在debug模式下使用")));
    }

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
    request: Request<DeleteEntitiesRequest>,
) -> Result<Request<DeleteEntitiesRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;
    let entity_ids = &request.get_ref().entity_ids;

    // 管理编号不能为0
    if manage_id.is_empty() {
        return Err(Status::invalid_argument(format!(
            "{}-{}",
            t!("管理编号不能为空"),
            "get_entities"
        )));
    }

    // 实体编号不能为空
    if entity_ids.is_empty() {
        return Err(Status::invalid_argument(format!(
            "{}-{}",
            t!("实体编号不能为空"),
            "get_entities"
        )));
    }

    // 实体列表不能超过100
    if entity_ids.len() > 100 {
        return Err(Status::invalid_argument(format!(
            "{}-{}",
            t!("一次最多删除100个实体"),
            "get_entities"
        )));
    }

    Ok(request)
}

async fn handle_delete_entities(
    request: Request<DeleteEntitiesRequest>,
) -> StreamResponseResult<DeleteEntitiesResponse> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;
    let entity_ids = &request.get_ref().entity_ids;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id.as_str()).unwrap();

    let manage_id = manage_id.to_owned();

    let (resp_tx, resp_rx) = tokio::sync::mpsc::channel(1);
    let mut id_stream = stream::iter(entity_ids.clone());
    tokio::spawn(async move {
        while let Some(ref id) = id_stream.next().await {
            if !can_entity_write(&manage_id.clone(), id, &account_id, &role_group).await {
                error!(
                    "{}, {}: {}-{}, {}",
                    t!("删除实体失败"),
                    t!("没有权限"),
                    manage_id,
                    id,
                    role_group,
                );

                continue;
            }

            let query_doc = doc! {
                ID_FIELD_ID.to_string(): id
            };

            let entity = manager.get_entity_by_id(id, &[], &[]).await;

            if let Err(e) = manager.delete_entity(id.as_str()).await {
                error!(
                    "{}: {}-{}, {}",
                    t!("删除实体失败"),
                    manage_id,
                    id,
                    e.details()
                );

                continue;
            }

            if let Ok(e) = entity {
                let resp = DeleteEntitiesResponse {
                    entity: bson::to_vec(&e).unwrap(),
                };
                send_stream_response(&resp_tx, resp).await;
            }
        }
    });

    let resp_stream = ReceiverStream::new(resp_rx);

    Ok(Response::new(
        Box::pin(resp_stream) as ResponseStream<DeleteEntitiesResponse>
    ))
}
