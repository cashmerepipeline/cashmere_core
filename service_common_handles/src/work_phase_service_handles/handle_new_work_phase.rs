use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use manage_define::field_ids::*;
use managers::traits::ManagerTrait;
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleNewWorkPhase {
    async fn handle_new_work_phase(
        &self,
        request: Request<NewWorkPhaseRequest>,
    ) -> UnaryResponseResult<NewWorkPhaseResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let name = &request.get_ref().name;

        if !view::can_collection_write(&account_id, &role_group, &WORK_PHASES_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let group_id =
            match view::get_first_write_group(&groups, &WORK_PHASES_MANAGE_ID.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let mut new_entity_doc = Document::new();

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(WORK_PHASES_MANAGE_ID)
            .await
            .unwrap();
        let new_id = manager.get_new_entity_id().await.unwrap();

        new_entity_doc.insert("_id", new_id.to_string());
        new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
        new_entity_doc.insert(NAME_FIELD_ID.to_string(), name);

        let result = manager
            .new_entity(&mut new_entity_doc, &account_id, &group_id)
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewWorkPhaseResponse {
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
