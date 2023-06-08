use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{doc};
use dependencies_sync::futures::TryFutureExt;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};


use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetManageEntryCount {
    /// 取得管理记录数量
    async fn handle_get_manage_entry_count(
        &self,
        request: Request<GetManageEntryCountRequest>,
    ) -> UnaryResponseResult<GetManageEntryCountResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_manage_entry_count)
            .await
    }
}


async fn validate_view_rules(
    request: Request<GetManageEntryCountRequest>,
) -> Result<Request<GetManageEntryCountRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if let Err(e) = view::validates::validate_collection_can_read(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<GetManageEntryCountRequest>,
) -> Result<Request<GetManageEntryCountRequest>, Status> {
    Ok(request)
}

async fn handle_get_manage_entry_count(
    request: Request<GetManageEntryCountRequest>,
) -> Result<Response<GetManageEntryCountResponse>, Status> {
    let (_account_id, _groups, _role_group) = request_account_context(request.metadata());

    let manage_id = &request.get_ref().manage_id;

    

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

    let result = manager.get_entry_counts(doc! {}).await;

    match result {
        Ok(r) => Ok(Response::new(GetManageEntryCountResponse { count: r })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}