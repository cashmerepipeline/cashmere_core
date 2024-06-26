use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::futures::TryFutureExt;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::general_field_ids::*;

use managers::{entity_interface::EntityInterface};
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};


use service_utils::types::UnaryResponseResult;

use validates::{validate_field_id, validate_entity_id, validate_role_group};

#[async_trait]
pub trait HandleEditEntityMapFieldRemoveKey {
    /// 编辑区域
    async fn handle_edit_entity_map_field_remove_key(
        &self,
        request: Request<EditEntityMapFieldRemoveKeyRequest>,
    ) -> UnaryResponseResult<EditEntityMapFieldRemoveKeyResponse> {
        validate_role_group(request)
            .and_then(validate_view_rules)
            .and_then(validate_request_params)
            .and_then(handle_edit_entity_map_field_remove_key)
            .await
    }
}

async fn validate_view_rules(
    request: Request<EditEntityMapFieldRemoveKeyRequest>,
) -> Result<Request<EditEntityMapFieldRemoveKeyRequest>, Status> {
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
    request: Request<EditEntityMapFieldRemoveKeyRequest>,
) -> Result<Request<EditEntityMapFieldRemoveKeyRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let field_id = &request.get_ref().field_id;
    let _key = &request.get_ref().key;

    validate_entity_id(manage_id, entity_id).await?;
    validate_field_id(manage_id, field_id).await?;

    Ok(request)
}

async fn handle_edit_entity_map_field_remove_key(
    request: Request<EditEntityMapFieldRemoveKeyRequest>,
) -> Result<Response<EditEntityMapFieldRemoveKeyResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let field_id = &request.get_ref().field_id;
    let key = &request.get_ref().key;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id.as_str()).unwrap();

    let query_doc = doc! {
        ID_FIELD_ID.to_string():entity_id,
    };
    let mut modify_doc = Document::new();
    modify_doc.insert(format!("{}.{}", field_id, key), bson::Bson::Null);

    let result = manager
        .delete_entity_map_field_key(query_doc, modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(EditEntityMapFieldRemoveKeyResponse {
            result: key.to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
