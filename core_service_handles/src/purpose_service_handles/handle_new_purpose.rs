use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::entity_interface::EntityInterface;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use validates::{validate_description, validate_name, validate_name_map};

#[async_trait]
pub trait HandleNewPurpose {
    /// 新建管理属性
    async fn handle_new_purpose(
        &self,
        request: Request<NewPurposeRequest>,
    ) -> Result<Response<NewPurposeResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_purpose)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewPurposeRequest>,
) -> Result<Request<NewPurposeRequest>, Status> {
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
    request: Request<NewPurposeRequest>,
) -> Result<Request<NewPurposeRequest>, Status> {
    let name = &request.get_ref().name;
    let description = &request.get_ref().description;
    
    validate_name(name.as_ref()).await?;
    validate_description(description)?;

    Ok(request)
}

async fn handle_new_purpose(
    request: Request<NewPurposeRequest>,
) -> Result<Response<NewPurposeResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let name = &request.get_ref().name;
    let description = &request.get_ref().description;

    let manage_id = &LANGUAGE_CODES_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(manage_id.to_owned())
        .unwrap();
    
    let name = name.as_ref().unwrap();
    let name_doc = bson::to_document(name).unwrap();

    let query_doc = doc! {format!("{}.{}", NAME_MAP_FIELD_ID, name.language): name.name.clone()};
    if manager.entity_exists(&query_doc).await.is_some() {
        return Err(Status::already_exists(format!(
            "{}: {:?}",
            t!("已经存在"),
            name
        )));
    }

    if let Ok(mut new_entity_doc) = make_new_entity_document(manager, &account_id).await {
        new_entity_doc.insert(
            NAME_MAP_FIELD_ID.to_string(),
            name_doc,
        );
        new_entity_doc.insert(
            DESCRIPTION_FIELD_ID.to_string(),
            bson::to_document(description).unwrap(),
        );

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewPurposeResponse { result: r })),
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
            "new_purpose"
        )))
    }
}
