use dependencies_sync::{
    bson::Document,
    futures::TryFutureExt,
    tonic::async_trait,
    tonic::{Request, Response, Status},
};
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::manager_trait::ManagerTrait;

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleNewComment {
    async fn handle_new_comment(
        &self,
        request: Request<NewCommentRequest>,
    ) -> UnaryResponseResult<NewCommentResponse> {
        validate_view_rules(request)
            .and_then(handle_new_comment)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewCommentRequest>,
) -> Result<Request<NewCommentRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        use request_utils::request_account_context;
        
        let manage_id = COMMENTS_MANAGE_ID;
        let (account_id, groups, role_group) = request_account_context(request.metadata());
        if let Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn handle_new_comment(
    request: Request<NewCommentRequest>,
) -> Result<Response<NewCommentResponse>, Status> {
    let metadata = request.metadata();
    // 已检查过，不需要再检查正确性
    let token = auth::get_auth_token(metadata).unwrap();
    let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
    let role_group = auth::get_current_role(metadata).unwrap();

    let target_manage_id = &request.get_ref().target_manage_id;
    let target_entity_id = &request.get_ref().target_entity_id;
    let contents = &request.get_ref().contents;

    let mut new_entity_doc = Document::new();

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(COMMENTS_MANAGE_ID).unwrap();
    let new_id = manager.get_new_entity_id(&account_id).await.unwrap();

    new_entity_doc.insert("_id", new_id.to_string());
    new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
    new_entity_doc.insert(
        COMMENTS_TARGET_MANAGE_FIELD_ID.to_string(),
        target_manage_id,
    );
    new_entity_doc.insert(
        COMMENTS_TARGET_ENTITY_FIELD_ID.to_string(),
        target_entity_id,
    );
    new_entity_doc.insert(COMMENTS_CONTENTS_FIELD_ID.to_string(), contents);

    let result = manager
        .sink_entity(&mut new_entity_doc, &account_id, &role_group)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(NewCommentResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
