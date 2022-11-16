use async_trait::async_trait;
use bson::{doc, Document};
use linked_hash_map::LinkedHashMap;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::{LANGUAGES_CODES_CODE_FIELD_ID, LANGUAGES_CODES_NATIVE_FIELD_ID};
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use property_field::{FieldDataType, PropertyField};
use view;

#[async_trait]
pub trait HandleEditLanguageCode {
    /// 新建管理属性
    async fn handle_edit_language_code(
        &self,
        request: Request<EditLanguageCodeRequest>,
    ) -> Result<Response<EditLanguageCodeResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let id = &request.get_ref().id;
        let new_code = &request.get_ref().new_code;
        let new_native = &request.get_ref().new_native;

        let manage_id = &LANGUAGES_CODES_MANAGE_ID;

        if !view::can_manage_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(manage_id.to_owned())
            .await
            .unwrap();

        let query_doc = doc! {
            "_id": id
        };
        let modify_doc = doc! {
            LANGUAGES_CODES_CODE_FIELD_ID.to_string(): new_code,
            LANGUAGES_CODES_NATIVE_FIELD_ID.to_string(): new_native
        };

        let result = manager
            .update_entity_field(query_doc, modify_doc, &account_id)
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
