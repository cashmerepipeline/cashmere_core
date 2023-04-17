use async_trait::async_trait;
use bson::doc;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use tonic::{Request, Response, Status};
use view;

#[async_trait]
pub trait HandleAssociateDataToTask {
    async fn handle_associate_data_to_task(
        &self,
        request: Request<AssociateDataToTaskRequest>,
    ) -> Result<Response<AssociateDataToTaskResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let task_id = &request.get_ref().task_id;
        let data_id = &request.get_ref().data_id;

        if !view::can_entity_write(
            &account_id,
            &groups,
            &TASKS_MANAGE_ID.to_string(),
            &TASK_DATA_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let group_id =
            match view::get_first_write_group(&groups, &TASKS_MANAGE_ID.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let task_manager = majordomo_arc
            .get_manager_by_id(TASKS_MANAGE_ID)
            .await
            .unwrap();

        let result = task_manager
            .update_entity_field(
                task_id,
                &TASK_DATA_FIELD_ID.to_string(),
                bson::to_bson(data_id).unwrap(),
                &account_id,
            )
            .await;

        match result {
            Ok(_r) => Ok(Response::new(AssociateDataToTaskResponse {
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

