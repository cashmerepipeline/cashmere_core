
use async_trait::async_trait;
use bson::{doc};



use tonic::{Request, Response, Status};

use manage_define::cashmere::*;
use crate::UnaryResponseResult;

use majordomo::{self, get_majordomo};

use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;

use view;

#[async_trait]
pub trait HandleMarkDataRemoved {
    async fn handle_mark_data_remved(
        &self,
        request: Request<MarkDataRemovedRequest>,
    ) -> UnaryResponseResult<MarkDataRemovedResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;
        let data_id = &request.get_ref().data_id;

        if !view::can_manage_write(&account_id, &role_group, &DATAS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let data_manager = majordomo_arc
            .get_manager_by_id(DATAS_MANAGE_ID)
            .await
            .unwrap();
        let associated_manager = majordomo_arc
            .get_manager_by_id(*manage_id)
            .await
            .unwrap();

        let result = tokio::try_join!(
            data_manager.mark_entity_removed(data_id, &account_id),
            associated_manager.push_entity_array_field(
                doc! {"_id": entity_id.clone()},
                doc! {DATAS_REMOVED_FIELD_ID.to_string(): data_id.clone()},
                &account_id
            ),
            associated_manager.pull_entity_array_field(
                doc! {"_id": entity_id.clone()},
                doc! {DATAS_FIELD_ID.to_string(): data_id.clone()},
                &account_id
            )
        );

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