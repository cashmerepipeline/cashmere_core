use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;
use view;

#[async_trait]
pub trait HandleNewWorkNode {
    async fn handle_new_work_node(
        &self,
        request: Request<NewWorkNodeRequest>,
    ) -> Result<Response<NewWorkNodeResponse>, Status> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let _phase_id = &request.get_ref().phase_id;
        let name = &request.get_ref().name;

        if !view::can_collection_write(&account_id, &role_group, &WORK_NODES_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(WORK_NODES_MANAGE_ID)
            .await
            .unwrap();

        let local_name = match name {
            Some(n) => n,
            None => {
                return Err(Status::aborted(format!(
                    "没有指定名称--{}",
                    PROCEDURES_MANAGE_ID
                )));
            }
        };
        let name_doc = doc! {local_name.language.clone():local_name.name.clone()};

        if let Some(mut new_doc) = make_new_entity_document(&manager).await {
            new_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);

            let result = manager
                .sink_entity(&mut new_doc, &account_id, &role_group)
                .await;

            match result {
                Ok(_r) => Ok(Response::new(NewWorkNodeResponse {
                    result: "ok".to_string(),
                })),
                Err(e) => Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                ))),
            }
        } else {
            Err(Status::aborted("创建新工作失败"))
        }
    }
}
