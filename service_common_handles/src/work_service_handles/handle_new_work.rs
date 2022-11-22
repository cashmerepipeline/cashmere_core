use async_trait::async_trait;
use bson::{doc, Document};
use manage_define::field_ids::WORKS_MANAGE_FIELD_ID;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleNewWork {
    async fn handle_new_work(
        &self,
        request: Request<NewWorkRequest>,
    ) -> UnaryResponseResult<NewWorkResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let work_name = &request.get_ref().name;

        if !view::can_collection_write(&account_id, &role_group, manage_id).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(WORKS_MANAGE_ID)
            .await
            .unwrap();

        if let Some(mut new_entity_doc) = make_new_entity_document(&manager).await {
            new_entity_doc.insert(NAME_MAP_FIELD_ID.to_string(), work_name);
            new_entity_doc.insert(WORKS_MANAGE_FIELD_ID.to_string(), manage_id);

            let result = manager
                .sink_entity(&mut new_entity_doc, &account_id, &role_group)
                .await;

            match result {
                Ok(r) => Ok(Response::new(NewWorkResponse {
                    result: "ok".to_string(),
                })),
                Err(e) => Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                ))),
            }
        } else {
            Err(Status::aborted("创建新工作失败"))
        }
    }
}
