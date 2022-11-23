use async_trait::async_trait;
use bson::doc;
use managers::utils::make_new_entity_document;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use manage_define::field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use view;

#[async_trait]
pub trait HandleNewWorkNode {
    async fn handle_new_work_node(
        &self,
        request: Request<NewWorkNodeRequest>,
    ) -> Result<Response<NewWorkNodeResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let phase_id = &request.get_ref().phase_id;
        let name = &request.get_ref().name;

        if !view::can_collection_write(
            &account_id,
            &role_group,
            &WORK_NODES_MANAGE_ID.to_string(),
        )
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
                return Err(Status::aborted(format!("没有指定名称--{}", PROCEDURES_MANAGE_ID)));
            }
        };
        let name_doc = doc! {local_name.language.clone():local_name.name.clone()};

        if let Some(mut new_doc) = make_new_entity_document(&manager).await {
            new_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);

            let result = manager
                .sink_entity(&mut new_doc, &account_id, &role_group)
                .await;

            match result {
                Ok(r) => {
                    Ok(Response::new(NewWorkNodeResponse {
                        result: "ok".to_string(),
                    }))
                }
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
