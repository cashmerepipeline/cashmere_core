use async_trait::async_trait;
use bson::Document;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleListStageVersions {
    async fn handle_list_stage_versions(
        &self,
        request: Request<ListStageVersionsRequest>,
    ) -> UnaryResponseResult<ListStageVersionsResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let stage_id = &request.get_ref().stage_id;

        if !view::can_entity_read(&account_id, &role_group, &STAGES_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可读权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(STAGES_MANAGE_ID)
            .await
            .unwrap();

        let mut filter_doc = Document::new();
        filter_doc.insert(STAGES_SPECS_ID_FIELD_ID.to_string(), stage_id);

        let result = manager.get_entity_by_id(&stage_id).await;

        match result {
            Ok(entity) => {
                let versions: Vec<Version> = entity
                    .get_array(STAGES_VERSIONS_FIELD_ID.to_string())
                    .unwrap()
                    .iter()
                    .map(|x|  bson::from_bson(x.to_owned()).unwrap())
                    .collect();
                Ok(Response::new(ListStageVersionsResponse {
                    versions: versions,
                }))
            }
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
