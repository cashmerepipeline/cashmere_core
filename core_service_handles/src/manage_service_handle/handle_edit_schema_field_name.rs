use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{doc};
use dependencies_sync::futures::TryFutureExt;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};


#[async_trait]
pub trait HandleEditSchemaFieldName {
    ///编辑管理属性字段名
    async fn handle_edit_schema_field_name(
        &self,
        request: Request<EditSchemaFieldNameRequest>,
    ) -> Result<Response<EditSchemaFieldNameResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_edit_schema_field_name)
            .await
    }
}


async fn validate_view_rules(
    request: Request<EditSchemaFieldNameRequest>,
) -> Result<Request<EditSchemaFieldNameRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if let Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<EditSchemaFieldNameRequest>,
) -> Result<Request<EditSchemaFieldNameRequest>, Status> {
    Ok(request)
}

async fn handle_edit_schema_field_name(
    request: Request<EditSchemaFieldNameRequest>,
) -> Result<Response<EditSchemaFieldNameResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let manage_id = &request.get_ref().manage_id;
    let field_id = request.get_ref().field_id;
    let language = &request.get_ref().language;
    let new_name = &request.get_ref().new_name;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();
    let result = manager
        .edit_schema_field_name(field_id, language, new_name, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(EditSchemaFieldNameResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}