use async_trait::async_trait;
use bson::doc;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;

use tonic::{Request, Response, Status};
use view;

#[async_trait]
pub trait HandleNewTask {
    async fn handle_new_task(
        &self,
        request: Request<NewTaskRequest>,
    ) -> Result<Response<NewTaskResponse>, Status> {
        let (account_id, _groups, role_group) =
            request_account_context(&request.metadata());

        let work_node_id = &request.get_ref().work_node_id;

        if !view::can_collection_write(&account_id, &role_group, &TASKS_MANAGE_ID.to_string()).await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let task_manager = majordomo_arc
            .get_manager_by_id(TASKS_MANAGE_ID)
            .await
            .unwrap();

        let mut new_doc = make_new_entity_document(&task_manager).await.unwrap();
        new_doc.insert(TASKS_WORK_NODE_ID_FIELD_ID.to_string(), work_node_id);
        let new_id = new_doc
            .get_str(ID_FIELD_ID.to_string())
            .unwrap()
            .to_string();

        let result = task_manager
            .sink_entity(&mut new_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewTaskResponse { result: new_id })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
