use dependencies_sync::bson::doc;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::futures::TryFutureExt;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::{LANGUAGES_CODES_CODE_FIELD_ID, LANGUAGES_CODES_NATIVE_FIELD_ID};
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};


#[async_trait]
pub trait HandleUpdateLanguageCode {
    /// 新建管理属性
    async fn handle_update_language_code(
        &self,
        request: Request<UpdateLanguageCodeRequest>,
    ) -> Result<Response<UpdateLanguageCodeResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_update_language_code)
            .await
    }
}

async fn validate_view_rules(
    request: Request<UpdateLanguageCodeRequest>,
) -> Result<Request<UpdateLanguageCodeRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = LANGUAGES_CODES_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<UpdateLanguageCodeRequest>,
) -> Result<Request<UpdateLanguageCodeRequest>, Status> {
    Ok(request)
}

async fn handle_update_language_code(
    request: Request<UpdateLanguageCodeRequest>,
) -> Result<Response<UpdateLanguageCodeResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let id = &request.get_ref().id;
    let new_code = &request.get_ref().new_code;
    let new_native = &request.get_ref().new_native;

    let manage_id = &LANGUAGES_CODES_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(manage_id.to_owned())
        .unwrap();

    let query_doc = doc! {
        "_id": id
    };
    let mut modify_doc = doc! {
        LANGUAGES_CODES_CODE_FIELD_ID.to_string(): new_code,
        LANGUAGES_CODES_NATIVE_FIELD_ID.to_string(): new_native
    };

    let result = manager
        .update_entity_field(query_doc, &mut modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(UpdateLanguageCodeResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
