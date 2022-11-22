use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use view;

#[async_trait]
pub trait HandleNewWorkPhase {
    async fn handle_new_work_phase(
        &self,
        request: Request<NewPhaseForWorkRequest>,
    ) -> Result<Response<NewPhaseForWorkResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let work_id = &request.get_ref().work_id;
        let phase_name = &request.get_ref().phase_name;

        if !view::can_entity_write(&account_id, &role_group, &WORKS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let work_manager = majordomo_arc
            .get_manager_by_id(WORKS_MANAGE_ID)
            .await
            .unwrap();

        let procedure_manager = majordomo_arc
            .get_manager_by_id(PROCEDURES_MANAGE_ID)
            .await
            .unwrap();

        // TODO: 阶段名是否已经在工作中检查

        // 新过程
        if let Some(mut new_procedure_doc) = make_new_entity_document(&procedure_manager).await {
            new_procedure_doc.insert(PROCEDURES_WORK_ID_FIELD_ID.to_string(), work_id);
            new_procedure_doc.insert(PROCEDURES_PHASES_INDEX_FIELD_ID.to_string(), phase_name);

            let result = procedure_manager
                .sink_entity(&mut new_procedure_doc, &account_id, &role_group)
                .await;

            let query_doc = doc! {
                "_id":work_id
            };
            let modify_doc = doc! {
                format!("{}.{}", WORKS_PHASE_SET_FIELD_ID.to_string(), phase_name):new_procedure_id
            };

            match result {
                Ok(r) => {
                    match work_manager
                        .insert_entity_map_field(query_doc, modify_doc, &account_id)
                        .await
                    {
                        Err(e) => {
                            return Err(Status::aborted(format!(
                                "{} {}",
                                e.operation(),
                                e.details()
                            )));
                        }
                        _ => (),
                    };

                    Ok(Response::new(NewPhaseForWorkResponse {
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
