use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{self, doc};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::{LANGUAGES_CODES_CODE_FIELD_ID, LANGUAGES_CODES_NATIVE_FIELD_ID};
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use view;

#[async_trait]
pub trait HandleNewLanguageCode {
    /// 新建管理属性
    async fn handle_new_language_code(
        &self,
        request: Request<NewLanguageCodeRequest>,
    ) -> Result<Response<NewLanguageCodeResponse>, Status> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let name = &request.get_ref().name;
        let code = &request.get_ref().code;
        let native_name = &request.get_ref().native_name;

        let manage_id = &LANGUAGES_CODES_MANAGE_ID;

        if !view::can_manage_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated(t!("用户不具有可写权限")));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(manage_id.to_owned())
            .await
            .unwrap();

        // TODO: 检查语言编号是否存在
        let query_doc = doc! {ID_FIELD_ID.to_string(): code.clone()};
        if manager.entity_exists(&query_doc).await {
            return Err(Status::already_exists(format!(
                "{}: {}",
                t!("语言已经存在"),
                code
            )));
        }

        if let Some(mut new_entity_doc) = make_new_entity_document(&manager).await {
            new_entity_doc.insert(ID_FIELD_ID.to_string(), code);
            new_entity_doc.insert(
                NAME_MAP_FIELD_ID.to_string(),
                bson::to_document(name).unwrap(),
            );
            new_entity_doc.insert(LANGUAGES_CODES_CODE_FIELD_ID.to_string(), code);
            new_entity_doc.insert(LANGUAGES_CODES_NATIVE_FIELD_ID.to_string(), native_name);

            let result = manager
                .sink_entity(&mut new_entity_doc, &account_id, &role_group)
                .await;

            match result {
                Ok(r) => Ok(Response::new(NewLanguageCodeResponse { result: r })),
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
