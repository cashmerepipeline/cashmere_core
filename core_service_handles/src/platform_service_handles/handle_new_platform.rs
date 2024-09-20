use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::PLATFORMS_WEBSITE_FIELD_ID;
use manage_define::general_field_ids::{DESCRIPTION_FIELD_ID, ID_FIELD_ID, NAME_MAP_FIELD_ID};
use manage_define::manage_ids::*;
use managers::entity_interface::EntityInterface;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use validates::{validate_description, validate_name, validate_name_map};

#[async_trait]
pub trait HandleNewPlatform {
    /// 新建管理属性
    async fn handle_new_platform(
        &self,
        request: Request<NewPlatformRequest>,
    ) -> Result<Response<NewPlatformResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_platform)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewPlatformRequest>,
) -> Result<Request<NewPlatformRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = LANGUAGE_CODES_MANAGE_ID;
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
    request: Request<NewPlatformRequest>,
) -> Result<Request<NewPlatformRequest>, Status> {
    let name = &request.get_ref().name;
    let platform = &request.get_ref().platform;
    let description = &request.get_ref().description;
    
    validate_name(name.as_ref()).await?;
    validate_description(description)?;

    Ok(request)
}

async fn handle_new_platform(
    request: Request<NewPlatformRequest>,
) -> Result<Response<NewPlatformResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let name = &request.get_ref().name;
    let platform = &request.get_ref().platform;
    let description = &request.get_ref().description;
    let website = &request.get_ref().website;

    let manage_id = &LANGUAGE_CODES_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(manage_id.to_owned())
        .unwrap();

    let query_doc = doc! {ID_FIELD_ID.to_string(): platform.clone()};
    if manager.entity_exists(&query_doc).await.is_some() {
        return Err(Status::already_exists(format!(
            "{}: {}",
            t!("平台已经存在"),
            platform
        )));
    }

    if let Ok(mut new_entity_doc) = make_new_entity_document(manager, &account_id).await {
        new_entity_doc.insert(ID_FIELD_ID.to_string(), platform);
        new_entity_doc.insert(
            NAME_MAP_FIELD_ID.to_string(),
            bson::to_document(name).unwrap(),
        );
        new_entity_doc.insert(
            DESCRIPTION_FIELD_ID.to_string(),
            bson::to_document(description).unwrap(),
        );
        new_entity_doc.insert(PLATFORMS_WEBSITE_FIELD_ID.to_string(), website);

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewPlatformResponse { result: r })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    } else {
        Err(Status::aborted(format!(
            "{}: {}",
            t!("获取新实体失败"),
            "new_platform"
        )))
    }
}
