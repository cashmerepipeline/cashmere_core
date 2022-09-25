use async_trait::async_trait;
use bson::doc;

use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use managers::utils::make_new_entity_document;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::{LANGUAGES_CODES_CODE_FIELD_ID, LANGUAGES_CODES_NATIVE_FIELD_ID, VIEW_RULES_MANAGE_FIELD_ID};

use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;

use view;
use view::ReadRule;

#[async_trait]
pub trait HandleRemoveGroupFromManageReadrule {
    /// 新建管理属性
    async fn handle_remove_group_from_manage_readrule(
        &self,
        request: Request<RemoveGroupFromManageReadRuleRequest>,
    ) -> Result<Response<RemoveGroupFromManageReadRuleResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let group_id = &request.get_ref().group_id;

        let majordomo_arc = get_majordomo().await;

        // 检查管理是否存在
        if majordomo_arc.get_manager_ids().await.contains(manage_id) {
            return Err(Status::data_loss(format!("管理不存在: {}", manage_id)));
        }

        // 检查组是否存在
        let group_manager = majordomo_arc.get_manager_by_id(GROUPS_MANAGE_ID).await.unwrap();
        let group_query_doc = doc! {ID_FIELD_ID.to_string():group_id.clone()};
        if !group_manager.entity_exists(group_query_doc).await {
            return Err(Status::data_loss(format!("组不存在: {}", manage_id)));
        }

        if !view::can_manage_write(&account_id, &groups, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let account_group_id =
            match view::get_first_write_group(&groups, &manage_id.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let view_rules_manager = majordomo_arc
            .get_manager_by_id(VIEW_RULES_MANAGE_ID.to_owned())
            .await
            .unwrap();

        let query_doc = doc! {
            VIEW_RULES_MANAGE_FIELD_ID.to_string():manage_id
        };

        // 将组权限设置为不可见
        let modify_doc = doc! {
            format!("{}.{}.read_rule", VIEW_RULES_MANAGE_FIELD_ID, group_id):ReadRule::InVisible.to_string()
        };

        let result = view_rules_manager.update_entity_map_field(query_doc, modify_doc, &account_id).await;

        match result {
            Ok(r) => Ok(Response::new(RemoveGroupFromManageReadRuleResponse {
                result: r.details(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
