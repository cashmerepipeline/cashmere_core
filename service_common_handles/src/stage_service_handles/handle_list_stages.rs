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
pub trait HandleListStages {
    async fn handle_list_stages(
        &self,
        request: Request<ListStagesRequest>,
    ) -> UnaryResponseResult<ListStagesResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let specs_id = &request.get_ref().specs_id;

        

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc
            .get_manager_by_id(STAGES_MANAGE_ID)
            .unwrap();

        let mut filter_doc = Document::new();
        filter_doc.insert(STAGES_SPECS_ID_FIELD_ID.to_string(), specs_id);

        let result = manager.get_entities_by_filter(&Some(filter_doc)).await;

        match result {
            Ok(entities) => Ok(Response::new(ListStagesResponse {
                stages: entities.iter().map(|x| bson::to_vec(x).unwrap()).collect(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
