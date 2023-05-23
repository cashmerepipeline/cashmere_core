use dependencies_sync::{
    bson::{doc},
    tonic::{Request, Response, Status},
    tonic::async_trait,
};
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;

use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleEditComment {
    async fn handle_edit_comment(
        &self,
        request: Request<EditCommentRequest>,
    ) -> UnaryResponseResult<EditCommentResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let comment_id = &request.get_ref().comment_id;
        let new_contents = &request.get_ref().new_contents;

        if !view::can_manage_write(&account_id, &role_group, &COMMENTS_MANAGE_ID.to_string()).await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(COMMENTS_MANAGE_ID)
            .await
            .unwrap();

        let query_doc = doc! {
            "_id":comment_id,
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
}
