use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::ID_FIELD_ID;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;

use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use view;
use view::WriteRule;

#[async_trait]
pub trait HandleChangeFieldWriteRule {
    /// 新建管理属性
    async fn handle_change_field_write_rule(
        &self,
        request: Request<ChangeFieldWriteRuleRequest>,
    ) -> Result<Response<ChangeFieldWriteRuleResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_change_field_write_rule)
            .await
    }
}

async fn validate_view_rules(
    request: Request<ChangeFieldWriteRuleRequest>,
) -> Result<Request<ChangeFieldWriteRuleRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = VIEW_RULES_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<ChangeFieldWriteRuleRequest>,
) -> Result<Request<ChangeFieldWriteRuleRequest>, Status> {
    Ok(request)
}

async fn handle_change_field_write_rule(
    request: Request<ChangeFieldWriteRuleRequest>,
) -> Result<Response<ChangeFieldWriteRuleResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let manage_id = &request.get_ref().manage_id;
    let group_id = &request.get_ref().group_id;
    let field_id = &request.get_ref().field_id;
    let write_rule = &request.get_ref().write_rule;

    let majordomo_arc = get_majordomo();

    // 检查管理是否存在
    if !majordomo_arc.get_manager_ids().contains(manage_id) {
        return Err(Status::data_loss(format!("管理不存在: {}", manage_id)));
    }

    // 检查组是否存在
    let group_manager = majordomo_arc.get_manager_by_id(GROUPS_MANAGE_ID).unwrap();
    let group_query_doc = doc! {ID_FIELD_ID.to_string():group_id.clone()};
    if !group_manager.entity_exists(&group_query_doc).await {
        return Err(Status::data_loss(format!("组不存在: {}", manage_id)));
    }

    //  检查输入规则
    if WriteRule::Unknown == WriteRule::from(write_rule.to_owned()) {
        return Err(Status::data_loss("输入读取规则错误"));
    }

    let view_rules_manager = majordomo_arc
        .get_manager_by_id(VIEW_RULES_MANAGE_ID.to_owned())
        .unwrap();

    let query_doc = doc! {
        ID_FIELD_ID.to_string():manage_id.to_string()
    };

    let modify_doc = doc! {
        format!("{}.{}.{}.write_rule",  VIEW_RULES_ENTITY_FIELD_ID, field_id, group_id): write_rule.to_owned()
    };

    let result = view_rules_manager
        .update_entity_map_field(query_doc, modify_doc, &account_id)
        .await;

    match result {
        Ok(r) => Ok(Response::new(ChangeFieldWriteRuleResponse {
            result: r.details(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
