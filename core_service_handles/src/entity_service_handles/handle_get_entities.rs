use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio;
use dependencies_sync::tokio_stream::wrappers::ReceiverStream;
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use managers::{manager_trait::ManagerInterface, entity_interface::EntityInterface};
use request_utils::request_account_context;

use dependencies_sync::tokio_stream::{self as stream, StreamExt};
use dependencies_sync::tonic::{Request, Response, Status};
use service_utils::send_stream_response;
use view::{self, can_entity_read, get_manage_schema_view_mask};

use service_utils::types::{ResponseStream, StreamResponseResult};

#[async_trait]
pub trait HandleGetEntities {
    /// 取得管理记录数量
    async fn handle_get_entities(
        &self,
        request: Request<GetEntitiesRequest>,
    ) -> StreamResponseResult<GetEntitiesResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_entities)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetEntitiesRequest>,
) -> Result<Request<GetEntitiesRequest>, Status> {
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
    request: Request<GetEntitiesRequest>,
) -> Result<Request<GetEntitiesRequest>, Status> {
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
            t!("一次最多取得100个实体"),
            "get_entities"
        )));
    }

    Ok(request)
}

async fn handle_get_entities(
    request: Request<GetEntitiesRequest>,
) -> StreamResponseResult<GetEntitiesResponse> {
    let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;
    let entity_ids = &request.get_ref().entity_ids;
    let _present_fields = &request.get_ref().present_fields;
    let no_present_fields = &request.get_ref().no_present_fields;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id.as_str()).unwrap();

    let fields = manager.get_manage_schema().await;
    let view_mask = get_manage_schema_view_mask(manage_id, &fields, &role_group).await;
    
    let manage_id = manage_id.to_owned();
    let mut filtered_ids = no_present_fields.clone();
    view_mask.iter().for_each(|(k, v)| {
        if !v {
            filtered_ids.push(k.to_owned())
        }
    });

    let (resp_tx, resp_rx) = tokio::sync::mpsc::channel(1);
    let mut id_stream = stream::iter(entity_ids.clone());
    tokio::spawn(async move {
        while let Some(ref id) = id_stream.next().await {
            if can_entity_read(&manage_id.clone(), &role_group).await {
                filtered_ids.push(id.to_owned());
            };

            let entity = manager.get_entity_by_id(id, &[], &filtered_ids).await;
            if let Ok(e) = entity {
                let resp = GetEntitiesResponse {
                    entity: bson::to_vec(&e).unwrap(),
                };
                send_stream_response(&resp_tx, resp).await;
            }
        }
    });

    let resp_stream = ReceiverStream::new(resp_rx);

    Ok(Response::new(
        Box::pin(resp_stream) as ResponseStream<GetEntitiesResponse>
    ))
}
