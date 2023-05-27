use dependencies_sync::{
    bson::{doc},
    tokio,
    tonic::{Request, Response, Status},
    tonic::async_trait,
};
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleRemoveComment {
    async fn handle_remove_comment(
        &self,
        request: Request<RemoveCommentRequest>,
    ) -> UnaryResponseResult<RemoveCommentResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let target_manage_id = &request.get_ref().target_manage_id;
        let target_entity_id = &request.get_ref().target_entity_id;
        let comment_id = &request.get_ref().comment_id;

        let majordomo_arc = get_majordomo();
        let target_manager = majordomo_arc
            .get_manager_by_id(target_manage_id.parse::<i32>().unwrap())
            .unwrap();
        let comment_manager = majordomo_arc
            .get_manager_by_id(COMMENTS_MANAGE_ID)
            .unwrap();

        let query_doc = doc! {
            "_id":target_entity_id,
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
}
