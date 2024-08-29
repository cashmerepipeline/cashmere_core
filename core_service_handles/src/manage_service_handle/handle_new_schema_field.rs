use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::indexmap::IndexMap;
use dependencies_sync::log::debug;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use cash_core::SchemaField as CoreSchemaField;
use managers::manager_trait::ManagerInterface;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use validates::{validate_field_id, validate_name, validate_name_map, validate_role_group};

#[async_trait]
pub trait HandleNewSchemaField {
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
    let manage_id = &request.get_ref().manage_id;
    let field = &request.get_ref().new_field;

    if field.is_none() {
        return Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("字段不能为空"),
            manage_id
        )));
    }
    let field = field.as_ref().unwrap();

    // 已经存在
    if validate_field_id(manage_id.as_str(), &field.id.to_string())
        .await
        .is_ok()
    {
        return Err(Status::already_exists(format!(
            "{}: {}-{}",
            t!("字段已经存在"),
            manage_id,
            field.id
        )));
    }

    validate_name_map(Some(&field.name_map)).await?;

    Ok(request)
}

async fn handle_new_schema_field(
    request: Request<NewSchemaFieldRequest>,
) -> Result<Response<NewSchemaFieldResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;
    let field = request.get_ref().new_field.as_ref().unwrap();

    let name_map = field.name_map.clone();
    let name_doc = bson::to_document(&name_map).unwrap();
    let name: IndexMap<String, String> = bson::from_document(name_doc).unwrap();

    let new_field: CoreSchemaField = CoreSchemaField {
        id: field.id,
        name_map: name,
        data_type: field.data_type.clone(),
        removed: false,
    };

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id.as_str()).unwrap();
    let result = manager.new_schema_field(new_field, &account_id).await;

    // let new_id = field.id.to_string();
    // let new_id = format!("{}_{}", manage_id, field.id);
    // let response = Response::new(NewSchemaFieldResponse {
    // result: "ok".to_string(),
    // });

    match result {
        Ok(_r) => {
            if cfg!(debug_assertions) {
                debug!("{}: {}-{:?}", t!("新增字段完成"), manage_id, field);
            };
            
            // FIXME: 这里会崩溃
            Ok(Response::new(NewSchemaFieldResponse {
                // result: field.id.to_string(),
                result: "ok".to_string(),
            }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
