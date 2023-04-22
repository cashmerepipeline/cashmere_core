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
pub trait HandleAddStageCurrentVersion {
    async fn handle_set_stage_current_version(
        &self,
        request: Request<SetStageCurrentVersionRequest>,
    ) -> UnaryResponseResult<SetStageCurrentVersionResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let stage_id = &request.get_ref().stage_id;
        let target_version = &request.get_ref().target_version;

        if !view::can_manage_write(&account_id, &role_group, &STAGES_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        if !view::can_field_write(
            &account_id,
            &role_group,
            &STAGES_MANAGE_ID.to_string(),
            &STAGES_VERSIONS_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::permission_denied("用户不具有属性可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(STAGES_MANAGE_ID)
            .await
            .unwrap();

        let stage_entity = match manager.get_entity_by_id(stage_id).await {
            Ok(e) => e,
            Err(e) => {
                // 不存在
                return Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                )))
            }
        };

        if stage_entity
            .get_array(STAGES_VERSIONS_FIELD_ID.to_string())
            .unwrap()
            .iter()
            .map(|v| bson::from_bson::<Version>(v.clone()).unwrap())
            .find(|v| v.name == *target_version)
            .is_none()
        {
            return Err(Status::invalid_argument("版本不存在"));
        };

        let query_doc = doc! {
            ID_FIELD_ID.to_string():stage_id,
        };

        let mut modify_doc = bson::Document::new();
        modify_doc.insert(STAGES_CURRENT_VERSION_FIELD_ID.to_string(), target_version);

        let result = manager
            .update_entity_field(query_doc, &mut modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(SetStageCurrentVersionResponse {
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
