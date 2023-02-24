use async_trait::async_trait;
use bson::doc;
use tokio_stream::{self as stream, StreamExt};
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use view::{self, can_field_read};

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleRecoverRemovedEntity {
    /// 取得管理记录数量
    async fn handle_recover_removed_entity(
        &self,
        request: Request<RecoverRemovedEntityRequest>,
    ) -> UnaryResponseResult<RecoverRemovedEntityResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;

        // 集合可写检查
        if !view::can_collection_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有管理可读权限"));
        }
        // 实体可写检查
        if !view::can_collection_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有管理可读权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(*manage_id).await.unwrap();

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

