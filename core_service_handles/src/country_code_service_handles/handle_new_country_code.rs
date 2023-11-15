use dependencies_sync::bson::doc;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::futures::TryFutureExt;

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

#[async_trait]
pub trait HandleNewCountryCode {
    async fn handle_new_country_code(
        &self,
        request: Request<NewCountryCodeRequest>,
    ) -> UnaryResponseResult<NewCountryCodeResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_country)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewCountryCodeRequest>,
) -> Result<Request<NewCountryCodeRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = COUNTRIES_MANAGE_ID;
        let (account_id, groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<NewCountryCodeRequest>,
) -> Result<Request<NewCountryCodeRequest>, Status> {
    Ok(request)
}

async fn handle_new_country(
    request: Request<NewCountryCodeRequest>,
) -> UnaryResponseResult<NewCountryCodeResponse> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let name = &request.get_ref().name;
    let code = &request.get_ref().code;
    let native = &request.get_ref().native;
    let phone_area_code = &request.get_ref().phone_area_code;
    let languages = &request.get_ref().languages;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(COUNTRY_CODES_MANAGE_ID)
        .unwrap();

    let local_name = match name {
        Some(n) => n,
        None => {
            return Err(Status::aborted(format!("没有指定名称--{}", code)));
        }
    };

    let name_doc = doc! {local_name.language.clone():local_name.name.clone()};

    // 是否存在，存在则返回
    if manager
        .entity_exists(&doc! {
            COUNTRY_CODES_CODE_FIELD_ID.to_string():code.clone(),
        })
        .await.is_some()
    {
        return Err(Status::aborted("国家已经存在"));
    }

    if let Some(mut new_entity_doc) = make_new_entity_document(&manager, &account_id).await {
        new_entity_doc.insert(ID_FIELD_ID.to_string(), code);
        new_entity_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);
        new_entity_doc.insert(COUNTRY_CODES_NATIVE_FIELD_ID.to_string(), native.clone());
        new_entity_doc.insert(COUNTRY_CODES_CODE_FIELD_ID.to_string(), code);
        new_entity_doc.insert(
            COUNTRY_CODES_PHONE_AREA_CODE_FIELD_ID.to_string(),
            phone_area_code,
        );
        new_entity_doc.insert(COUNTRY_CODES_LANGUAGES_FIELD_ID.to_string(), languages);

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(NewCountryCodeResponse {
                result: code.to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    } else {
        Err(Status::aborted(format!("{}: {}", t!("获取新实体失败"), "new_country")))
    }
}
