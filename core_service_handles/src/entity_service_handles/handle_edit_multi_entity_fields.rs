use dependencies_sync::bson;
use dependencies_sync::bson::{doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::prost::bytes::Buf;
use dependencies_sync::rust_i18n::{self};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::general_field_ids::*;

use managers::manager_trait::ManagerTrait;
use managers::Manager;
use request_utils::request_account_context;

use crate::entity_service_handles::validate_new_value_doc;
use service_utils::types::UnaryResponseResult;

use super::{validate_edit_entity_id, validate_edit_field_id};

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
    request: Request<EditMultiEntityFieldsRequest>,
) -> Result<Request<EditMultiEntityFieldsRequest>, Status> {
    let edits = &request.get_ref().edits;

    for edit in edits {
        let manage_id = &edit.manage_id;
        let entity_id = &edit.entity_id;
        let field_id = &edit.field_id;
        // bson bytes {field_id:new_value}
        let new_value = &edit.edit;

        if let Err(err) = validate_edit_entity_id(manage_id, entity_id).await {
            return Err(err);
        }

        let fields = match validate_edit_field_id(manage_id, entity_id, field_id).await {
            Ok(fields) => fields,
            Err(err) => return Err(err),
        };

        if let Err(err) = validate_new_value_doc(new_value, field_id, fields) {
            return Err(err);
        }
    }

    Ok(request)
}

async fn handle_edit_multi_entity_fields(
    request: Request<EditMultiEntityFieldsRequest>,
) -> UnaryResponseResult<EditMultiEntityFieldsResponse> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

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
