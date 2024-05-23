use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use managers::{entity_interface::EntityInterface};
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};


use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleMarkEntityRemoved {
    async fn handle_mark_entity_remved(
        &self,
        request: Request<MarkEntityRemovedRequest>,
    ) -> UnaryResponseResult<MarkEntityRemovedResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_mark_entity_remved)
            .await
    }
}

async fn validate_view_rules(
    request: Request<MarkEntityRemovedRequest>,
) -> Result<Request<MarkEntityRemovedRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<MarkEntityRemovedRequest>,
) -> Result<Request<MarkEntityRemovedRequest>, Status> {
    Ok(request)
}

async fn handle_mark_entity_remved(
    request: Request<MarkEntityRemovedRequest>,
) -> Result<Response<MarkEntityRemovedResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id.as_str()).unwrap();

    let result = manager.mark_entity_removed(entity_id, &account_id).await;

    match result {
        Ok(_r) => Ok(Response::new(MarkEntityRemovedResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
