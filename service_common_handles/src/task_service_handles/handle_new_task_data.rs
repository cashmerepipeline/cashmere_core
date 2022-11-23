use async_trait::async_trait;
use bson::doc;
use managers::utils::make_new_entity_document;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use view;

#[async_trait]
pub trait HandleNewTaskData {
    async fn handle_new_task_data(
        &self,
        request: Request<NewTaskDataRequest>,
    ) -> Result<Response<NewTaskDataResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let task_id = &request.get_ref().task_id;
        let data_name = &request.get_ref().data_name;

        if !view::can_collection_write(&account_id, &role_group, &DATAS_MANAGE_ID.to_string()).await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let data_manager = majordomo_arc
            .get_manager_by_id(DATAS_MANAGE_ID)
            .await
            .unwrap();

        let local_name = match data_name {
            Some(n) => n,
            None => {
                return Err(Status::aborted(format!(
                    "没有指定名称--{}",
                    DATAS_MANAGE_ID
                )));
            }
        };
        let name_doc = doc! {local_name.language.clone():local_name.name.clone()};

        let mut new_doc = make_new_entity_document(&data_manager).await.unwrap();
        new_doc.insert(
            DATAS_MANAGE_ID_FIELD_ID.to_string(),
            TASKS_MANAGE_ID.clone(),
        );
        new_doc.insert(DATAS_ENTITY_ID_FIELD_ID.to_string(), task_id.clone());
        new_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);

        let result = data_manager
            .sink_entity(&mut new_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(r) => {
                let new_id = new_doc
                    .get_str(ID_FIELD_ID.to_string())
                    .unwrap()
                    .to_string();
                Ok(Response::new(NewTaskDataResponse { result: new_id }))
            }
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
