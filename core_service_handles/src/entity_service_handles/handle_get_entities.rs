use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::ID_FIELD_ID;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tokio_stream::{self as stream, iter, StreamExt};
use dependencies_sync::tonic::{Request, Response, Status};
use view::{self, can_entity_read, can_field_read, filter_can_read_fields};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetEntities {
    /// 取得管理记录数量
    async fn handle_get_entities(
        &self,
        request: Request<GetEntitiesRequest>,
    ) -> UnaryResponseResult<GetEntitiesResponse> {
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
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
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
    if manage_id == &0 {
        return Err(Status::invalid_argument(format!(
            "{}-{}",
            t!("管理编号不能为0"),
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
            t!("实体列表不能超过100"),
            "get_entities"
        )));
    }

    Ok(request)
}

async fn handle_get_entities(
    request: Request<GetEntitiesRequest>,
) -> Result<Response<GetEntitiesResponse>, Status> {
    let (_account_id, _groups, role_group) = request_account_context(request.metadata());

    let manage_id = &request.get_ref().manage_id;
    let entity_ids = &request.get_ref().entity_ids;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

    // 实体可见性过滤
    let mut filtered_ids = vec![];
    let mut id_stream = stream::iter(entity_ids);
    while let Some(ref id) = id_stream.next().await {
        if can_entity_read(&manage_id.to_string(), &role_group).await {
            filtered_ids.push(id.to_owned());
        }
    }

    let query_doc = doc! {
        ID_FIELD_ID.to_string():{"$in":filtered_ids.clone()}
    };

    let result = manager.get_entities_by_filter(&Some(query_doc)).await;

    match result {
        Ok(entities) => {
            let mut result_docs = vec![];
            while let Some(doc) = iter(&entities).next().await {
                let entity = filter_can_read_fields(&doc, manage_id, &role_group).await;
                result_docs.push(entity);
            }

            Ok(Response::new(GetEntitiesResponse {
                entities: result_docs
                    .iter()
                    .map(|x| bson::to_vec(x).unwrap())
                    .collect(),
            }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
