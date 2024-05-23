use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;

use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use managers::{entity_interface::EntityInterface};

use dependencies_sync::tokio_stream::StreamExt;
use dependencies_sync::tonic::{Request, Response, Status};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetConstants {
    /// 取得管理记录数量
    async fn handle_get_constants(
        &self,
        request: Request<GetConstantsRequest>,
    ) -> UnaryResponseResult<GetConstantsResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_constants)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetConstantsRequest>,
) -> Result<Request<GetConstantsRequest>, Status> {
    // 全部可读
    Ok(request)
}

async fn validate_request_params(
    request: Request<GetConstantsRequest>,
) -> Result<Request<GetConstantsRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;

    if manage_id.is_empty(){
        return Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("无效参数"),
            "get_constants"
        )));
    }
    
    // TODO: 检查是否为常量管理

    Ok(request)
}

async fn handle_get_constants(
    request: Request<GetConstantsRequest>,
) -> Result<Response<GetConstantsResponse>, Status> {
    let manage_id = &request.get_ref().manage_id;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id.as_str()).unwrap();

    let query_doc = doc! {};

    let result = manager.get_entity_stream(query_doc, &[], None, None, 0).await;

    match result {
        Ok(mut entities_iter) => {
            let mut results = vec![];
            while let Some(r) = entities_iter.next().await {
                results.push(bson::to_vec(&r).unwrap());
            }

            Ok(Response::new(GetConstantsResponse { constants: results }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
