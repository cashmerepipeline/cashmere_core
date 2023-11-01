use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;



use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;


use manage_define::manage_ids::PHONE_AREA_CODES_MANAGE_ID;
use managers::manager_trait::ManagerTrait;


use dependencies_sync::tokio_stream::{StreamExt};
use dependencies_sync::tonic::{Request, Response, Status};


use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetPhoneAreaCodes {
    /// 取得管理记录数量
    async fn handle_get_phone_area_codes(
        &self,
        request: Request<GetPhoneAreaCodesRequest>,
    ) -> UnaryResponseResult<GetPhoneAreaCodesResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_phone_area_codes)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetPhoneAreaCodesRequest>,
) -> Result<Request<GetPhoneAreaCodesRequest>, Status> {
    // 全部可读
    Ok(request)
}

async fn validate_request_params(
    request: Request<GetPhoneAreaCodesRequest>,
) -> Result<Request<GetPhoneAreaCodesRequest>, Status> {
    // 没有参数
    Ok(request)
}

async fn handle_get_phone_area_codes(
    _request: Request<GetPhoneAreaCodesRequest>,
) -> Result<Response<GetPhoneAreaCodesResponse>, Status> {
    let manage_id = PHONE_AREA_CODES_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    let query_doc = doc! {};

    let result = manager.get_entity_stream(query_doc, None, None).await;

    match result {
        Ok(mut entities_iter) => {
            let mut result_codes = vec![];
            while let Some(r) = entities_iter.next().await {
                let code = bson::to_vec(&r).unwrap();
                
                result_codes.push(code);
            }

            Ok(Response::new(GetPhoneAreaCodesResponse {
                codes: result_codes
            }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
