use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::log::{info, error};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::{LANGUAGES_CODES_CODE_FIELD_ID, LANGUAGES_CODES_NATIVE_FIELD_ID
};
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use manage_define::manage_ids::{PHONE_AREA_CODES_MANAGE_ID, LANGUAGES_CODES_MANAGE_ID};
use managers::traits::ManagerTrait;
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

    let result = manager.get_query_cursor(query_doc, None, None).await;

    match result {
        Ok(mut entities_iter) => {
            let mut result_codes = vec![];
            while let Some(r) = entities_iter.next().await {
                let d = if let Ok(d) = r {
                    d
                } else {
                    error!("{}", t!("获取手机区号失败"));
                    continue;
                };

                let code = d
                    .get_str(LANGUAGES_CODES_CODE_FIELD_ID.to_string())
                    .unwrap();
                let name_map = d.get_document(NAME_MAP_FIELD_ID.to_string()).unwrap().to_owned();
                let native = d.get_str(LANGUAGES_CODES_NATIVE_FIELD_ID.to_string()).unwrap().to_owned();
                
                let language_code = LanguageCode {
                    code: code.to_string(),
                    name_map: bson::from_document(name_map).unwrap(),
                    native_name: native,
                };

                result_codes.push(language_code);
            }

            Ok(Response::new(GetLanguageCodesResponse {
                language_codes: result_codes
            }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
