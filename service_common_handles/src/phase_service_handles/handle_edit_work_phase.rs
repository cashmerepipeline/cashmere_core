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

#[async_trait]
pub trait HandleEditWorkPhase {
    async fn handle_edit_work_phase(
        &self,
        request: Request<EditWorkPhaseRequest>,
    ) -> ResponseResult<EditWorkPhaseResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let phase_id = &request.get_ref().phase_id;
        let new_phase = &request.get_ref().new_phase;

        if !view::can_collection_write(&account_id, &groups, &WORK_PHASES_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(WORK_PHASES_MANAGE_ID)
            .await
            .unwrap();

        let mut new_value = new_phase.clone();
        let new_value_docs = bson::Document::from_reader(&mut new_value.as_slice()).unwrap();
        let new_value = bson::to_bson(&new_value_docs).unwrap();

        let query_doc = doc! {
            "_id":phase_id
        };
        let modify_doc = doc! {
             WORK_PHASE_PHASES_FIELD_ID.to_string():worker_id
        };

        let result = manager
            .update_entity_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(r) => Ok(Response::new(EditWorkPhaseResponse {
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
