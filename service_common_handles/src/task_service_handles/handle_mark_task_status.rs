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
pub trait HandleMarkTaskStatus {
    async fn handle_mark_task_status(
        &self,
        request: Request<MarkTaskStatusRequest>,
    ) -> Result<Response<MarkTaskStatusResponse>, Status> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let task_id = &request.get_ref().task_id;
        let status_set_id = &request.get_ref().status_set_id;
        let status_index = &request.get_ref().status_index;

        if !view::can_entity_write(&account_id, &role_group, &TASKS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let task_manager = majordomo_arc
            .get_manager_by_id(TASKS_MANAGE_ID)
            .await
            .unwrap();

        let new_value = bson::to_bson(&doc! {status_set_id:status_index}).unwrap();
        let query_doc = doc! {
            ID_FIELD_ID.to_string(): task_id.clone(),
        };
        let mut modify_doc = doc! {
            TASKS_STATUS_FIELD_ID.to_string(): new_value,
        };

        let result = task_manager
            .update_entity_field(query_doc, &mut modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(MarkTaskStatusResponse {
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
