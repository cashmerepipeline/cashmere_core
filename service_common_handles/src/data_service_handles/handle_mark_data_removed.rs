use dependencies_sync::tonic::async_trait;


use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;


use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleMarkDataRemoved {
    async fn handle_mark_data_remved(
        &self,
        request: Request<MarkDataRemovedRequest>,
    ) -> UnaryResponseResult<MarkDataRemovedResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let manage_id = &request.get_ref().owner_manage_id;
        let entity_id = &request.get_ref().owner_entity_id;

        if !view::can_entity_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let data_manager = majordomo_arc.get_manager_by_id(*manage_id).await.unwrap();

        let result = data_manager
            .mark_entity_removed(entity_id, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(MarkDataRemovedResponse {
                result: "success".to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}

