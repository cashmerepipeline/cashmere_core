use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::{error, info};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::{LANGUAGES_CODES_CODE_FIELD_ID, LANGUAGES_CODES_NATIVE_FIELD_ID};
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use manage_define::manage_ids::{LANGUAGES_CODES_MANAGE_ID, PHONE_AREA_CODES_MANAGE_ID};
use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tokio_stream::{self as stream, StreamExt};
use dependencies_sync::tonic::{Request, Response, Status};
use view::{self, can_entity_read, can_field_read};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetLanguageCodes {
    /// 取得管理记录数量
    async fn handle_get_language_codes(
        &self,
        request: Request<GetLanguageCodesRequest>,
    ) -> UnaryResponseResult<GetLanguageCodesResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_language_codes)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetLanguageCodesRequest>,
) -> Result<Request<GetLanguageCodesRequest>, Status> {
    // 全部可读
    Ok(request)
}

async fn validate_request_params(
    request: Request<GetLanguageCodesRequest>,
) -> Result<Request<GetLanguageCodesRequest>, Status> {
    // 没有参数
    Ok(request)
}

async fn handle_language_codes(
    _request: Request<GetLanguageCodesRequest>,
) -> Result<Response<GetLanguageCodesResponse>, Status> {
    let manage_id = LANGUAGES_CODES_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    let query_doc = doc! {};

    let result = manager.get_entity_stream(query_doc, None, None).await;

    match result {
        Ok(mut entities_iter) => {
            let mut result_codes = vec![];
            while let Some(r) = entities_iter.next().await {
                let code = bson::to_vec(&r).unwrap();

                result_codes.push(code);
            }

            Ok(Response::new(GetLanguageCodesResponse {
                language_codes: result_codes,
            }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
