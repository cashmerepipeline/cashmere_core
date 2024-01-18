use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::manager_trait::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};

use service_utils::types::UnaryResponseResult;
use validates::validate_name;

#[async_trait]
pub trait HandleNewCategory {
    async fn handle_new_category(
        &self,
        request: Request<NewCategoryRequest>,
    ) -> UnaryResponseResult<NewCategoryResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_category)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewCategoryRequest>,
) -> Result<Request<NewCategoryRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = TAGS_MANAGE_ID;

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
    request: Request<NewCategoryRequest>,
) -> Result<Request<NewCategoryRequest>, Status> {
    let name = &request.get_ref().name;
    let manage_id = &request.get_ref().manage_id;

    validate_name(name)?; 

    // 目标管理不能为空
    if manage_id.is_empty() {
        return Err(Status::invalid_argument(format!(
            "{}-{}",
            t!("目标管理不能为0"),
            "new_category"
        )));
    }

    Ok(request)
}

async fn handle_new_category(
    request: Request<NewCategoryRequest>,
) -> UnaryResponseResult<NewCategoryResponse> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;
    let manager_id = CATEGORIES_MANAGE_ID;

    let name = &request.get_ref().name;
    let manage_id = &request.get_ref().manage_id;
    let description = &request.get_ref().description;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manager_id).unwrap();

    let name = name.to_owned().unwrap();
    let name_doc = doc! {name.language.clone():name.name.clone()};

    // 是否存在，存在则返回
    if manager
        .entity_exists(&doc! {
            CATEGORIES_MANAGE_ID_FIELD_ID.to_string(): manage_id,
            NAME_MAP_FIELD_ID.to_string(): {"$elementMatch":name_doc.clone()},
        })
        .await.is_some()
    {
        return Err(Status::aborted(format!(
            "{}-{}-{}",
            t!("标签已经存在"),
            name.language,
            name.name,
        )));
    }

    if let Ok(mut new_entity_doc) = make_new_entity_document(&manager, &account_id).await {
        new_entity_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);
        new_entity_doc.insert(CATEGORIES_MANAGE_ID_FIELD_ID.to_string(), manage_id);
        new_entity_doc.insert(DESCRIPTION_FIELD_ID.to_string(), description.clone());

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        let new_id = new_entity_doc.get_str(ID_FIELD_ID.to_string()).unwrap();

        match result {
            Ok(_r) => Ok(Response::new(NewCategoryResponse {
                result: _r,
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    } else {
        Err(Status::aborted(format!("{}: {}", t!("获取新实体失败"), "new_category")))
    }
}
