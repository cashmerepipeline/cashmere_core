use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{doc};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleRecoverRemovedEntity {
    /// 取得管理记录数量
    async fn handle_recover_removed_entity(
        &self,
        request: Request<RecoverRemovedEntityRequest>,
    ) -> UnaryResponseResult<RecoverRemovedEntityResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;

        // 集合可写检查
        
        // 实体可写检查
        

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

        let result = manager.recover_removed_entity(entity_id, &account_id).await;

        match result {
            Ok(_r) => Ok(Response::new(RecoverRemovedEntityResponse {
                result: "ok".to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}


async fn validate_view_rules(
    request: Request<RecoverRemovedEntityRequest>,
) -> Result<Request<RecoverRemovedEntityRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if let Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<RecoverRemovedEntityRequest>,
) -> Result<Request<RecoverRemovedEntityRequest>, Status> {
    Ok(request)
}