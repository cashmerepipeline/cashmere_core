use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleMarkEntityRemoved {
    async fn handle_mark_entity_remved(
        &self,
        request: Request<MarkEntityRemovedRequest>,
    ) -> UnaryResponseResult<MarkEntityRemovedResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;

        if !view::can_collection_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有集合可写权限"));
        }
        if !view::can_entity_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有实体可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(*manage_id)
            .await
            .unwrap();

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
}
