use bson::doc;
use bson::Document;
use chrono::Utc;
use tonic::{Request, Response, Status};

use crate::protocol::*;
use crate::CashmereServer;
use cash_core::field::ids::*;
use cash_core::results::OperationResult;
use majordomo::get_majordomo;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;

type ResponseResult<T> = Result<Response<T>, Status>;

impl CashmereServer {
    // pub(crate) async fn handle_new_procedure_for_work(
    //     &self,
    //     request: Request<NewProcedureForWorkRequest>,
    // ) -> Result<Response<NewProcedureForWorkResponse>, Status> {
    //     let metadata = request.metadata();
    //     // 已检查过，不需要再检查正确性
    //     let token = auth::get_auth_token(metadata).unwrap();
    //     let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
    //
    //     let work_id = &request.get_ref().work_id;
    //     let phase_index = &request.get_ref().phase_index;
    //
    //     if !view::can_entity_write(&account_id, &groups, &PROCEDURES_MANAGE_ID.to_string()).await {
    //         return Err(Status::unauthenticated("用户不具有可写权限"));
    //     }
    //
    //     // 取得第一个可写组作为组
    //     let group_id =
    //         match view::get_first_write_group(&groups, &PROCEDURES_MANAGE_ID.to_string()).await {
    //             Some(r) => r,
    //             None => return Err(Status::unauthenticated("用户不具有可写权限")),
    //         };
    //
    //     let majordomo_arc = get_majordomo().await;
    //     let procedure_manager = majordomo_arc
    //         .get_manager_by_id(PROCEDURES_MANAGE_ID)
    //         .await
    //         .unwrap();
    //     let work_manager = majordomo_arc
    //         .get_manager_by_id(WORKS_MANAGE_ID)
    //         .await
    //         .unwrap();
    //
    //     let new_id = procedure_manager.get_new_entity_id().await.unwrap();
    //     let mut new_doc = Document::new();
    //     new_doc.insert("_id", new_id);
    //     new_doc.insert(ID_FIELD_ID.to_string(), new_id);
    //     new_doc.insert(PROCEDURE_WORK_ID_FIELD_ID.to_string(), work_id);
    //     new_doc.insert(PROCEDURE_PHASE_INDEX_FIELD_ID.to_string(), phase_index);
    //
    //     let result = procedure_manager
    //         .new_entity(&mut new_doc, &account_id.to_string(), &group_id.to_string())
    //         .await;
    //
    //     match result {
    //         Ok(r) => {
    //             if let Err(e) = work_manager
    //                 .push_entity_array_field(
    //                     work_id,
    //                     &WORK_PROCEDURE_FIELD_ID.to_string(),
    //                     bson::to_bson(&doc! {phase_index:new_id}).unwrap(),
    //                     &account_id,
    //                 )
    //                 .await
    //             {
    //                 return Err(Status::aborted(format!(
    //                     "{} {}",
    //                     e.operation(),
    //                     e.details()
    //                 )));
    //             }
    //             Ok(Response::new(NewProcedureForWorkResponse {
    //                 result: "ok".to_string(),
    //             }))
    //         }
    //         Err(e) => Err(Status::aborted(format!(
    //             "{} {}",
    //             e.operation(),
    //             e.details()
    //         ))),
    //     }
    // }




}
