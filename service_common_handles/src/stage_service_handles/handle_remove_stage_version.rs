use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{self, doc};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;

use dependencies_sync::tonic::{Request, Response, Status};
use view;
use request_utils::request_account_context;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleRemoveStageVersion {
    async fn handle_remove_stage_version(
        &self,
        request: Request<RemoveStageVersionRequest>,
    ) -> UnaryResponseResult<RemoveStageVersionResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let stage_id = &request.get_ref().stage_id;
        let version = &request.get_ref().version;

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

        let query_doc = doc! {
            ID_FIELD_ID.to_string():stage_id,
            format!("{}.name", STAGES_VERSIONS_FIELD_ID):version,
        };

        let field_key = format!("{}.$.removed", STAGES_VERSIONS_FIELD_ID);
        let mut modify_doc = bson::Document::new();
        modify_doc.insert(field_key, true);

        let result = manager
            .update_array_element_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(RemoveStageVersionResponse {
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
