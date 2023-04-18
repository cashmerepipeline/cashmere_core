use async_trait::async_trait;
use bson::doc;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use tonic::{Request, Response, Status};
use view;

use crate::UnaryResponseResult;
use crate::name_utils::validate_name;

#[async_trait]
pub trait HandleNewStage {
    async fn handle_new_data_stage(
        &self,
        request: Request<NewStageRequest>,
    ) -> UnaryResponseResult<NewStageResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let specs_id = &request.get_ref().specs_id;
        let name = &request.get_ref().stage_name;
        let description = &request.get_ref().description;

        if validate_name(name).is_err() {
            return Err(Status::data_loss(format!("{}", t!("名字不能为空"))));
        }
        let name = name.as_ref().unwrap();

        if !view::can_manage_write(&account_id, &role_group, &STAGES_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(STAGES_MANAGE_ID)
            .await
            .unwrap();

        // 新建条目
        let mut new_entity_doc = if let Some(r) = make_new_entity_document(&manager).await{
            r
        } else{
            return Err(Status::aborted(format!("{}: 管理 {}", t!("新建实体文档失败"), SPECSES_MANAGE_ID)));
        };

        new_entity_doc.insert(
            STAGES_SPECS_ID_FIELD_ID.to_string(),
            specs_id.clone(),
        );
        new_entity_doc.insert(DESCRIPTIONS_FIELD_ID.to_string(), description.clone());

        let new_id = new_entity_doc.get_str(ID_FIELD_ID.to_string()).unwrap().to_owned();

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group).await;

        match result {
            Ok(_r) => Ok(Response::new(NewStageResponse {
                result: new_id,
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
