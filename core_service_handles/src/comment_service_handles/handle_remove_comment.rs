use dependencies_sync::{
    bson::{doc},
    tokio,
    tonic::{Request, Response, Status},
    tonic::async_trait,
    futures::TryFutureExt,
};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::manager_trait::ManagerTrait;

use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleRemoveComment {
    async fn handle_remove_comment(
        &self,
        request: Request<RemoveCommentRequest>,
    ) -> UnaryResponseResult<RemoveCommentResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_remove_comment)
            .await
    }
}


async fn validate_view_rules(
    request: Request<RemoveCommentRequest>,
) -> Result<Request<RemoveCommentRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = COMMENTS_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<RemoveCommentRequest>,
) -> Result<Request<RemoveCommentRequest>, Status> {
    Ok(request)
}

async fn handle_remove_comment(
    request: Request<RemoveCommentRequest>,
) -> Result<Response<RemoveCommentResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let target_manage_id = &request.get_ref().target_manage_id;
    let target_entity_id = &request.get_ref().target_entity_id;
    let comment_id = &request.get_ref().comment_id;

    let majordomo_arc = get_majordomo();
    let target_manager = majordomo_arc
        .get_manager_by_id(target_manage_id.as_str())
        .unwrap();
    let comment_manager = majordomo_arc
        .get_manager_by_id(COMMENTS_MANAGE_ID)
        .unwrap();

    // TODO: 注释数据测存储需要改变
    let query_doc = doc! {
        ID_FIELD_ID.to_string():target_entity_id,
    };
    let modify_doc = doc! {
        COMMENTS_FIELD_ID.to_string():comment_id
    };

    let result = tokio::try_join!(
        target_manager.remove_from_array_field(query_doc, modify_doc, &account_id),
        // 不处理子注释
        comment_manager.mark_entity_removed(comment_id, &account_id)
    );

    match result {
        Ok(_r) => Ok(Response::new(RemoveCommentResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
