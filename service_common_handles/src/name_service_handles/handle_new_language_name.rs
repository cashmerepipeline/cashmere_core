use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use view;

#[async_trait]
pub trait HandleNewLanguageName {
    async fn handle_new_language_name(
        &self,
        request: Request<NewLanguageNameRequest>,
    ) -> Result<Response<NewLanguageNameResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;
        let language = &request.get_ref().language;
        let new_name = &request.get_ref().new_name;

        // 管理可写性
        if !view::can_manage_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 集合可写性检查
        if !view::can_collection_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有集合可读权限"));
        }

        // 检查属性是否可写
        if !view::can_field_write(
            &account_id,
            &role_group,
            &manage_id.to_string(),
            &NAME_MAP_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有集合可读权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(*manage_id)
            .await
            .unwrap();

        // 检查语言是否已经存在
        let entity = manager.get_entity_by_id(entity_id).await.unwrap();
        let lang_name_exists = entity
            .get_document(NAME_MAP_FIELD_ID.to_string())
            .unwrap()
            .contains_key(language);
        if lang_name_exists {
            return Err(Status::already_exists("语言已经存在"));
        };

        let query_doc = doc! {
        ID_FIELD_ID.to_string():entity_id
            };
        let modify_doc = doc! {
            format!("{}.{}", NAME_MAP_FIELD_ID, language): new_name.clone()
        };

        let result = manager
            .insert_entity_map_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(NewLanguageNameResponse {
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
