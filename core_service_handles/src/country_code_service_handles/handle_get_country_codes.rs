use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;

use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::manage_ids::COUNTRY_CODES_MANAGE_ID;
use managers::manager_trait::ManagerTrait;

use dependencies_sync::tokio_stream::StreamExt;
use dependencies_sync::tonic::{Request, Response, Status};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetCountryCodes {
    /// 取得管理记录数量
    async fn handle_get_country_codes(
        &self,
        request: Request<GetCountryCodesRequest>,
    ) -> UnaryResponseResult<GetCountryCodesResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_country_codes)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetCountryCodesRequest>,
) -> Result<Request<GetCountryCodesRequest>, Status> {
    // 全部可读
    Ok(request)
}

async fn validate_request_params(
    request: Request<GetCountryCodesRequest>,
) -> Result<Request<GetCountryCodesRequest>, Status> {
    // 没有参数
    Ok(request)
}

async fn handle_get_country_codes(
    _request: Request<GetCountryCodesRequest>,
) -> Result<Response<GetCountryCodesResponse>, Status> {
    let manage_id = COUNTRY_CODES_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    let query_doc = doc! {};

    let result = manager.get_entity_stream(query_doc, &vec![], None, None, 0).await;

    match result {
        Ok(mut entities_iter) => {
            let mut result_codes = vec![];
            while let Some(r) = entities_iter.next().await {
                let code = bson::to_vec(&r).unwrap();
                result_codes.push(code);
            }

            Ok(Response::new(GetCountryCodesResponse {
                codes: result_codes,
            }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
