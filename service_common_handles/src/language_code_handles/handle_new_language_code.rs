use async_trait::async_trait;
use bson::{doc, Document};
use linked_hash_map::LinkedHashMap;
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use managers::utils::make_new_entity_document;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::{LANGUAGES_CODES_CODE_FIELD_ID, LANGUAGES_CODES_NATIVE_FIELD_ID};

use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use property_field::{FieldDataType, PropertyField};
use view;

#[async_trait]
pub trait HandleNewLanguageCode {
    /// 新建管理属性
    async fn handle_new_language_code(
        &self,
        request: Request<NewLanguageCodeRequest>,
    ) -> Result<Response<NewLanguageCodeResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let name = &request.get_ref().name;
        let code = &request.get_ref().code;
        let native_name = &request.get_ref().native_name;

        let manage_id = &LANGUAGES_CODES_MANAGE_ID;

        if !view::can_manage_write(&account_id, &groups, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let account_group_id =
            match view::get_first_write_group(&groups, &manage_id.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(manage_id.to_owned())
            .await
            .unwrap();

        if let Some(mut new_entity_doc) = make_new_entity_document(&manager).await {
            new_entity_doc.insert(
                NAME_MAP_FIELD_ID.to_string(),
                bson::to_document(name).unwrap(),
            );
            new_entity_doc.insert(LANGUAGES_CODES_CODE_FIELD_ID.to_string(), code);
            new_entity_doc.insert(LANGUAGES_CODES_NATIVE_FIELD_ID.to_string(), native_name);

            let mut result_id = None;
            let result = manager
                .sink_entity(&mut new_entity_doc, &account_id, &account_group_id)
                .await
                .and_then(|id| {
                    result_id = Some(id.clone());
                    Ok(())
                });

            match result {
                Ok(_r) => Ok(Response::new(NewLanguageCodeResponse {
                    result: result_id.unwrap().clone(),
                })),
                Err(e) => Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                ))),
            }
        } else {
            Err(Status::aborted("新增语言编码失败。"))
        }
    }
}
