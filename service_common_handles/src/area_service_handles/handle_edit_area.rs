use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleEditArea {
    /// 编辑区域
    async fn handle_edit_area(
        &self,
        request: Request<EditAreaRequest>,
    ) -> UnaryResponseResult<EditAreaResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let area_id = &request.get_ref().area_id;
        let new_parent_id = &request.get_ref().new_parent_id;
        let new_level = &request.get_ref().new_level;

        if !view::can_manage_write(&account_id, &groups, &AREAS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(AREAS_MANAGE_ID)
            .await
            .unwrap();

        let query_doc = doc! {
            "_id": area_id
        };
        let modify_doc = doc! {
            AREAS_PARENT_ID_FIELD_ID.to_string(): new_parent_id,
            AREAS_LEVEL_FIELD_ID.to_string():new_level
        };

        let result = manager
            .update_entity_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(EditAreaResponse {
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

