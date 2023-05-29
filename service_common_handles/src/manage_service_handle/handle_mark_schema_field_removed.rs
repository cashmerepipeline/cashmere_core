use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{doc};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use view;

#[async_trait]
pub trait HandleMarkSchemaFieldRemoved {
    /// 移除管理属性
    async fn handle_mark_schema_field_removed(
        &self,
        request: Request<MarkSchemaFieldRemovedRequest>,
    ) -> Result<Response<MarkSchemaFieldRemovedResponse>, Status> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let manage_id: i32 = request.get_ref().manage_id;
        let field_id = request.get_ref().field_id;

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();
        let result = manager
            .mark_schema_field_removed(field_id, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(MarkSchemaFieldRemovedResponse {
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
    request: Request<MarkSchemaFieldRemovedRequest>,
) -> Result<Request<MarkSchemaFieldRemovedRequest>, Status> {
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
    request: Request<MarkSchemaFieldRemovedRequest>,
) -> Result<Request<MarkSchemaFieldRemovedRequest>, Status> {
    Ok(request)
}