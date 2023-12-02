use dependencies_sync::bson::{doc, Document, self};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::ID_FIELD_ID;
use manage_define::manage_ids::*;
use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;
use validates::{validate_entity_id, validate_manage_id, validate_role_group};
use view::view_rules_map::get_view_rules_map;
use view::{WriteRule, get_manage_schema_view_mask, can_field_read, can_field_write};

#[async_trait]
pub trait HandleGetSchemaViewRulesMap {
    /// 新建管理属性
    async fn handle_change_manage_write_rule(
        &self,
        request: Request<GetSchemaViewRulesMapRequest>,
    ) -> Result<Response<GetSchemaViewRulesMapResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_change_manage_write_rule)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetSchemaViewRulesMapRequest>,
) -> Result<Request<GetSchemaViewRulesMapRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<GetSchemaViewRulesMapRequest>,
) -> Result<Request<GetSchemaViewRulesMapRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;
    let group_id = &request.get_ref().group_id;

    validate_manage_id(manage_id).await?;
    // 验证组有效性
    validate_entity_id(&GROUPS_MANAGE_ID, group_id).await?;

    Ok(request)
}

async fn handle_change_manage_write_rule(
    request: Request<GetSchemaViewRulesMapRequest>,
) -> UnaryResponseResult<GetSchemaViewRulesMapResponse> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;
    let group_id = &request.get_ref().group_id;

    let majordomo_arc = get_majordomo();

    // 检查管理是否存在
    if !majordomo_arc.get_manager_ids().contains(manage_id) {
        return Err(Status::data_loss(format!("管理不存在: {}", manage_id)));
    }

    let view_rules_manager = majordomo_arc
        .get_manager_by_id(VIEW_RULES_MANAGE_ID.to_owned())
        .unwrap();

    let fields = view_rules_manager.get_manage_schema().await;

    let mut rules_map: Document = doc! {};
    for f in fields.iter() {
      let read_rule = can_field_read(manage_id, &f.id.to_string(), &role_group).await;
      let write_rule = can_field_write(manage_id, &f.id.to_string(), &role_group).await;
      rules_map.insert(f.id.to_string(), doc!{
        "read": read_rule,
        "write": write_rule,});
    }

    Ok(Response::new(GetSchemaViewRulesMapResponse {
            rules_map: bson::to_vec(&rules_map).unwrap(),
        }))
}
