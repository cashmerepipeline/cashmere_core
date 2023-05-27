use dependencies_sync::tonic::async_trait;

use dependencies_sync::bson::{self, Document};
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;

use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use view;

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleListStageVersions {
    async fn handle_list_stage_versions(
        &self,
        request: Request<ListStageVersionsRequest>,
    ) -> UnaryResponseResult<ListStageVersionsResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let stage_id = &request.get_ref().stage_id;

        

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc
            .get_manager_by_id(STAGES_MANAGE_ID)
            .unwrap();

        let mut filter_doc = Document::new();
        filter_doc.insert(STAGES_SPECS_ID_FIELD_ID.to_string(), stage_id);

        let result = manager.get_entity_by_id(stage_id).await;

        match result {
            Ok(entity) => {
                let versions: Vec<Version> = entity
                    .get_array(STAGES_VERSIONS_FIELD_ID.to_string())
                    .unwrap()
                    .iter()
                    .map(|x| bson::from_bson(x.to_owned()).unwrap())
                    .collect();
                Ok(Response::new(ListStageVersionsResponse {
                    versions,
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
