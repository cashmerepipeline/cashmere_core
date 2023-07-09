use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::log::{info, error};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::{COUNTRY_CODES_CODE_FIELD_ID, COUNTRY_CODES_LANGUAGES_FIELD_ID, COUNTRY_CODES_NATIVE_FIELD_ID, COUNTRY_CODES_PHONE_AREA_CODE_FIELD_ID
};
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use manage_define::manage_ids::{PHONE_AREA_CODES_MANAGE_ID, COUNTRY_CODES_MANAGE_ID};
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tokio_stream::{self as stream, StreamExt};
use dependencies_sync::tonic::{Request, Response, Status};
use view::{self, can_entity_read, can_field_read};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetCountryCodes {
    /// 取得管理记录数量
    async fn handle_get_country_codes(
        &self,
        request: Request<GetCountryCodesRequest>,
    ) -> UnaryResponseResult<GetCountryCodesResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_country_codes)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetCountryCodesRequest>,
) -> Result<Request<GetCountryCodesRequest>, Status> {
    // 全部可读
    Ok(request)
}

async fn validate_request_params(
    request: Request<GetCountryCodesRequest>,
) -> Result<Request<GetCountryCodesRequest>, Status> {
    // 没有参数
    Ok(request)
}

async fn handle_get_country_codes(
    request: Request<GetCountryCodesRequest>,
) -> Result<Response<GetCountryCodesResponse>, Status> {
    let manage_id = COUNTRY_CODES_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    let query_doc = doc! {};

    let result = manager.get_query_cursor(query_doc, None).await;

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
                    .get_str(COUNTRY_CODES_CODE_FIELD_ID.to_string())
                    .unwrap();
                let name_map = d.get_document(NAME_MAP_FIELD_ID.to_string()).unwrap().to_owned();
                let native = d.get_str(COUNTRY_CODES_NATIVE_FIELD_ID.to_string()).unwrap().to_owned();
                let phone_area_code = d.get_str(COUNTRY_CODES_PHONE_AREA_CODE_FIELD_ID.to_string()).unwrap().to_owned();
                let languages = d
                    .get_array(COUNTRY_CODES_LANGUAGES_FIELD_ID.to_string())
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect();

                let phone_code = CountryCode {
                    code: code.to_string(),
                    name_map: bson::from_document(name_map).unwrap(),
                    native: native,
                    phone_area_code: phone_area_code,
                    languages: languages,
                };

                result_codes.push(phone_code);
            }

            Ok(Response::new(GetCountryCodesResponse {
                country_codes: result_codes
            }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
