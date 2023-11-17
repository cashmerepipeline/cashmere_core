use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use dependencies_sync::linked_hash_map::LinkedHashMap;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use cash_core::SchemaField as CoreSchemaField;
use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use validates::{validate_role_group, validate_field_id};

#[async_trait]
pub trait HandleNewSchemaField{
    /// 新建管理属性
    async fn handle_new_schema_field(
        &self,
        request: Request<NewSchemaFieldRequest>,
    ) -> Result<Response<NewSchemaFieldResponse>, Status> {
        validate_role_group::<NewSchemaFieldRequest>(request)
            .and_then(validate_view_rules)
            .and_then(validate_request_params)
            .and_then(handle_new_schema_field)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewSchemaFieldRequest>,
) -> Result<Request<NewSchemaFieldRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) = view::validates::validate_manage_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<NewSchemaFieldRequest>,
) -> Result<Request<NewSchemaFieldRequest>, Status> {
    let manage_id = request.get_ref().manage_id;
    let field: &SchemaField = request.get_ref().new_field.as_ref().unwrap();
    
    // 已经存在
    if validate_field_id(&manage_id, &field.id.to_string()).await.is_ok(){
        return Err(Status::already_exists(
            format!("{}: {}-{}", t!("字段已经存在"), manage_id, field.id)
        ));
    }

    Ok(request)
}

async fn handle_new_schema_field(
    request: Request<NewSchemaFieldRequest>,
) -> Result<Response<NewSchemaFieldResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let manage_id = request.get_ref().manage_id;
    let field: &SchemaField = request.get_ref().new_field.as_ref().unwrap();

    let name_map = field.name_map.clone();
    let name_doc = bson::to_document(&name_map).unwrap();
    let name: LinkedHashMap<String, String> = bson::from_document(name_doc).unwrap();

    let new_field: CoreSchemaField = CoreSchemaField {
        id: field.id,
        name_map: name,
        data_type: field.data_type.clone(),
        removed: false,
    };

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();
    let result = manager.new_schema_field(new_field, &account_id).await;

    match result {
        Ok(_r) => Ok(Response::new(NewSchemaFieldResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
