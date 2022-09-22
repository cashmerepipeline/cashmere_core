use async_trait::async_trait;
use bson::doc;

use manage_define::general_field_ids::NAME_MAP_FIELD_ID;
use managers::utils::make_new_entity_document;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::{LANGUAGES_CODES_CODE_FIELD_ID, LANGUAGES_CODES_NATIVE_FIELD_ID};

use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;

use view;

#[async_trait]
pub trait HandleAddGroupIntoManageReadrule {
    /// 新建管理属性
    async fn handle_add_group_into_manage_readrule(
        &self,
        request: Request<AddGroupIntoManageReadRuleRequest>,
    ) -> Result<Response<AddGroupIntoManageReadRuleResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let group_id = &request.get_ref().group_id;
        let read_rule = &request.get_ref().read_rule;

        if !view::can_manage_write(&account_id, &groups, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let account_group_id =
            match view::get_first_write_group(&groups, &manage_id.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(VIEW_RULES_MANAGE_ID.to_owned())
            .await
            .unwrap();

        match result {
            Ok(_r) => Ok(Response::new(AddGroupIntoManageReadRuleResponse {
                result: result_id.unwrap().clone(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
