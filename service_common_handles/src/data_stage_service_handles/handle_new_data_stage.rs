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

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleNewDataStage {
    async fn handle_new_data_stage(
        &self,
        request: Request<NewDataStageRequest>,
    ) -> UnaryResponseResult<NewDataStageResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let data_id = &request.get_ref().data_id;
        let stage_name = &request.get_ref().stage_name;
        let description = &request.get_ref().description;

        if !view::can_manage_write(&account_id, &role_group, &DATAS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let data_manager = majordomo_arc
            .get_manager_by_id(DATAS_MANAGE_ID)
            .await
            .unwrap();

        if !view::can_field_write(&account_id, &role_group, &DATAS_MANAGE_ID.to_string(), &DATAS_STAGES_FIELD_ID.to_string()).await
        {
            return Err(Status::permission_denied("用户不具有属性可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(DATAS_MANAGE_ID)
            .await
            .unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():data_id,
        };

        let mut modify_doc = bson::Document::new();
        let new_stage = DataStageInfo{
            name: stage_name.to_owned(),
            versions: vec![],
            current_version: "".to_string()
        };
        modify_doc.insert(DATAS_STAGES_FIELD_ID.to_string(), bson::to_document(&new_stage).unwrap());

        let result = manager
            .push_entity_array_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(NewDataStageResponse {
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
