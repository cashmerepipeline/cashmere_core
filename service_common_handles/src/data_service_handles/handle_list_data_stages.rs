use async_trait::async_trait;
use bson::Document;
use manage_define::field_ids::DATAS_STAGES_FIELD_ID;
use managers::traits::ManagerTrait;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::manage_ids::*;
use manage_define::general_field_ids::*;
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleListDataStages {
    async fn handle_list_data_stages(
        &self,
        request: Request<ListDataStagesRequest>,
    ) -> UnaryResponseResult<ListDataStagesResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let data_id = &request.get_ref().data_id;

        if !view::can_entity_read(&account_id, &role_group, &DATAS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可读权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(DATAS_MANAGE_ID)
            .await
            .unwrap();

        let mut query_doc = Document::new();
        query_doc.insert(ID_FIELD_ID.to_string(), data_id);

        let result = manager.get_entity_by_id(data_id).await;

        match result {
            Ok(e) => {
                let r = e.get_array(DATAS_STAGES_FIELD_ID.to_string()).unwrap().iter().map(|s| {
                    let stage_info: DataStageInfo = bson::from_bson(s.to_owned()).unwrap();
                    stage_info
                }).collect();
                Ok(Response::new(ListDataStagesResponse {
                    stages: r
                }))
            },
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
