use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};

use manage_define::cashmere::*;

use managers::Manager;
use request_utils::request_account_context;

use service_utils::types::UnaryResponseResult;

use validates::{validate_entity_id, validate_field_id, validate_value_doc, get_manage_schema_fields};

#[async_trait]
pub trait HandleEditMultiEntityFields {
    /// 编辑修改实体属性
    async fn handle_edit_multi_entity_fields(
        &self,
        request: Request<EditMultiEntityFieldsRequest>,
    ) -> UnaryResponseResult<EditMultiEntityFieldsResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_edit_multi_entity_fields)
            .await
    }
}

async fn validate_view_rules(
    request: Request<EditMultiEntityFieldsRequest>,
) -> Result<Request<EditMultiEntityFieldsRequest>, Status> {
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
    request: Request<EditMultiEntityFieldsRequest>,
) -> Result<Request<EditMultiEntityFieldsRequest>, Status> {
    let edits = &request.get_ref().edits;

    for edit in edits {
        let manage_id = &edit.manage_id;
        let entity_id = &edit.entity_id;
        let field_id = &edit.field_id;
        // bson bytes {field_id:new_value}
        let new_value = &edit.edit;

        validate_entity_id(manage_id, entity_id).await?;
        validate_field_id(manage_id, field_id).await?;
        let fields = get_manage_schema_fields(manage_id).await?;
        validate_value_doc(new_value, field_id, fields)?
    }

    Ok(request)
}

async fn handle_edit_multi_entity_fields(
    request: Request<EditMultiEntityFieldsRequest>,
) -> UnaryResponseResult<EditMultiEntityFieldsResponse> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let edits = &request.get_ref().edits;

    let result = Manager::update_multi_entity_fields(edits, &account_id).await;

    match result {
        Ok(_r) => Ok(Response::new(EditMultiEntityFieldsResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
