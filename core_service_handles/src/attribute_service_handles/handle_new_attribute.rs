use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;

use service_utils::types::UnaryResponseResult;
use validates::{validate_manage_id, validate_name};

use managers::entity_interface::EntityInterface;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;

#[async_trait]
pub trait HandleNewAttribute {
    /// 新建产品
    async fn handle_new_attribute(
        &self,
        request: Request<NewAttributeRequest>,
    ) -> UnaryResponseResult<NewAttributeResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_attribute)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewAttributeRequest>,
) -> Result<Request<NewAttributeRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = ATTRIBUTES_MANAGE_ID;
        let (account_id, groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<NewAttributeRequest>,
) -> Result<Request<NewAttributeRequest>, Status> {
    // 检查目标实体存在
    let name = &request.get_ref().name;

    validate_name(name.as_ref()).await?;

    Ok(request)
}

async fn handle_new_attribute(
    request: Request<NewAttributeRequest>,
) -> Result<Response<NewAttributeResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let name = &request.get_ref().name;
    let categories = &request.get_ref().categories;
    let default_value = &request.get_ref().default_value;
    let data_type = &request.get_ref().data_type;
    let description = &request.get_ref().description;

    let local_name = name.as_ref().unwrap();
    let name_doc = doc! {local_name.language.clone():local_name.name.clone()};

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(ATTRIBUTES_MANAGE_ID)
        .unwrap();

    // 新建条目
    let mut new_entity_doc = if let Ok(r) = make_new_entity_document(manager, &account_id).await {
        r
    } else {
        return Err(Status::aborted(format!(
            "{}: 管理 {}",
            t!("新建实体文档失败"),
            ATTRIBUTES_MANAGE_ID
        )));
    };
    let new_id = new_entity_doc
        .get_str(ID_FIELD_ID.to_string())
        .unwrap()
        .to_owned();

    new_entity_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);
    new_entity_doc.insert(
        DESCRIPTION_FIELD_ID.to_string(),
        bson::to_document(description).unwrap(),
    );
    new_entity_doc.insert(CATEGORIES_FIELD_ID.to_string(), categories);
    new_entity_doc.insert(ATTRIBUTES_DATA_TYPE_FIELD_ID.to_string(), data_type.clone());
    new_entity_doc.insert(
        ATTRIBUTES_DEFAULT_VALUE_FIELD_ID.to_string(),
        bson::to_bson(default_value).unwrap(),
    );

    let new_specs_result = manager
        .sink_entity(&mut new_entity_doc, &account_id, &role_group)
        .await;

    match new_specs_result {
        Ok(_r) => Ok(Response::new(NewAttributeResponse { result: new_id })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
