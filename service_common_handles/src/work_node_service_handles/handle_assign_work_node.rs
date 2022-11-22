use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use view;

#[async_trait]
pub trait HandleAssignWorkNode{
    async fn handle_assign_work_node(
        &self,
        request: Request<AssignWorkNodeRequest>,
    ) -> Result<Response<AssignWorkNodeResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let work_node_id = &request.get_ref().work_node_id;
        let worker_id = &request.get_ref().worker_id;

        if !view::can_entity_write(
            &account_id,
            &groups,
            &WORK_NODES_MANAGE_ID.to_string(),
            &WORK_NODE_WORKER_FIELD_ID.to_string(),
        )
            .await
        {
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
        let modify_doc = doc! {
             WORK_NODE_WORKER_FIELD_ID.to_string():worker_id
        };

        let result = node_manager
            .update_entity_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(AssignWorkNodeToWorkerResponse {
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
