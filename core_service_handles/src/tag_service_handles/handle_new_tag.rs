use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::rust_i18n::{self, t};

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
use validates::{validate_name, validate_manage_id};

#[async_trait]
pub trait HandleNewTag {
    async fn handle_new_tag(
        &self,
        request: Request<NewTagRequest>,
    ) -> UnaryResponseResult<NewTagResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_tag)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewTagRequest>,
) -> Result<Request<NewTagRequest>, Status> {
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
    request: Request<NewTagRequest>,
) -> Result<Request<NewTagRequest>, Status> {
    let name = &request.get_ref().name;
    let target_manage_id = &request.get_ref().target_manage_id;

    validate_manage_id(target_manage_id).await?; 
    validate_name(name)?;
    
    Ok(request)
}

async fn handle_new_tag(request: Request<NewTagRequest>) -> UnaryResponseResult<NewTagResponse> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;
    let manager_id = TAGS_MANAGE_ID;

    let name = &request.get_ref().name;
    let target_manage_id = &request.get_ref().target_manage_id;
    let description = &request.get_ref().description;
    

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(manager_id)
        .unwrap();

    let name = name.to_owned().unwrap();
    let name_doc = doc! {name.language.clone():name.name.clone()};

    // 是否存在，存在则返回
    if manager
        .entity_exists(&doc! {
            TAGS_TARGET_MANAGES_FIELD_ID.to_string(): target_manage_id,
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

    if let Some(mut new_entity_doc) = make_new_entity_document(&manager, &account_id).await {
        new_entity_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);
        new_entity_doc.insert(TAGS_TARGET_MANAGES_FIELD_ID.to_string(), target_manage_id);
        new_entity_doc.insert(DESCRIPTION_FIELD_ID.to_string(), description.clone());

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        let new_id = new_entity_doc.get_str(ID_FIELD_ID.to_string()).unwrap();

        match result {
            Ok(_r) => Ok(Response::new(NewTagResponse {
                result: new_id.to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    } else {
        Err(Status::aborted(format!("{}: {}", t!("获取新实体失败"), "new_tag")))
    }
}
