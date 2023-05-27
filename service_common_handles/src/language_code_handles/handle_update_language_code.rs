use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{doc};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::{LANGUAGES_CODES_CODE_FIELD_ID, LANGUAGES_CODES_NATIVE_FIELD_ID};
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use view;

#[async_trait]
pub trait HandleEditLanguageCode {
    /// 新建管理属性
    async fn handle_edit_language_code(
        &self,
        request: Request<EditLanguageCodeRequest>,
    ) -> Result<Response<EditLanguageCodeResponse>, Status> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

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
            Ok(_r) => Ok(Response::new(EditLanguageCodeResponse {
                result: "ok".to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
