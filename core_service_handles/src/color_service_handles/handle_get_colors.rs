use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;


use manage_define::cashmere::*;
use manage_define::manage_ids::COLORS_MANAGE_ID;

use dependencies_sync::tonic::{Request, Response, Status};

use service_utils::get_constants;
use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetColors {
    /// 取得管理记录数量
    async fn handle_get_country_codes(
        &self,
        request: Request<GetColorsRequest>,
    ) -> UnaryResponseResult<GetColorsResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_country_codes)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetColorsRequest>,
) -> Result<Request<GetColorsRequest>, Status> {
    // 全部可读
    Ok(request)
}

async fn validate_request_params(
    request: Request<GetColorsRequest>,
) -> Result<Request<GetColorsRequest>, Status> {
    // 没有参数
    Ok(request)
}

async fn handle_get_country_codes(
    _request: Request<GetColorsRequest>,
) -> Result<Response<GetColorsResponse>, Status> {
    let result = get_constants(COLORS_MANAGE_ID).await;

    match result {
        Ok(entities) => Ok(Response::new(GetColorsResponse {
            colors: entities.iter().map(|e| bson::to_vec(e).unwrap()).collect(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
