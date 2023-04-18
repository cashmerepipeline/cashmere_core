use async_trait::async_trait;
use bson::doc;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use tonic::{Request, Response, Status};
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleAddStageVersion {
    async fn handle_add_stage_version(
        &self,
        request: Request<AddStageVersionRequest>,
    ) -> UnaryResponseResult<AddStageVersionResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let stage_id = &request.get_ref().stage_id;
        let version = &request.get_ref().version;

        if !view::can_manage_write(&account_id, &role_group, &STAGES_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        if !view::can_field_write(&account_id, &role_group, &STAGES_MANAGE_ID.to_string(), &STAGES_VERSIONS_FIELD_ID.to_string()).await
        {
            return Err(Status::permission_denied("用户不具有属性可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(STAGES_MANAGE_ID)
            .await
            .unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():stage_id,
        };

        let field_key = format!("{}.versions", STAGES_VERSIONS_FIELD_ID);
        let mut modify_doc = bson::Document::new();
        modify_doc.insert(field_key, version);

        let result = manager
            .push_entity_array_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(AddStageVersionResponse {
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

