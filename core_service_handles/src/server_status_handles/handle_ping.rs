use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::{
    PHONE_AREA_CODES_CODE_FIELD_ID, PHONE_AREA_CODES_USING_AREAS_FIELD_ID,
};
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::{request_account_context, validate_auth_token, validate_has_role_group};

use dependencies_sync::tonic::{Request, Response, Status};

#[async_trait]
pub trait HandlePing {
    /// 新建管理属性
    async fn handle_ping(
        &self,
        request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        validate_auth_token::<PingRequest>(request)
            .and_then(validate_view_rules)
            .and_then(validate_request_params)
            .and_then(handle_ping)
            .await
    }
}

async fn validate_view_rules(
    request: Request<PingRequest>,
) -> Result<Request<PingRequest>, Status> {
    // 不需要权限
    Ok(request)
}

async fn validate_request_params(
    request: Request<PingRequest>,
) -> Result<Request<PingRequest>, Status> {
    Ok(request)
}

async fn handle_ping(request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata());

    let time = &request.get_ref().time;
    Ok(Response::new(PingResponse { time: *time }))
}
