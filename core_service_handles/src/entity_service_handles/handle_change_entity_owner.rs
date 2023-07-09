use dependencies_sync::bson::{doc, Document};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::prost::bytes::Buf;
use dependencies_sync::tonic::{Request, Response, Status};
use dependencies_sync::rust_i18n::{self, t};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::general_field_ids::*;

use managers::traits::ManagerTrait;
use request_utils::request_account_context;



use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleChangeEntityOwner {
    /// 编辑修改实体属性
    async fn handle_change_entity_owner(
        &self,
        request: Request<ChangeEntityOwnerRequest>,
    ) -> UnaryResponseResult<ChangeEntityOwnerResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_change_entity_owner)
            .await
    }
}

async fn validate_view_rules(
    request: Request<ChangeEntityOwnerRequest>,
) -> Result<Request<ChangeEntityOwnerRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
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
    request: Request<ChangeEntityOwnerRequest>,
) -> Result<Request<ChangeEntityOwnerRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let old_owner_id = &request.get_ref().old_owner_id;
    let new_owner_id = &request.get_ref().new_owner_id;
    
    let majordomo_arc = get_majordomo();
    
    if !majordomo_arc.get_manager_ids().contains(manage_id) {
        return Err(Status::aborted(t!("管理不存在")));
    }

    if entity_id.is_empty() {
        return Err(Status::aborted(t!("实体编号不能为空")));
    }
    if old_owner_id.is_empty() {
        return Err(Status::aborted(t!("旧拥有者编号不能为空")));
    }
    if new_owner_id.is_empty() {
        return Err(Status::aborted(t!("新拥有者编号不能为空")));
    }
    
    Ok(request)
}

async fn handle_change_entity_owner(
    request: Request<ChangeEntityOwnerRequest>,
) -> UnaryResponseResult<ChangeEntityOwnerResponse> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let old_owner_id = &request.get_ref().old_owner_id;
    let new_owner_id = &request.get_ref().new_owner_id;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

    let query_doc = doc! {
        ID_FIELD_ID.to_string():entity_id,
    };
    
    let mut modify_doc = doc! {
        OWNER_FIELD_ID.to_string():new_owner_id.clone(),
    };

    let result = manager
        .update_entity_field(query_doc, &mut modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(ChangeEntityOwnerResponse {
            result: new_owner_id.to_owned(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}

