use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::futures::TryFutureExt;

use dependencies_sync::linked_hash_map::LinkedHashMap;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;
use property_field::{FieldDataType, PropertyField};

use dependencies_sync::tonic::{Request, Response, Status};


#[async_trait]
pub trait HandleNewSchemaField {
    /// 新建管理属性
    async fn handle_new_schema_field(
        &self,
        request: Request<NewSchemaFieldRequest>,
    ) -> Result<Response<NewSchemaFieldResponse>, Status> {
        validate_view_rules(request)
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
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if let Err(e) = view::validates::validate_manage_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<NewSchemaFieldRequest>,
) -> Result<Request<NewSchemaFieldRequest>, Status> {
    Ok(request)
}

async fn handle_new_schema_field(
    request: Request<NewSchemaFieldRequest>,
) -> Result<Response<NewSchemaFieldResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let manage_id = request.get_ref().manage_id;
    let field: &SchemaField = request.get_ref().field.as_ref().unwrap();

    let name_bytes = field.name_map.clone();
    let name_doc = Document::from_reader(&mut name_bytes.as_slice()).unwrap();
    let name: LinkedHashMap<String, String> = bson::from_document(name_doc).unwrap();

    let new_field: PropertyField = PropertyField {
        id: field.id,
        name_map: name,
        data_type: FieldDataType::from(field.data_type.clone()),
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