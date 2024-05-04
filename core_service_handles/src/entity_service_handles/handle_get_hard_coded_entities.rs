use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;

use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use managers::manager_trait::ManagerTrait;

use dependencies_sync::tokio_stream::StreamExt;
use dependencies_sync::tonic::{Request, Response, Status};

use service_utils::types::UnaryResponseResult;
use validates::validate_hard_coded;

#[async_trait]
pub trait HandleGetHardCodedEntities {
    /// 取得管理记录数量
    async fn handle_get_hard_coded_entities(
        &self,
        request: Request<GetHardCodedEntitiesRequest>,
    ) -> UnaryResponseResult<GetHardCodedEntitiesResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_country_codes)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetHardCodedEntitiesRequest>,
) -> Result<Request<GetHardCodedEntitiesRequest>, Status> {
    // 全部可读
    Ok(request)
}

async fn validate_request_params(
    request: Request<GetHardCodedEntitiesRequest>,
) -> Result<Request<GetHardCodedEntitiesRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;

    validate_hard_coded(manage_id).await?;

    Ok(request)
}

async fn handle_get_country_codes(
    request: Request<GetHardCodedEntitiesRequest>,
) -> Result<Response<GetHardCodedEntitiesResponse>, Status> {
    let manage_id = &request.get_ref().manage_id;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    let query_doc = doc! {};

    let result = manager
        .get_entity_stream(query_doc, &[], None, None, 0)
        .await;

    match result {
        Ok(mut entities_iter) => {
            let mut result_codes = vec![];
            while let Some(r) = entities_iter.next().await {
                let code = bson::to_vec(&r).unwrap();
                result_codes.push(code);
            }

            Ok(Response::new(GetHardCodedEntitiesResponse {
                entities: result_codes,
            }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
