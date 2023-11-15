
use dependencies_sync::bson::{doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::prost::bytes::Buf;

use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::general_field_ids::*;

use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;

use service_utils::types::UnaryResponseResult;

use validates::{validate_value_doc, validate_field_id, validate_entity_id, get_manage_schema_fields};

#[async_trait]
pub trait HandleEditEntityField {
    /// 编辑修改实体属性
    async fn handle_edit_entity_field(
        &self,
        request: Request<EditEntityFieldRequest>,
    ) -> UnaryResponseResult<EditEntityFieldResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_edit_entity_field)
            .await
    }
}

async fn validate_view_rules(
    request: Request<EditEntityFieldRequest>,
) -> Result<Request<EditEntityFieldRequest>, Status> {
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
    request: Request<EditEntityFieldRequest>,
) -> Result<Request<EditEntityFieldRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let field_id = &request.get_ref().field_id;
    // bson bytes {field_id:new_value}
    let new_value = &request.get_ref().new_value;

    validate_entity_id(manage_id, entity_id).await?;

    validate_field_id(manage_id, field_id).await?;
    let fields = get_manage_schema_fields(manage_id).await?;

    validate_value_doc(new_value, field_id, fields)?;

    Ok(request)
}

async fn handle_edit_entity_field(
    request: Request<EditEntityFieldRequest>,
) -> UnaryResponseResult<EditEntityFieldResponse> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let field_id = &request.get_ref().field_id;
    // bson bytes {field_id:new_value}
    let new_value = &request.get_ref().new_value;

    let mut modify_doc = match Document::from_reader(new_value.reader()) {
        Ok(v) => {
            let t_v = v.get(field_id);
            if t_v.is_some() {
                v
            } else {
                return Err(Status::data_loss("新值不能为空"));
            }
        }
        Err(_) => return Err(Status::data_loss("新值不能为空")),
    };

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();
    let query_doc = doc! {
        ID_FIELD_ID.to_string():entity_id,
    };

    let result = manager
        .update_entity_field(query_doc, &mut modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(EditEntityFieldResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
