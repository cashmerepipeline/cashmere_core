use dependencies_sync::{
    bson::{doc},
    tonic::{Request, Response, Status},
    tonic::async_trait,
    futures::TryFutureExt,
};
use majordomo::{self, get_majordomo};
use manage_define::{cashmere::*, general_field_ids::ID_FIELD_ID};
use manage_define::field_ids::*;

use manage_define::manage_ids::*;
use managers::manager_trait::ManagerTrait;

use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleEditComment {
    async fn handle_edit_comment(
        &self,
        request: Request<EditCommentRequest>,
    ) -> UnaryResponseResult<EditCommentResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_edit_comment)
            .await
    }
}



async fn validate_view_rules(
    request: Request<EditCommentRequest>,
) -> Result<Request<EditCommentRequest>, Status> {
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
    request: Request<EditCommentRequest>,
) -> Result<Request<EditCommentRequest>, Status> {
    Ok(request)
}

async fn handle_edit_comment(
    request: Request<EditCommentRequest>,
) -> Result<Response<EditCommentResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let comment_id = &request.get_ref().comment_id;
    let new_contents = &request.get_ref().new_contents;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(COMMENTS_MANAGE_ID)
        .unwrap();

    let query_doc = doc! {
        ID_FIELD_ID.to_string():comment_id,
    };
    let mut modify_doc = doc! {
        COMMENTS_CONTENTS_FIELD_ID.to_string():new_contents
    };

    let result = manager
        .update_entity_field(query_doc, &mut modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(EditCommentResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
