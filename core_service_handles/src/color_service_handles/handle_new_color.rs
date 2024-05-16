use dependencies_sync::bson::{self, doc};
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
use validates::{validate_description_length, validate_name};

#[async_trait]
pub trait HandleNewColor {
    async fn handle_new_color(
        &self,
        request: Request<NewColorRequest>,
    ) -> UnaryResponseResult<NewColorResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_color)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewColorRequest>,
) -> Result<Request<NewColorRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = COUNTRIES_MANAGE_ID;
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
    request: Request<NewColorRequest>,
) -> Result<Request<NewColorRequest>, Status> {
    let name = &request.get_ref().name;
    let description = &request.get_ref().description;
    validate_name(name)?;
    validate_description_length(description)?;

    Ok(request)
}

async fn handle_new_color(
    request: Request<NewColorRequest>,
) -> UnaryResponseResult<NewColorResponse> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let name = &request.get_ref().name;
    let color = &request.get_ref().color;
    let description = &request.get_ref().description;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(COLORS_MANAGE_ID).unwrap();

    let local_name = name.as_ref().unwrap();
    let _name_doc = doc! {local_name.language.clone():local_name.name.clone()};

    // 是否存在，存在则返回
    if manager
        .entity_exists(&doc! {
            COLORS_VALUE_FIELD_ID.to_string():color,
        })
        .await
        .is_some()
    {
        return Err(Status::aborted("颜色已经存在"));
    }

    if let Ok(mut new_entity_doc) = make_new_entity_document(&manager, &account_id).await {
        new_entity_doc.insert(COLORS_VALUE_FIELD_ID.to_string(), color.clone());
        new_entity_doc.insert(DESCRIPTION_FIELD_ID.to_string(), bson::to_document(description).unwrap());

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewColorResponse {
                result: r,
            })),
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
            "new_country"
        )))
    }
}
