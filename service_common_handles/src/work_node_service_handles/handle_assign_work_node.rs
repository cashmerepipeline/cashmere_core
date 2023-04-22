use async_trait::async_trait;
use bson::doc;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use tonic::{Request, Response, Status};
use view;

#[async_trait]
pub trait HandleAssignWorkNode {
    async fn handle_assign_work_node(
        &self,
        request: Request<AssignWorkNodeRequest>,
    ) -> Result<Response<AssignWorkNodeResponse>, Status> {
        let (account_id, _groups, role_group) = request_account_context(&request.metadata());

        let work_node_id = &request.get_ref().work_node_id;
        let worker_id = &request.get_ref().worker_id;

        if !view::can_entity_write(&account_id, &role_group, &worker_id).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let node_manager = majordomo_arc
            .get_manager_by_id(WORK_NODES_MANAGE_ID)
            .await
            .unwrap();

        let query_doc = doc! {
            "_id":work_node_id
        };
        let mut modify_doc = doc! {
             WORK_NODES_WORKER_FIELD_ID.to_string():worker_id
        };

        let result = node_manager
            .update_entity_field(query_doc, &mut modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(AssignWorkNodeResponse {
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
