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
pub trait HandleNewWorkPhase {
    async fn handle_new_work_phase(
        &self,
        request: Request<NewPhaseRequest>,
    ) -> Result<Response<NewPhaseResponse>, Status> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let procedure_id = &request.get_ref().procedure_id;
        let name = &request.get_ref().name;

        if !view::can_collection_write(&account_id, &role_group, &PHASES_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let phases_manager = majordomo_arc
            .get_manager_by_id(PHASES_MANAGE_ID)
            .await
            .unwrap();

        // TODO: 阶段名是否已经在工作中检查

        let local_name = match name {
            Some(n) => n,
            None => {
                return Err(Status::aborted(format!(
                    "没有指定名称--{}",
                    PHASES_MANAGE_ID
                )));
            }
        };
        let name_doc = doc! {local_name.language.clone():local_name.name.clone()};

        // 新过程
        if let Some(mut new_phase_doc) = make_new_entity_document(&phases_manager).await {
            new_phase_doc.insert(PHASES_PROCEDURE_ID_FIELD_ID.to_string(), procedure_id);
            new_phase_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);

            let result = phases_manager
                .sink_entity(&mut new_phase_doc, &account_id, &role_group)
                .await;

            match result {
                Ok(_r) => Ok(Response::new(NewPhaseResponse {
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
